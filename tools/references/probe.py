"""Sonde rapide : nombre de pages et part de pages contenant du texte, pour détecter les scans."""
import sys
import pymupdf

for path in sys.argv[1:]:
    doc = pymupdf.open(path)
    n = doc.page_count
    with_text = sum(1 for p in doc if len(p.get_text().strip()) > 50)
    print(f"{path}\n  pages={n} pages_avec_texte={with_text}")
