# CMSC 23500: SQL Assignment

This assignment has two parts. The first includes you writing SQL to work on your ability to write SQL to answer specific tasks.
The second part, is writing some limited SQL to interact with a simple front-end command-line application. This is to 
give you some experience with how to interact with a database from a program.

You can find more details on part 1 in [sql.md](sql.md) and on part 2 in [python.md](python.md)

## Introduction

For this assignment you will be writing SQL, Python, and will be using a PostgreSQL server, which provides a standards-compliant SQL implementation.  


We are using version 10.6 of Postgres, so the documentation at https://www.postgresql.org/docs/10.6/static/ may be useful as well.

One approach to interacting with the database is via a postgresql client (`psql`) that supports version 10 or later. On debian/ubuntu machines this is as simple as installing the PSQL client via:
`sudo apt install postgresql-client-common postgresql-client-14 libpq-dev` Note the psql client is also installed on the linux.cs machines
so you can ssh into a linux machine and use the client from there. Other clients exist for querying a database.

For this homework we will be using the database server `addison.cs.uchicago.edu` which is publicly accessible from anywhere.

## Dataset
We will use a public version of a boardgames dataset for this homework (scraped from a website Boardgamegeek.com).

Once connected, there are two kinds of commands useful to a database user. The first kind are the psql client meta-commands
and the second are SQL statements (which end in a ;).
The most important command is \?, which gives you help on meta-commands. There are two others you will find very useful.  

First, you can list the tables in the database with \dt

```
boardgames=> \dt
           List of relations
 Schema |    Name    | Type  |  Owner   
--------+------------+-------+----------
 public | categories | table | postgres
 public | desgame    | table | postgres
 public | designers  | table | postgres
 public | gamecat    | table | postgres
 public | games      | table | postgres
(5 rows)


```

Second, you can view the schema  of a given table with \d *table_name*

```
boardgames=> \d games
                         Table "public.games"
   Column    |          Type          | Collation | Nullable | Default
-------------+------------------------+-----------+----------+---------
 g_id        | integer                |           | not null |
 name        | character varying(100) |           | not null |
 avgscore    | numeric(7,6)           |           |          |
 numvotes    | numeric(6,1)           |           |          |
 minplayers  | smallint               |           |          |
 maxplayers  | smallint               |           |          |
 minplaytime | smallint               |           |          |
 maxplaytime | smallint               |           |          |
Indexes:
    "games_pkey" PRIMARY KEY, btree (g_id)
Referenced by:
    TABLE "desgame" CONSTRAINT "desgame_g_id_fkey" FOREIGN KEY (g_id) REFERENCES games(g_id)
    TABLE "gamecat" CONSTRAINT "gamecat_g_id_fkey" FOREIGN KEY (g_id) REFERENCES games(g_id)
```

The second class of useful commands are SQL commands. All SQL queries in PostgreSQL must be terminated with a semi-colon.
For example, to get a list of 10 records in the `games` table, you would type:

`SELECT * FROM games LIMIT 10;`

This query  requests a max 10 rows from the table. Using **LIMIT**  in this manner one can explore the data small bits at a time. If you really wanted to produce all the records, though, the query is:

`SELECT * FROM games;`

You can use Ctrl+C to end a query that is taking too long (it is very possible to write such bad queries even unintentionally).
Note that using the LIMIT keyword by itself offers no guarantee on which 10 rows from the result are
returned, so do not assume an ordering. \\

Finally, you can change the way psql displays the result sets to suit you better.
In particular, wrapped lines in `less` can make the output of wide tables hard to read. If this bothers you, you can try exiting the client
(you can use Ctrl+D) and starting it again with the LESS  env. variable set like this:

` LESS='-S' psql`

