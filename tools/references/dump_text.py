"""Extrait le texte d'un PDF page par page dans un fichier texte (avec marqueurs de page)."""
import sys
import pymupdf

src, dst = sys.argv[1], sys.argv[2]
doc = pymupdf.open(src)
with open(dst, "w", encoding="utf-8") as out:
    for i, page in enumerate(doc, start=1):
        out.write(f"\n=====PAGE {i}=====\n{page.get_text()}")
