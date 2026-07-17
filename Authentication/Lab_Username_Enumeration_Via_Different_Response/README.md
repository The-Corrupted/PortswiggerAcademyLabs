# Lab: Username enumeration via different responses

**Lab Goal**: Use differences in the username and password error messages to identify a valid user and brute-force their password.

**Writeup**: Navigate to the login page in burp browser. In Target you should notice a POST /login endpoint that contains a username=test&password= field. Send
this to Turbo Intruder. From Turbo Intruder, modify the username and password field to insert a payload into username and a default password in the password
field like so "username=%s&password=test". Load ti_username_enum.py into Turbo Intruder and modify the file path to point to the username list provided for these
labs. Once you've done that, run the attack. After the attack is finished you should notice that at least one of the requests has a different Content-Length than
the others. Inspecting the request, you should notice that the "Invalid username" message has changed to an "Invalid password" message, indicating this
is a valid user. Copy the username and load the ti_password_enum.py script. Like the ti_username_enum script, you'll want to modify the file path to point
to the password list provided for these labs. Modify the username and password fields to inject the valid user and a payload in the password field as such
"username=<valid_username>&password=%s. Run the attack. Just like the username, you'll want to let the attack run until a request returns a different
Content-Length from the others. Click on the request. You should see that the password triggered a 302 redirect to the users my-account page.

Take the discovered username and password and enter it into the login page in burp browser. You should now be logged in.
