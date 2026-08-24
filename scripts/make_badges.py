#!/usr/bin/env python3
"""Generate shields.io endpoint badges with live figure counts.

Counts mirror crates/rhetorica/src/lib.rs `geometrized()`:
an entry counts when its `geometry` block is present.
"""
import glob
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, "..", "data", "figures")
OUT = os.path.join(HERE, "..", "badges")

CATALOG = 456  # inherited catalog size (see MANIFESTO.md); repo may ship fewer entries


def endpoint(label, message, color):
    return {"schemaVersion": 1, "label": label, "message": message, "color": color}


def main():
    geometrized = 0
    witness_tested = 0
    for path in sorted(glob.glob(os.path.join(DATA, "*.json"))):
        with open(path, encoding="utf-8") as fh:
            entry = json.load(fh)
        if entry.get("geometry") is not None:
            geometrized += 1
            if (entry.get("epistemic") or {}).get("status") == "WITNESS_TESTED":
                witness_tested += 1

    os.makedirs(OUT, exist_ok=True)
    targets = {
        "geometrized.json": endpoint(
            "figures geometrized", f"{geometrized} / {CATALOG}", "brightgreen"
        ),
        "witness.json": endpoint("witness-tested", str(witness_tested), "blue"),
    }
    for name, payload in targets.items():
        with open(os.path.join(OUT, name), "w", encoding="utf-8") as fh:
            json.dump(payload, fh, indent=2)
            fh.write("\n")

    print(f"geometrized {geometrized}/{CATALOG}, witness-tested {witness_tested}")


if __name__ == "__main__":
    main()
