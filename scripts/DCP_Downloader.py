import email
import imaplib
import json
import logging
import os
import re

import config


def load_emails():
    """Will load cached emails"""
    if os.path.isfile(config.proj_dir + "/emails.json"):
        with open(config.proj_dir + "/emails.json", "r", encoding="UTF-8") as f:
            logging.debug("Cache Loaded")
            return json.load(f)
    else:
        logging.debug("No cache returning empty dict")
        return {}


def save_emails(emails):
    """Will Save the cached emails"""
    with open(config.proj_dir + "/emails.json", "w", encoding="utf-8") as f:
        json.dump(emails, f, indent=2, sort_keys=True)
        logging.debug("Cache Saved")


def get_emails(force_refresh=False):
    """Will retrieve all emails from gmail"""
    m = imaplib.IMAP4_SSL("imap.gmail.com")
    m.login(config.user, config.password)
    logging.debug(m.noop())

    m.select('"Daily Coding Problem"')
    resp, items = m.search(None, "ALL")
    items = items[0].split()
    logging.debug(len(items))
    if force_refresh:
        emails = {}
    else:
        emails = load_emails()

    for emailid in items:
        resp, data = m.fetch(emailid, "(RFC822)")
        email_body = data[0][1]
        message = email.message_from_string(email_body.decode())
        subject = message["subject"]
        number = subject.split()[-2].strip("#")
        difficulty = subject.split()[-1].strip("[]")
        body = ""

        if number in emails:
            logging.info(f"Skipping Problem #{number}, Already Downloaded")
            continue

        logging.info(f"Downloading Problem #{number}")

        if message.is_multipart():
            for part in message.walk():
                ctype = part.get_content_type()
                cdispo = str(part.get("Content-Disposition"))

                # skip any text/plain (txt) attachments
                if ctype == "text/plain" and "attachment" not in cdispo:
                    body = part.get_payload(decode=True)  # decode
                    break
        # not multipart - i.e. plain text, no attachments, keeping fingers crossed
        else:
            body = message.get_payload(decode=True)

        body = body.decode()

        emails[number] = {"difficulty": difficulty, "body": body}

    m.close()
    m.logout()

    return emails


def load_problems():
    """Will load cached problems"""
    if os.path.isfile(config.proj_dir + "/problems.json"):
        with open(config.proj_dir + "/problems.json", "r", encoding="UTF-8") as f:
            logging.debug("Cache Loaded")
            return json.load(f)
    else:
        logging.debug("No cache returning empty dict")
        return {}


def save_problems(problems):
    """Will Save the cached emails"""
    with open(config.proj_dir + "/problems.json", "w", encoding="utf-8") as f:
        json.dump(problems, f, indent=2, sort_keys=True)
        logging.debug("Cache Saved")


def get_problems(emails, force_refresh=False):
    """Will create problems from emails"""

    if force_refresh:
        problems = {}
    else:
        problems = load_problems()

    for number in emails:
        if number in problems:
            logging.info(f"Skipping Problem #{number}, Already Parsed")
            continue
        logging.info(f"Parsing Problem #{number}")

        body = emails[number]["body"]

        group = re.match(
            "(?:(?:.*asked (?:.*?)\.)|(?:.*interview problem for today\.))(.*)(?:(?:We will be .*)|(?:^-+.*Upgrade.*))",
            body,
            flags=re.S | re.M,
        )

        body = group[1].strip().replace("\r", "")
        problems[number] = {"difficulty": emails[number]["difficulty"], "body": body}

    return problems


def line_prepender(filename, line):
    if not os.path.isfile(filename):
        with open(filename, "w") as f:
            logging.info(f"Generating {filename}")
    with open(filename, "r+") as f:
        content = f.read()
        f.seek(0, 0)
        f.write(line.rstrip("\r\n") + "\n" + content)


def gen_problem(problems, num):
    """Generate Missing Problems from Problem List"""
    # Create Filenames for all files with problems
    section = (num - 1) // 10
    path = f"src/problems/problems_{section:02}1_{section + 1:02}0"
    filename = f"{path}/problem_{num:03}.rs"
    parent_filename = f"src/problems/problems_{section:02}1_{section + 1:02}0.rs"

    # Make sure the files exist
    if not os.path.exists("src"):
        os.makedirs("src")
    if not os.path.exists(path):
        os.makedirs(path)
    if not os.path.isfile(parent_filename):
        with open(parent_filename, "w") as f:
            logging.info(f"Generating {parent_filename}")
        line_prepender(
            "src/problems.rs",
            f"mod problems_{section:02}1_{section + 1:02}0;",
        )

    logging.debug(filename)
    if os.path.isfile(filename):
        return

    # Create the file
    with open(config.proj_dir + "/templates/problem.txt", "r") as f:
        template = f.read()

    template = template.replace(
        "{difficulty}", problems[str(num)]["difficulty"].upper()
    )
    template = template.replace(
        "{problem_body}", problems[str(num)]["body"].replace("\r", "")
    )
    template = template.replace("{problem_number}", f"{num:03}")

    with open(filename, "w", encoding="UTF-8") as f:
        logging.info(f"Generating file for #{num:03}")
        f.write(template)

    # Include file in cpp file
    line_prepender(
        parent_filename,
        f"pub mod problem_{num:03};",
    )


def add_problems(cache_only=(False, False), force_refresh=False):
    if cache_only[0]:
        logging.info("Loading Emails Cache")
        problems = load_emails()
    else:
        logging.info("Loading Emails")
        problems = get_emails(force_refresh)
    save_emails(problems)

    if cache_only[1]:
        logging.info("Loading Problems Cache")
        problems = load_problems()
    else:
        logging.info("Loading Problems")
        problems = get_problems(problems, force_refresh)
    save_problems(problems)

    for num in problems:
        gen_problem(problems, int(num))


if __name__ == "__main__":
    add_problems((False, False))
