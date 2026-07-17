#Lab 2FA Broken Logic Writeup

**Lab Goal**: Bypass 2FA to compromise Carlos' account

**Writeup**: This lab give you a user account wiener:peter. In the Burp browser you want to login to the wiener account. In the target list you should see two
requests to /login2. The first is a GET request and the second is a POST request. Send both of these to repeater. In repeater, notice that in the Cookie header
a key-value pair "verify=wiener" is present. If you send the GET request you should recieve an email with a new 2FA code. The GET to /login2 appears to be
what's responsible for generating a new 2FA code for the user. If you change "verify=wiener" to "verify=carlos" you should notice that you do not recieve an
email. Presumably this is because the cookie value is used to identify the user to generate a 2FA code for. Now check the POST request. Like the GET
request it uses the verify cookie to determine the user and you should notice a new value "mfa-code=<your_2fa_code". Send this to Turbo Intruder.

From Turbo Intruder you'll want to change the mfa-code value to a payload marker: "mfa-code=%s". Load the included script into Turbo Intruder and start the
attack. This will attempt to brute force the 2FA code that was generated for the carlos account. You'll know when you've guessed it correctly because
you'll get a 302 status code instead of the 200 status code you get for incorrect guesses. Check the 302 request. You should notice that a session cookie
has been granted for the carlos user. To access the carlos account you will want to take this cookie value and add ( or override ) the session cookie
in your browser with the cookie recieved from the 302 response. Once added, navigate to /my-account. Instead of being redirected to the /login route you'll
be given access to Carlos' account page.
