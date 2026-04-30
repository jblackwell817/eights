import requests
import re
import json

API_URL = "https://api.github.com/repos/johnwalley/bumps-results/contents/results/ad_format"
RAW_BASE = "https://raw.githubusercontent.com/johnwalley/bumps-results/main/results/ad_format"

OUTPUT_MEN = "data/results_men.json"
OUTPUT_WOMEN = "data/results_women.json"

ROMAN_MAP = {
    "I": 1,
    "II": 2,
    "III": 3,
    "IV": 4,
    "V": 5,
    "VI": 6,
    "VII": 7,
    "VIII": 8,
    "IX": 9,
    "X": 10
}

# Mapping for college renames
RENAME_MAP = {
    "Brasenose/StPeters": "BrasenoseStPeters",
    "LMH": "LadyMargaretHall",
    "Manchester": "HarrisManchester",
    "NewCollege": "New",
    "Osler-Green": "OslerGreen",
    "SEH": "StEdmundHall",
    "StAnnes/StHildas": "StAnnesStHildas",
    "Templeton": "GreenTempleton",
}


def normalize_name(name: str) -> str:
    """
    Remove punctuation/spaces and apply special renames
    """
    clean = (
        name.replace("'", "")
            .replace(".", "")
            .replace(" ", "")
    )
    return RENAME_MAP.get(clean, clean)


def parse_college_and_boat(raw: str):
    parts = raw.strip().rsplit(" ", 1)

    if parts[-1] in ROMAN_MAP:
        college = parts[0]
        boat = ROMAN_MAP[parts[1]]
    else:
        college = raw
        boat = 1

    return normalize_name(college), boat


def process_file(filename: str):
    year = int(filename[1:5])  # e2014m.txt → 2014
    url = f"{RAW_BASE}/{filename}"

    text = requests.get(url).text.splitlines()

    crews = []
    position = 0

    for line in text:
        line = line.rstrip()

        if not line or line.startswith("EIGHTS"):
            continue

        if re.match(r"\s*\d+\s+(Men's|Women's) Div", line):
            continue

        # Split into tokens
        tokens = line.split()

        # Collect trailing integers (race results)
        moves = []
        while tokens and re.fullmatch(r"-?\d+", tokens[-1]):
            moves.append(int(tokens.pop()))

        if not moves:
            continue  # not a crew line

        moves.reverse()  # restore original order

        raw_name = " ".join(tokens)
        college, boat = parse_college_and_boat(raw_name)

        total_move = sum(moves)

        crews.append({
            "college": college,
            "boat": boat,
            "start_pos": position,
            "final_pos": position - total_move
        })

        position += 1

    crews.sort(key=lambda c: c["final_pos"])

    return {
        "year": year,
        "standings": [
            {
                "college": c["college"],
                "boat": c["boat"]
            }
            for c in crews
        ]
    }


def main():
    response = requests.get(API_URL)
    response.raise_for_status()
    files = response.json()

    men_results = []
    women_results = []

    for f in files:
        name = f["name"]

        if re.fullmatch(r"e\d{4}m\.txt", name):
            print(f"Processing men: {name}")
            men_results.append(process_file(name))
        elif re.fullmatch(r"e\d{4}w\.txt", name):
            print(f"Processing women: {name}")
            women_results.append(process_file(name))

    men_results.sort(key=lambda r: r["year"])
    women_results.sort(key=lambda r: r["year"])

    with open(OUTPUT_MEN, "w") as f:
        json.dump(men_results, f, indent=2)

    with open(OUTPUT_WOMEN, "w") as f:
        json.dump(women_results, f, indent=2)

    print(f"\nWrote {OUTPUT_MEN} ({len(men_results)} years)")
    print(f"Wrote {OUTPUT_WOMEN} ({len(women_results)} years)")


if __name__ == "__main__":
    main()
