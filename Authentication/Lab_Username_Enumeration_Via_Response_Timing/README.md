## Lab: Username enumeration via response timing

**Lab Goal**: Enumerate users and identify valid accounts through variations in response time. Brute-force a valid account and gain access

**Exploit Vector**: Improper handling of X-Forwarded-For headers allows users to bypass login attempt lockouts. The application only hashes passwords for valid usernames and allows passwords of arbitrary length. This allows attackers to inject extremely long passwords, inflating the hashing time well over what is expected from network jitter and general speed differences between attempts, allowing the attacker to identify valid users based on response time alone.

**Steps**:

1. Open the login page in burp browser
2. Send the POST /login to turbo intruder
3. Add the X-Forwarded-For header and insert a payload marker
4. Insert a payload marker in the username and password fields as such "username=%s&password=%s"
5. Load the ti_username_enumeration_timing script into turbo intruder
6. Run the attack
7. Sort requests by time
8. The request that took significantly longer than the other requests is the valid user
9. Load the ti_password_brute_force script into turbo intruder
10. Insert the discovered username into the username field as such "username=<valid_username>&password=%s"
11. Run the attack
12. Sort responses by status code
13. 302 response is the correct password
14. Insert credentials into the login page
15. ( NOTE ) If you're blocked from attempting the login, take the session cookie returned from the 302 and either create or override the sites session cookie with the extracted cookie. Navigate to /my-account and you should land on the users account page.

**Writeup**: Starting the lab the attack is simple. Enumerate the usernames and insert a randomly generated password of at least 500 characters into the password field. On running the attack it should be quickly noticed that the login error message changed from "Invalid username or password" to "You have made too many incorrect login attempts. Please try again in 30 minutes(s). The lab has a login attempt limit of three attempts before you're blocked from making further attempts. We'll utilize a trick we learned in the SQL Injection labs and insert a new header X-Forwarded-For and inject a random IPv6 address into it for every new request. Restart the attack. Notice that the "Too many login attempts" message has changed back to "Invalid username or password". Let the attack complete. Sort by response time and notice that one request has a 10x longer response time than the other. This indicates to us the username is valid as a different code path was triggered and the password was hashed. Start brute-forcing the discovered users password using the same X-Forwarded-For trick we used for username enumeration. Once the attack has finished, sort the responses by status code. The 302 response and redirect attempt to the users account page indicates the password was correct. Take the discovered username and password and enter them into the login page. This will log you into the users account, completing the lab.
