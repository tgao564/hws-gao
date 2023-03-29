## Part 1 - SQL Problems

### Notes

If you are trying to add a predicate for a character attribute, you must wrap the value in a single quote (e.g name = 'Bob').


## Autograder
To allow you quickly debug your SQL statements we will provide an autograder on gradescope that will let you enter your answer and compare this against our expected right answer. **If your schema is not what
we expect then it will not compare results.** If all your answers are identical, you will see no output. Otherwise, if your answer is identical it will let you know. If you answer is partially right the comments give some hints and in some cases it will give you a sample record you missed or a sample record in your result that we are not expecting. The autograder is not perfect so if you are confident in your result and it does not think so let us know!

The output messages are not the best (blame Aaron). For example on prob 1 in the homework if you give `select name from games where avgscore > 6` you will get the following output, which is trying to tell you that your answer is different than ours as your records are wrong and it thinks you 96 more records than the answer does.

```
Autograder thinks:
SQL Result Comparison [TooManyRecords quantity=96]
SQL Result Comparison [RecordsWrong]
Your SQL: select name from games where avgscore > 6
```

To use the autograder, simply zip your answers/ directory and upload the zip file in a submission on gradescope. You will see both the autograder output with a score and a brief explanation about the error for each question.

### Submission

You have to submit 1 file for each item in a problem:  your query in SQL
(the file extension should be .sql).

For example, let’s assume that there is a problem 0 and item a, which
asks you to find the distinct names of 10 games. You have to create the following file named 0a.sql:

Contents:

`select distinct name from games limit 10;`

All the .sql  files for your answers are already creates, so you just need to insert your answers.
*No output is needed*

## Problems
For the below problems, you will connect to Postgres to use a preloaded database using:

`psql -d boardgames -U dbstudent -h addison.cs.uchicago.edu`

when it asks you for the password use *dbrocks33*  
Note that you can add a password to avoid having to type in the password every time https://www.postgresql.org/docs/10.6/static/libpq-pgpass.html

psql should be installed on the csil machines(linux.cs.uchicago.edu) so you can use it from there.

Alternatively you can use OmniDB (or another SQL Gui) to connect. 

Database Name=boardgames
host = addison.cs.uchicago.edu
Database User = dbstudent

Schemas for output as provided in parentheses. *Note that all SQL must run as a single SQL statement (eg no temp tables).*
However, note that CTEs (i.e., WITHs) are acceptable.

### 1
What is the name of the game with an avgscore of 6.226650 (given as name)

### 2
What are the names of games that either have (a) an avgscore >7.3 and < 7.6 or (b) have maxplayers = 9 (given as name)

### 3
Without using any join, subquery, CTE, or view (e.g. a single table query), what is the count and c_id of the single category that has
the most number of games in it? If there are more than one category that has the same number of games, give the smallest c_id. (given as as count, c_id)

### 4
What is the mean (eg average) avgscore of games that have the same maxplayers (hint group by), sorted by maxplayers ascending (given as avg, maxplayers)?

### 5
For the above query, only show results where the mean avgscore (eg avg) is above 6, still sorted by maxplayers ascending (given as avg, maxplayers)

### 6
 Find games whose name contains the word *edition* with either uppercase or lowercase 'e'. (given as name)

### 7
What are the names and avgscore of the 10 highest rated games (avgscore) by avgscore in descending order (given as name, avgscore)

### 8
What are the names of games or designers that have 'arc' (lowercase) anywhere in the name. Note the attribute `designer` is the designer name. For full credit remove any preceeding whitespace at the start of the designer or game name (given as name).

### 9
Find the 10 longest game titles (by characters)  that allow for between 3 and 5 players inclusive (e.g. games that need at minimum 3 people to play (minplayers) and support a maximum of 5 players (maxplayers)) . (given as name, namelen)

Hint: See https://www.postgresql.org/docs/10/functions-string.html 

### 10
Find the category id and count of games in the category, for categories that have at least 15 games in them. (given as c_id, cnt)

### 11
Find the category name of the most 5 popular categories, where popularity is defined as the highest average(e.g. mean) of avgscore for all  games in the category. Order the categories by average (e.g. mean) of avgscore descending, followed by category name ascending. Avgscore is an attribute of games that indicates what the game’s score is (where the values contributing to this come from users and are not included in the dataset). (given as category, avg)

### 12
Extend the query for problem 4 to only show categories that have at least 15 games in them. (given as category, avg). Maintain the same sort order.

### 13
For each category show the designer that has the longest possible game as max (maxplaytime) -- considering only the games that have designers associated with them (e.g. you can omit games that have no designer).

Subsequently, only consider categories that have at least one designer associated with its respective games (e.g. 'if no designer exists for any game in a category, then omit the category' or alternatively 'if there does not exist any game in this category with a designer, then ignore the category').
In case of a tie for the longest game and/or if a game has multiple designers, show all designers. Do not show the same designer multiple times per category. 

The final output relation should given as (designer,category,max)  and sorted by category descending, designer ascending

Two categories  from my expected answer to pay attention to or hopefully help guide you 

```
  Matt Leacock        | Word Game                 |   0
```

```
  Francesco Nepitello | Novel-based               | 240
  Marco Maggi         | Novel-based               | 240
  Roberto Di Meglio   | Novel-based               | 240
```