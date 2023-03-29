## Part 2 - Interacting with a DBMS from Python

In this part, we will be accessing the `boardgames` database that you queries in Part I, programatically from within a Python program. We will be using the `psycopg2` library to interact with the database. 

## Setup

You will need to use python3 and have the dependencies listed in `2-python-app/requirements.txt` installed. We recommend you use a virtual environment for installing these, if you are using your own machine. The following commands should work on a linux-based system:

```bash
# create a virtualenv for managing python virtual environments

cd 2-python-app
python3 -m venv venv # create a virtual environment named venv

# activate the virtual environment
source venv/bin/activate

# install the dependencies
pip install -r requirements.txt
```


## Completing the Problem
You will need to complete the `TODO` tasks in `db.py` and `app.py` to complete this part of the assignment. You will need to setup the database connection to the `boardgames` database using the same parameters as Part I, and also need to implement all of the functions. The application is wired up to a command-line program in `app.py`. You can run the application by running the following command:

```bash
DB_USER='<username>' DB_PASSWORD='<password>' python app.py
```

The application will display a command-line interface that allows you to interact with the database. Look at the homework announcement for the username and password to access the database. 

### References
- [Python Tutorial](https://docs.python.org/3/tutorial/) - Chapters 3,4 and 9 are particularly relevant
- [psycopg2 documentation](https://www.psycopg.org/docs/)

### Design Considerations

In order to get a full score on this assignment, you will you will also need to ensure that you follow good design/security principles which are not explicity covered in our tests. Here are a few things to consider:

- The initial connection to the database is expensive. You should not be creating a new connection for every query. Instead, you should create a connection once and reuse it for all queries during the lifetime of the application.
- You should strive to retrieve the results using as few queries as possible. Your queries should not retreive more rows or columns than is necessary
    - For example, you should not retreive all the rows of the table and then filter them in python. 
- The username and password for the database should not be hard-coded in the application. Instead, you should use the environment variables `DB_USER` and `DB_PASSWORD` to retreive the username and password. This will allow you to easily change the username and password without having to change the code.
- You should close the connection to the database when you are done with it. You can do this by calling the `close()` method on the connection object.
- You should not be using the `*` operator in your SQL queries. You should be explicit about the columns you are selecting.
- You should be using parameterized queries to prevent SQL injection attacks. 



### Testing
The homework uses the `pytest` framework to run a series of tests against the functions you've implemented to verify that you are returning the correct results from the database. You can test your code by running the following command:

```bash
cd 2-python-app
DB_USER='<username>' DB_PASSWORD='<password>' pytest
```
