## Lab (Expert): 2FA bypass using a brute-force attack 

**Lab Goal** : Utilize rhe username and password provided by the lab to login to Carlos' account and brute-force his MFA code to gain access to his my-account page.

**Exploit Description**: Lack of login attempt limits and MFA code cycling allows an attacker to brute-force a users MFA code by repeatedly logging in to the account and cycling through all possible codes. 

**Steps**: 

1. Compile the provided Rust program
2. Copy the host URL
3. Run the attack as such ./mfa-force <host_url>
4. Wait until a session cookie is displayed
5. Create or modify the session cookie in your browser then navigate to /my-account 

**Writeup**: This is one of the labs where it's better not to use intruder and instead either use Turbo Intruder or write your own script. In our case we wrote our own script. First you'll want to navigate to the /login page and inspect the GET /login request. Notice that the login form contains an invisible field with a pre-filled value called "crsf". If you reload the page and inspect the response again, you should notice that the crsf value has changed. Send POST /login to repeater. You should notice that the url encoded body includes the crsf token the GET form included. Try removing this and submitting carlos' credentials. Inspecting the response, you should notice that you simply get a " CSRF token missing" error message. Repeat this but this time leave GET and POST request. This time leave the crsf token in the form and submit Carlos' credentials. This time you'll be granted a temporary session cookie and naviagated to /login2. Inspect the GET /login2 response. The form for the MFA code also has a crsf token. Try submitting a random code. You get an invalid code message. Also, notice that the crfs token never changed. Try again. This time you should be sent back to the login page. This should  should provide enough information to start setting up your attack. 

What we'll want to do for our attack is establish a session by: 

1. Send a GET request to /login
2. Extract the crsf token from the form
3. Create our POST body as such "crsf={get_token}&username={username}&password={password}
4. Extract our temporary session cookie from the 302 response
5. Send a GET request with our temporary cookie to /login2 and extract the crsf token from the form  


Once we have the cookie for our session as well as the crsf token we'll want to start iterating the codes by injecting the crsf token and code guess as such "crsf={extracted_token}&mfa-code={guess}". We have two guesses before our session is invalidated so after the second guess you will need to run the login sequence again. We keep doing this repeatedly until we recieve a 302 redirect to Carlos' account page. We can take the final session token and override our existing one then navigate to /my-account to access his account page. 

**Code Description**: The program consists of two main functions: 
* get_session
* guess_mfa 

get_session takes a reqwest client, the host URL, crsf extraction regex, username, password and request retry count. The function is responsible for running the entire login workflow and returning the crsf token we'll need to perform our MFA guess. The crsf token is extracted through a simple regex capture group. 

guess_mfa takes a reqwest client, pre-constructed /login2 URL, our current guess and request retry count. It returns an AttemptOutcome enum. This function is responsible for running our guess and either returning the response on success as an AttemptOutcome::Hit(response) or one of AttemptOutcome::Wrong or AttemptOutcome::Dead. To do this we check the status code of each request and if the response code is 302 it's a correct guess. If the response code isn't 302 then it was a wrong guess or we've been redirected back to the /login page. 

We use clap to parse command-line arguments so we can dynamically get the host, username, password, attempt count and request retry value. This allows the program to continue working across lab attempts. 

The program currently makes its guesses sequentially and we terminate after guessing all numbers from 0 to 9999, regardless of whether or not we obtained the session cookie.
