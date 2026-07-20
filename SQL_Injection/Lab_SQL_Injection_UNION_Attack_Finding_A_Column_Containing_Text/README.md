## Lab: SQL injection UNION attack, finding a column containing text

**Lab Goal**: Exploit the category filter sql injection vulnerability to discover a column containing text and output the random value.

**Exploit Description**: The server constructs its WHERE clause using string concatenation of the category url parameter. This allows an attacker to return new rows in the query by inserting a UNION statement into the original query which can be used to identify column data types and return arbitrary data.

**Steps**:

1. Send "GET /filter?category=<some_category>" to repeater
2. Discover the number of columns by modifying the category filter to inject an ORDER BY statement as such: "GET /filter?category=' ORDER BY 1 -- -"
3. Increment the ORDER BY statement until you trigger an error
4. Create your UNION statement as such: "GET /filter?category=' UNION SELECT Null,Null,Null -- -" where the number of nulls matches the largest order by number that doesn't trigger an error
5. Change one of the Null columns to 'a'. Check if an error is triggered. Repeat until a column that doesn't trigger an error is found
6. For the column that didn't trigger an error, copy and paste the random string into the column that accepted the 'a' string

**Writeup**: To complete this lab we'll start by getting the number of columns we need to use. To do this we'll replace the category selection with our ORDER BY injection as such: "GET /filter?category=' ORDER BY 1 -- -". This doesn't triggers an error so we'll continue incrementing the number until an error occurs. Once we have the number of columns we'll craft our UNION statement as such: "GET /filter?category=' UNION SELECT Null,Null,Null -- -". To discover which column takes a string we'll start replacing the Null values with a single character string as such: "GET /filter?category=' UNION SELECT 'a',Null,Null -- -". This may or may not trigger an error. If an error is triggered then we'll change the 'a' back into a Null and move to the next column. If an error isn't triggered that means the column takes a string. Once we find the column that takes a string, we'll replace the 'a' string with the random string provided, completing the lab.

Note: As with other labs, if a SELECT in the form "SELECT Null..." fails and you're certain you have the right number of columns, that likely means you're accessing an Oracle database. To fix this add a FROM statement that queries the dual table as such: "SELECT Null... FROM dual".
