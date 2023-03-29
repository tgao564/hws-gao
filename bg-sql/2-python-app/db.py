import psycopg2
import psycopg2.extras

from typing import List

class Boardgame:
    def __init__(self, g_id: int, name: str, avgscore: float, numvotes: int, 
                minplayers: int, maxplayers: int, 
                minplaytime: int, maxplaytime: int, designer=None):
        '''
        Initialize Boardgame object.
        id, average score, number of votes, min/max players, min/max playtime are ints or floats.
        name is a string.
        designer is an optional string
        '''
        self.id = g_id
        self.name = name
        self.avgscore = avgscore
        self.numvotes = numvotes
        self.minplayers = minplayers
        self.maxplayers = maxplayers
        self.minplaytime = minplaytime
        self.maxplaytime = maxplaytime

        self.designer = designer

    def __repr__(self):
        return f"Boardgame({self.name} ({self.id}))"

    def __hash__(self):
        return hash(self.id)


class Designer:
    def __init__(self, d_id: int, name: str, country: str):
        '''
        Initialize a Designer object.
        id, name, and country are strings.
        '''
        self.id = d_id
        self.name = name
        self.country = country

    def __repr__(self):
        return f"Designer({self.name} ({self.id}))"

    def __hash__(self):
        return hash(self.id)


class DBManager:
    def __init__(self, host, database, user, password):
        """ Setup a database connection here """
        # TODO: Set up DB connection here
       
    def get_all_games(self):
        """ Returns a list of all Boardgame objects. """
        games = []
        #TODO: Complete this function

        return games


    def get_games_by_name(self, name):
        """ Returns a list of Boardgame objects that match the given name. The parameter `name` can be matched anywhere in boardgame's name, case insensitive. """
        games = []
        #TODO: Complete this function

        return games


    def get_all_designers(self):
        """ Return a list of all Designer objects """
        designers = []
        # TODO: Complete this function

        return designers

    def get_games_by_designer(self, designer):
        """ Return a list of Boardgame objects that match the given desginer name. The parameter `designer` can be matched anywhere in designer's name, case insensitive. """
        games = []
        # TODO: Complete this function
        
        return games
