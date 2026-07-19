## Lab (Expert): Broken brute-force protection, multiple credentials per request

**Lab Goal**: Bypass brute-force protections to access the carlos account by figuring out a way to check multiple credentials in a single request.

**Exploit Vector**: Incorrect processing of the JSON login body allows attackers to inject an array of passwords into the password field. Attempts are not based on the number of passwords tested but instead on the number of requests made. By injecting an array of passwords an attacker can test an arbitrary number of passwords in a single request, bypassing the request based brute-force protection.

**Steps**:

1. Open the login page in burp browser
2. Attempt a login using a random username and password
3. Select the new POST /login that appears with a json body
4. Send this to repeater
5. Run the list_to_json scripts against the password file
6. Set the username to carlos and the password to the generated array
7. Send the request
8. Take the returned session cookie from the 302 response and create or override the sites session cookie
9. Navigate to /my-account

**Writeup**: This attack is not straightforward as the method of achieving a multi-credential request is not covered in the academy Authentication module and is only hinted at as being possible. Upon navigating to the login page send the POST /login to repeater. Attempt a login to the carlos account. Notice this triggers an internal server error. Try logging in from the webpage. Notice a second POST /login appears, this time with a json body. Send this to repeater. Do a quick google search and discover that some backends will silently accept arrays instead of strings and try to match-if-any. Take the password list provided by portswigger academy and run it through the list_to_json program to construct a properly formatted json array. Paste this into the password field and send the request. A 302 status is returned with the users session cookie. We do not know the password since they were all tested at once so we cannot login normally. Take the session cookie and create or override the sites session cookie in your browser. Navigate to /my-account to check that you're able to access carlos' account page.
