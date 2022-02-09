import json
import os
import logging
import getpass

proj_dir = os.path.dirname(__file__)

# Define the Logging config
logging.basicConfig(
    format="%(asctime)s - %(levelname)s - %(message)s", level=logging.INFO
)

logging.debug("Start Of Config.py")


def save_config():
    j = {"user": user, "password": password}

    with open(proj_dir + "/config.json", "w") as f:
        json.dump(j, f, indent=2)


if not os.path.isfile(proj_dir + "/config.json"):
    user = input("Email: ")
    password = getpass.getpass()
    save_config()

with open(proj_dir + "/config.json") as f:
    j = json.load(f)

user = j["user"]
password = j["password"]
