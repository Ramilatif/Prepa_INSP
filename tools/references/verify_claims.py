"""Aide à la vérification affirmation par affirmation d'un cours contre les ouvrages déposés.

Usage : python verify_claims.py claims.json [--only COURS_ID] [--width 240]

claims.json : liste d'objets {id, cours, claim, q, kw}
  - q  : termes de recherche plein texte (tous requis)
  - kw : mots-clés cherchés dans le passage retenu (le premier trouvé sert de centre à l'extrait)

Pour chaque affirmation, on affiche, pour chacun des ouvrages ciblés, le meilleur passage contenant un
mot-clé. C'est un outil de recherche, pas un juge : le verdict (concordance, écart, absence) reste une lecture
humaine consignée dans docs/VERIFICATION.md. Le texte affiché ne quitte pas la machine.
"""
import argparse
import json
import re
import sqlite3
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
ROOT = Path(__file__).resolve().parents[2]
DB = ROOT / "references" / "index" / "refs.db"
# Fragments de nom de fichier des ouvrages ciblés → étiquette courte.
BOOKS = {"Tome 1": "Rouvillois t.1", "La Ve République (Fr": "Rouvillois t.2", "Caporal": "Caporal-Greco"}


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("claims")
    ap.add_argument("--only")
    ap.add_argument("--width", type=int, default=240)
    ap.add_argument(
        "--books",
        help="fragments de nom de fichier séparés par des virgules (remplace les manuels de droit constitutionnel)",
    )
    a = ap.parse_args()
    if a.books:
        BOOKS.clear()
        BOOKS.update({frag.strip(): frag.strip() for frag in a.books.split(",")})
    claims = json.loads(Path(a.claims).read_text(encoding="utf-8"))
    con = sqlite3.connect(DB)
    docs = {}
    for frag, label in BOOKS.items():
        row = con.execute("SELECT id FROM docs WHERE file LIKE ?", (f"%{frag}%",)).fetchone()
        if row:
            docs[row[0]] = label
    for c in claims:
        if a.only and c["cours"] != a.only:
            continue
        print(f"\n### {c['id']} — {c['claim']}")
        match = " ".join(f'"{t}"' for t in re.findall(r"\w+", c["q"]))
        for doc_id, label in docs.items():
            rows = con.execute(
                "SELECT page, text FROM passages WHERE passages MATCH ? AND doc = ? ORDER BY rank LIMIT 6",
                (match, doc_id),
            ).fetchall()
            shown = False
            for page, text in rows:
                low = text.lower()
                pos = [(low.find(k.lower()), k) for k in c["kw"] if low.find(k.lower()) != -1]
                if not pos:
                    continue
                i = min(pos)[0]
                s = max(0, i - a.width // 3)
                print(f"  [{label} s.{page}] …{text[s:s + a.width]}…")
                shown = True
                break
            if not shown:
                print(f"  [{label}] (aucun passage avec le mot-clé)")


if __name__ == "__main__":
    main()
