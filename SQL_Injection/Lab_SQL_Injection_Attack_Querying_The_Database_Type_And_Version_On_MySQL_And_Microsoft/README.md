## Lab: SQL injection attack querying the database type and version on MySQL and Microsoft

**Lab Goal**: Use an SQL injection vulnerability in the product category filter to display the database type and version to the page.

**Exploit Vector**: The category filter uses string concatenation to construct its query and sends the result to the client as-is. This allows the attacker to end the category string early and inject a union into the query and exfiltrate database information they otherwise would be unable to access.

**Steps**:

1. Select a product category from burp browser
2. Select the "GET /filter?category=<some_category> url and send it to repeater
3. Remove the category string and insert your union as such: category=' UNION SELECT @@version,Null -- -
4. Send the request and inspect the response to verify the database information is displayed

**Writeup**: This lab follows the exact same steps as the Oracle lab but with slightly different semantics. Navigate to the home page in burp browser and select a category. Send the GET /filter?category=<some_category> request to repeater. Remove the category and test the injection site by adding "'" as such "GET /filter?category='". This will trigger an internal server error which verifies this a valid injection site. In the Oracle lab we guessed the columns returned by the original select by adding Null to our union until our SELECT statement stopped triggering an internal server error. We're going to use a shortcut this time. Discover the rows by injecting an ORDER BY statement into the category filter as such "GET /filter?category=' ORDER BY 2 -- -". You should notice this doesn't trigger an error so you'll want to increment the count to three. You should notice at three that this triggers an internal server error which confirms that the original SELECT is returning exactly two columns. Contruct the UNION statement as such "GET /filter?category=' UNION SELECT @@version,Null -- -". If you inspect the page you should see that the database information is rendered to the page.

Note: @@version is used by both MSSQL and MySQL which is why the lab combines them.
