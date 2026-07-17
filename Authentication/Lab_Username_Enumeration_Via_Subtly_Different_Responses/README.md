#Lab: Username enumeration via subtly different responses

**Lab Goal**: Use small differences in the page to identify valid users and then brute-force their password.

**Writeup**: Navigate to the login page in burp browser. Target should should show a POST request to /login. Send this to repeater. From repeater modify the
username and password field to any random user or password. Send this to turbo intruder. Change the username and password fields to inject a payload into
username and a default password into password as such "username=%s&password=test". Load the ti_subtle_difference_username_enum script into turbo intruder. You'll notice that in the handleResponse function that we're now excluding any response that contains the standard error message from getting added to the table. The reason
we're doing this is because the lab has been designed to randomize the Content-Length of each response by changing the length of the session cookie on each new request.
This means we can no longer rely on Content-Length alone like we could in other labs. "Invalid username or password" is the warning for a known bad username and is
the most likely to change so we start by extracting it and using it as a filter. If nothing is found then we can try a more thorough diff of the page contents. 
If you run the attack you'll notice that exactly one response is added to the table. Check the response. In the body you should notice that the difference between
this response and what we're expecting is a single period at the end of the message, suggesting to use this username triggered a different code path and is likely
a real user.

Copy the username and load the ti_subtle_difference_password_enum script into turbo intruder. Change the username and password fields to inject a payload into password
and the discovered username into username as such "username=<valid_username>&password=%s". Run the attack. Once it's finished you should find one response with a
different status code from the others and a redirect to the users my-account page. Copy this password and submit the discovered username and password into the login
form. This should successfully log you into the users account.
