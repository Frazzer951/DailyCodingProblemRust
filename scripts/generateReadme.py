import logging
import os
import re
from typing import Dict
from typing import List

logging.basicConfig(
    format="%(asctime)s - %(levelname)s - %(message)s", level=logging.INFO
)


def _replace_between_tags(string: str, content: str, start: str, end: str) -> str:
    content = "\n".join([start, content, end])

    return re.sub(
        pattern=rf"{start}.*?{end}",
        repl=content,
        string=string,
        flags=re.DOTALL,
    )


def get_problems() -> Dict[str, List[str]]:
    problems = {
        "Completed": [],
        "Incomplete": [],
        "Skipped": [],
    }
    path = os.path.join(ROOT_DIR, "../src/problems")
    problems_folder = os.path.abspath(path)

    sub_folders = [x[0] for x in os.walk(problems_folder)]

    for folder in sub_folders:
        if folder == problems_folder:
            continue
        logging.debug(f"Processing {folder}")
        files = os.listdir(folder)
        for file in files:
            logging.debug(file)
            with open(os.path.join(folder, file), encoding="UTF-8") as f:
                content = f.read()

                difficulty = ""
                if "/* EASY" in content:
                    difficulty = "Easy"
                elif "/* MEDIUM" in content:
                    difficulty = "Medium"
                elif "/* HARD" in content:
                    difficulty = "Hard"
                else:
                    assert False, f"Unknown difficulty for {file}"

                if "// NOT DONE" in content:
                    problems["Incomplete"].append((file, difficulty))
                elif "// SKIPPED" in content:
                    problems["Skipped"].append((file, difficulty))
                else:
                    problems["Completed"].append((file, difficulty))

    return problems


def create_readme_links(files: List[str]) -> List[str]:
    text: List[str] = []
    files.sort()
    for file, difficulty in files:
        num = int(file.rstrip(".rs").split("_")[-1])
        section = (num - 1) // 10
        text.append(
            f" - [Problem {num:03}](src/problems/problems_{section:02}1_{section + 1:02}0/{file}) - {difficulty}"
        )

    return text


def create_completed_text(problems: List[str]) -> str:
    text = [
        f"## Completed ⭐️ - {len(problems)}",
        "<details><summary>Completed</summary>",
        "<p>",
        "",
    ]
    text += create_readme_links(problems)
    text += ["", "</p>", "</details>", ""]

    return "\n".join(text)


def create_missing_text(problems: List[str]) -> str:
    text = [
        f"## Missing ❌️ - {len(problems)}",
        "<details><summary>Missing</summary>",
        "<p>",
        "",
    ]
    text += create_readme_links(problems)
    text += ["", "</p>", "</details>", ""]

    return "\n".join(text)


def create_skipped_text(problems: List[str]) -> str:
    text = [
        f"## Skipped️ ⏭️ - {len(problems)}",
        "<details><summary>Skipped</summary>",
        "<p>",
        "",
    ]
    text += create_readme_links(problems)
    text += ["", "</p>", "</details>", ""]

    return "\n".join(text)


def generate_readme():
    path = os.path.join(ROOT_DIR, "../README.md")
    readme_file = os.path.abspath(path)

    problems = get_problems()

    with open(readme_file, encoding="UTF-8") as f:
        current_readme = f.read()

    readme = _replace_between_tags(
        current_readme,
        create_completed_text(problems["Completed"]),
        "<!-- start completed section -->",
        "<!-- end completed section -->",
    )

    readme = _replace_between_tags(
        readme,
        create_missing_text(problems["Incomplete"]),
        "<!-- start missing section -->",
        "<!-- end missing section -->",
    )

    readme = _replace_between_tags(
        readme,
        create_skipped_text(problems["Skipped"]),
        "<!-- start skipped section -->",
        "<!-- end skipped section -->",
    )

    with open(readme_file, "w", encoding="UTF-8") as f:
        f.write(readme)


if __name__ == "__main__":
    ROOT_DIR = os.path.dirname(os.path.abspath(__file__))
    generate_readme()
