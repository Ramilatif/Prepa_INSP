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
kw = os.environ.get("PEEK_KW")  # si défini : extraits autour de ce mot-clé plutôt que le début de la page
for p in pages:
    row = con.execute("SELECT text FROM passages WHERE doc=? AND page=?", (doc[0], p)).fetchone()
    n = int(os.environ.get("PEEK_LEN", "350"))  # longueur de l'extrait affiché
    if kw and row:
        text, low, shown, start = row[0], row[0].lower(), 0, 0
        max_hits = int(os.environ.get("PEEK_HITS", "2"))
        while shown < max_hits and (i := low.find(kw.lower(), start)) != -1:
            a = max(0, i - n // 2)
            print(f"--- p.{p} @{i}: …{text[a:a + n]}…")
            shown, start = shown + 1, i + n // 2
        if shown == 0:
            print(f"--- p.{p}: mot-clé absent")
    else:
        print(f"--- p.{p}: {row[0][:n] if row else '(vide)'}")
