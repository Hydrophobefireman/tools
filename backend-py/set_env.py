from os import environ, getcwd
from os.path import join, isfile
from json import load
from dotenv import load_dotenv

def setup_env():
    load_dotenv()