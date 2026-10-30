# Pipeline des références

Environnement : Python 3.12, venv local `tools/references/.venv` (PyMuPDF, ebooklib, beautifulsoup4).

```
python -m venv tools/references/.venv
tools/references/.venv/Scripts/python -m pip install pymupdf ebooklib beautifulsoup4   # Windows
```

| Commande | Rôle |
|---|---|
| `python tools/references/dump_text.py <pdf> <txt>` | Texte d'un PDF avec marqueurs de page |
| `python tools/references/build_catalogue.py references/index/bibliographie_insp.txt references/catalogue.json` | Catalogue de la bibliographie INSP (les entrées incertaines portent `a_verifier`) |
| `python tools/references/refs.py index` | Extrait et indexe `references/ouvrages/` et `references/officiels/` (FTS5, avec pages), écrit `references/documents.json` |
| `python tools/references/refs.py search "requête" -n 8` | Passages pertinents avec fichier et page |
| `python tools/references/probe.py <pdf...>` | Détecte les scans (pages sans texte) |

L'index et le texte extrait sont dans `references/index/` (exclu de git). Les ouvrages ne sont
jamais envoyés à l'API Claude. Pour un EPUB, la « page » est le numéro de chapitre.
Un PDF dont plus de 30 % des pages sont vides est signalé « SCAN : OCR nécessaire ».
