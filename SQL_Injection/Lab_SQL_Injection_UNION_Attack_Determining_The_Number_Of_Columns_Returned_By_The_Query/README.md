## Lab: SQL Injection UNION attack, determining the number of columns returned by the query

**Lab Goal**: Exploit an SQL injection vulnerability in the product category filter to return an additional row with three null values.

**Exploit Description**: The server constructs its WHERE clause using string concatenation of the category url parameter. This allows an attacker to return new rows in the query by inserting a UNION statement into the original query.

**Steps**:

1. Send "GET /filter?category=<some_category>" to repeater
2. Remove the category string and insert an ORDER BY as such: "GET /filter?category=' ORDER BY 2 -- -"
3. Keep incrementing the ORDER BY number until you trigger an internal server error
4. Construct your UNION attack as such: "GET /filter?category=' UNION SELECT Null,Null,Null -- -" where the number of Null values is equivalent to the largest ORDER BY number that doesn't trigger an internal server error

**Writeup**: This is an important lab as many attacks will use UNION and in order to use UNION in your attacks you'll need to know how many columns the original SELECT returns as your SELECT will need to return the exact same number of columns otherwise the query is considered invalid. To do this we'll start by modifying the category filter to inject an ORDER BY statement as such: "GET /filter?category=' ORDER BY 2 -- -". This doesn't trigger an error so we're going to keep increasing the ORDER BY number until we do. At ORDER BY 4 an internal server error triggers which tells us we need to return three columns in our UNION statement for it to be valid. The reason the ORDER BY triggers an internal server error at four is because ORDER BY orders by column so when you do ORDER BY 3 you're asking that the data returned by the database be ordered by column three. When you exceed the number of columns returned by a select statement the database throws an error since there's no fourth column for it to order. Knowing that the SELECT statement needs to have three columns we can craft our query as such: "GET /filter?category=' UNION SELECT Null,Null,Null -- -". This should return a row containing all null values, completing the lab.

Note: If a select statement in the form of "SELECT Null..." fails, it's likely because you're querying an Oracle database which requires all SELECT statements to have an accompanying FROM statement specifying the table. You can fix this by doing "SELECT Null... FROM dual".
