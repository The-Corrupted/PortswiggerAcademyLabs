## Lab: SQL Injection attack, querying the database type and version on Oracle

**Lab Goal**: Use an SQL injection vulnerability in the product category filter to display the database type and version to the page.

**Exploit Vector**: The category filter uses string concatenation to construct its query and sends the result to the client as-is. This allows the attacker to end the category string early and inject a union into the query and exfiltrate database information they otherwise would be unable to access.

**Steps**:

1. Select a product category from the burp browser
2. Select the GET /filter?category=<some_category> url and send it to repeater
3. Remove the category string and insert your union as such: category=' UNION SELECT banner,Null FROM v$version -- -
4. Send the request and inspect the response to verify the database information is displayed

**Writeup**: To solve this lab what we want to do is craft an SQL query to return data from the v$version table to the page. To do this we start by putting the home page into burp browser. Select a category and send the URL to repeater. From repeater we want to modify the category filter to inject a UNION into the query so we can return data from v$version table. To do this we can delete the category value entirely and replace it with our string break as such "GET /filter?category=' ". This is a test to ensure that this is a valid injection. Notice that it returns an internal server error which verifies this caused the category string to close early. In order to fetch data from a seperate database we'll want to use a UNION which will combine the results of our SELECT statement with the original SELECT statement. To do this we'll modify our injection to be "GET /filter?category=' UNION SELECT banner FROM v$version -- -". Notice that this triggers an internal server error. In order for a UNION to be valid it must return the exact same number of columns as the original SELECT statement. We'll add another column to our SELECT by inserting a Null after the banner as such "GET /filter?category=' UNION SELECT banner,Null FROM v$version -- -". Now you should notice that instead of a blank page, the page now lists the database type and version.

Note: In most cases it's faster and more portable to use ORDER BY to get the number of columns the original SELECT returns. In this lab we could have done this by doing "GET /filter?category=' ORDER BY 1 -- -" and increment until we trigger an internal server error which would tell us we've exceeded the number of columns returned by the original select statement. Once we have that we can craft our SELECT to have the proper number of columns without needing to guess how many Null values we need to insert.
