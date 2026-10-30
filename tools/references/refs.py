"""Pipeline local des références : extraction (avec numéros de page), indexation plein texte, recherche.

  python refs.py index            # (ré)indexe references/ouvrages et references/officiels
  python refs.py search "requête" [-n 8]

L'index (SQLite FTS5) vit dans references/index/, exclu de git. Le texte des ouvrages ne
quitte jamais la machine. Pour un EPUB, la « page » est le numéro du chapitre (document HTML).
"""
import argparse
import hashlib
import json
import re
import sqlite3
import sys
from pathlib import Path

import pymupdf

ROOT = Path(__file__).resolve().parents[2]
REFS = ROOT / "references"
DB = REFS / "index" / "refs.db"
DIRS = {"ouvrages": REFS / "ouvrages", "officiels": REFS / "officiels"}


def pages_pdf(path: Path):
    """Produit (numéro de page, texte) ; signale les pages sans texte (scan probable)."""
    with pymupdf.open(path) as doc:
        for i, page in enumerate(doc, start=1):
            yield i, page.get_text()


def pages_epub(path: Path):
    from bs4 import BeautifulSoup
    from ebooklib import ITEM_DOCUMENT, epub

    book = epub.read_epub(str(path), options={"ignore_ncx": True})
    n = 0
    for item in book.get_items_of_type(ITEM_DOCUMENT):
        text = BeautifulSoup(item.get_content(), "html.parser").get_text(" ")
        if text.strip():
            n += 1
            yield n, text


def extract(path: Path):
    return pages_pdf(path) if path.suffix.lower() == ".pdf" else pages_epub(path)


def cmd_index() -> None:
    DB.parent.mkdir(parents=True, exist_ok=True)
    con = sqlite3.connect(DB)
    con.execute("CREATE TABLE IF NOT EXISTS docs(id INTEGER PRIMARY KEY, kind TEXT, file TEXT UNIQUE, sha TEXT, pages INTEGER, empty_pages INTEGER)")
    con.execute("CREATE VIRTUAL TABLE IF NOT EXISTS passages USING fts5(text, doc UNINDEXED, page UNINDEXED, tokenize='unicode61 remove_diacritics 2')")
    catalogue = []
    for kind, folder in DIRS.items():
        for path in sorted(folder.glob("*")) if folder.exists() else []:
            if path.suffix.lower() not in (".pdf", ".epub"):
                continue
            sha = hashlib.sha256(path.read_bytes()).hexdigest()
            row = con.execute("SELECT id, sha, pages, empty_pages FROM docs WHERE file=?", (str(path.name),)).fetchone()
            if row and row[1] == sha:
                doc_id, n, empty = row[0], row[2], row[3]
            else:
                if row:
                    con.execute("DELETE FROM passages WHERE doc=?", (row[0],))
                    con.execute("DELETE FROM docs WHERE id=?", (row[0],))
                cur = con.execute("INSERT INTO docs(kind,file,sha,pages,empty_pages) VALUES (?,?,?,0,0)", (kind, path.name, sha))
                doc_id, n, empty = cur.lastrowid, 0, 0
                for num, text in extract(path):
                    n += 1
                    if len(text.strip()) < 50:
                        empty += 1
                        continue
                    con.execute("INSERT INTO passages(text,doc,page) VALUES (?,?,?)", (re.sub(r"\s+", " ", text), doc_id, num))
                con.execute("UPDATE docs SET pages=?, empty_pages=? WHERE id=?", (n, empty, doc_id))
                con.commit()
            scan = n > 0 and empty / n > 0.3
            print(f"{kind}/{path.name}: {n} pages, {empty} sans texte{'  -> SCAN : OCR nécessaire' if scan else ''}")
            catalogue.append({"id": f"{kind}-{doc_id}", "type": kind, "fichier": path.name, "pages": n, "scan": scan, "sha256": sha})
    con.commit()
    (REFS / "documents.json").write_text(json.dumps(catalogue, ensure_ascii=False, indent=2), encoding="utf-8")


def cmd_search(query: str, limit: int) -> None:
    con = sqlite3.connect(DB)
    # Les termes sont mis entre guillemets pour neutraliser la syntaxe FTS.
    match = " ".join(f'"{t}"' for t in re.findall(r"\w+", query))
    rows = con.execute(
        "SELECT d.file, p.page, snippet(passages, 0, '[', ']', ' … ', 24) FROM passages p JOIN docs d ON d.id = p.doc "
        "WHERE passages MATCH ? ORDER BY rank LIMIT ?",
        (match, limit),
    ).fetchall()
    for file, page, snip in rows:
        print(f"- {file}, p. {page}\n    {snip}")
    if not rows:
        print("Aucun passage trouvé.")


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    sub.add_parser("index")
    s = sub.add_parser("search")
    s.add_argument("query")
    s.add_argument("-n", type=int, default=8)
    a = ap.parse_args()
    if sys.stdout.encoding != "utf-8":
        sys.stdout.reconfigure(encoding="utf-8")
    cmd_index() if a.cmd == "index" else cmd_search(a.query, a.n)
