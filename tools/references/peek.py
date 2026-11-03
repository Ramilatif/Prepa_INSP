"""Aperçu de quelques pages indexées d'un document : python peek.py <fragment du nom> [pages...]"""
import os
import sqlite3
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
con = sqlite3.connect(Path(__file__).resolve().parents[2] / "references" / "index" / "refs.db")
doc = con.execute("SELECT id, file, pages FROM docs WHERE file LIKE ?", (f"%{sys.argv[1]}%",)).fetchone()
print(doc)
pages = [int(p) for p in sys.argv[2:]] or [1, 2, 3, 10, 30, 45]
for p in pages:
    row = con.execute("SELECT text FROM passages WHERE doc=? AND page=?", (doc[0], p)).fetchone()
    n = int(os.environ.get("PEEK_LEN", "350"))  # longueur de l'extrait affiché
    print(f"--- p.{p}: {row[0][:n] if row else '(vide)'}")
