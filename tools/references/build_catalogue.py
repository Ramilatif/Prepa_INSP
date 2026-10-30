"""Construit references/catalogue.json à partir de la bibliographie de l'INSP (texte extrait).

Usage : python build_catalogue.py references/index/bibliographie_insp.txt references/catalogue.json

Les entrées dont l'analyse est incertaine reçoivent `"a_verifier": true` plutôt que
d'être devinées. Le champ `acces` vaut « payant » pour tout ouvrage du commerce ; les
publications officielles en accès libre sont ajoutées séparément (voir fetch_officiels).
"""
import json
import re
import sys
import unicodedata
from pathlib import Path

# Matière selon la rubrique de la bibliographie (chiffres romains).
SECTIONS = {
    "I": "droit_public",
    "II": "economie",
    "III": "finances_publiques",
    "IV": "questions_sociales",
    "V": "europe_international",
    "VI": "transitions",
}
NOISE = re.compile(r"^\s*(\d+\s*|Centre de documentation.*|INSP – Centre.*|Lien vers la notice en ligne\s*)$")
ROMAN = re.compile(r"^(I|II|III|IV|V|VI)\.\s*$")
# Début de la ligne de publication : « 4e édition… », « Septième édition… » ou « Paris : Éditeur, 2024 ».
PUB_START = re.compile(
    r"^(\[?\w+\s*(e|re|ère|er|ème)?\s*\[?(édition|éd\b)|\[?[^:]{2,40}\]?\s*:\s*[^,]+,\s*(\[?\d{4}))"
)
POSITION = re.compile(r"\b(\d{4})\b[^,]*,\s*[\dXVILC\-\s]*pages")


def norm(s: str) -> str:
    s = unicodedata.normalize("NFKD", s.lower())
    return re.sub(r"[^a-z0-9 ]+", " ", "".join(c for c in s if not unicodedata.combining(c))).strip()


def parse_block(lines: list[str], matiere: str) -> dict:
    entry = {"matiere": matiere, "acces": "payant", "a_verifier": False}
    k = next((i for i in range(2, len(lines)) if PUB_START.match(lines[i])), None)
    if k is None:
        entry.update(auteurs=lines[0] if lines else "", titre=" ".join(lines[1:]), a_verifier=True)
        return entry
    pub_lines = []
    j = k
    while j < len(lines):
        pub_lines.append(lines[j])
        j += 1
        if "pages" in " ".join(pub_lines) and re.search(r"[\.\)]\s*$", pub_lines[-1]):
            break
    pub = " ".join(pub_lines)
    entry["auteurs"] = lines[0]
    entry["titre"] = " ".join(lines[1:k])
    m = re.match(r"^(\[?\d+\s*(?:e|re|ère|er)\s+édition[^.]*)\.", pub)
    entry["edition"] = m.group(1).strip() if m else ""
    rest = pub[m.end():].strip() if m else pub
    m2 = re.match(r"^(?:\[?[^:\]]+\]?)\s*:\s*([^,]+),", rest)
    entry["editeur"] = m2.group(1).strip() if m2 else ""
    m3 = POSITION.search(rest)
    entry["annee"] = int(m3.group(1)) if m3 else None
    entry["description"] = " ".join(lines[j:]).strip()
    if not (entry["editeur"] and entry["annee"]):
        entry["a_verifier"] = True
    return entry


def main(src: str, dst: str) -> None:
    text = Path(src).read_text(encoding="utf-8")
    # Les pages 1 et 2 (couverture, sommaire) ne contiennent aucune notice.
    body = text.split("=====PAGE 3=====", 1)[1]
    matiere, current, ouvrages, zoom = "droit_public", [], [], {}
    in_zoom = False
    skip_title = False
    for raw in body.splitlines():
        line = raw.strip()
        if m := ROMAN.match(line):
            matiere = SECTIONS[m.group(1)]
            in_zoom = False
            skip_title = True  # la ligne suivante est l'intitulé de la rubrique
            continue
        if skip_title:
            skip_title = False
            continue
        if line.startswith("Zoom sur"):
            in_zoom = True
            continue
        if line.startswith("=====PAGE"):
            continue
        if NOISE.match(line) or not line:
            continue
        if in_zoom:
            # Une nouvelle notice (auteur, titre...) signale la fin de la zone « Zoom ».
            if re.search(r"pour (commencer|aller plus loin)", line, re.I) or re.match(r"^\d\.\s", line):
                in_zoom = False
            else:
                zoom.setdefault(matiere, []).append(line)
                continue
        if line.startswith("Cote"):
            if current:
                ouvrages.append(parse_block(current, matiere))
            current = []
            continue
        if re.match(r"^\d\.\s*$", line) or re.search(r"pour (commencer|aller plus loin)\s*:?$", line, re.I):
            continue
        current.append(line)
    for i, o in enumerate(ouvrages, start=1):
        o["id"] = f"ouvrage-{i:03d}"
        # Le rapprochement avec les fichiers déposés se fait à la main (champ `depose_id`),
        # un titre générique comme « Droit administratif » correspondant à plusieurs ouvrages.
        o["depose_id"] = None
    out = {
        "source": "INSP, Centre de ressources et d'ingénierie documentaires, Bibliographie « Ouvrages de référence », septembre 2026",
        "ouvrages": ouvrages,
        "ressources_zoom": zoom,
    }
    Path(dst).write_text(json.dumps(out, ensure_ascii=False, indent=2), encoding="utf-8")
    flagged = sum(o["a_verifier"] for o in ouvrages)
    print(f"{len(ouvrages)} ouvrages, dont {flagged} à vérifier ; zones Zoom : {list(zoom)}")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
