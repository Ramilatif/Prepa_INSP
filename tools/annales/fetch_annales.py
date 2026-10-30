"""Télécharge les sujets, meilleures copies et rapports du jury du concours externe (voie générale)
publiés par l'INSP et écrit content/annales/index.json (métadonnées + lien source).

Source : https://insp.gouv.fr/concours/ressources-utiles/sujets-meilleures-copies-et-rapports-des-jurys/voie-generale
(liste relevée le 2026-10-30). La page précise que « seuls les rapports à compter de la session 2024
correspondent aux concours réformés » : on classe donc 2024 et après en « reforme », 2021 à 2023 en
« ancien ». Les sujets 2022-2023 (cinq épreuves distinctes dont questions sociales et finances publiques)
sont bien de l'ancien format.

Usage : python fetch_annales.py   (les PDF vont dans content/annales/files/, exclus de git)
"""
import hashlib
import json
import sys
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "content" / "annales"
BASE = "https://insp.gouv.fr/files/files/concours/ressources-utiles/sujets-meilleurs-copies/"
SCR = BASE + "sujets-copies-rapports/"

NEW = ["question_contemporaine", "economie", "droit_public", "qrc", "cas_pratique"]
EPREUVE_CODE = {
    "question_contemporaine": "note_reflexion", "economie": "op_economie", "droit_public": "op_droit_public",
    "qrc": "qrc", "cas_pratique": "cas_pratique", "questions_sociales": "questions_sociales",
    "finances_publiques": "finances_publiques",
}

docs: list[dict] = []


def add(annee, type_, epreuve, url, format_):
    docs.append({"annee": annee, "concours": "externe", "type": type_, "epreuve": epreuve,
                 "epreuve_code": EPREUVE_CODE.get(epreuve), "format": format_, "url": url})


# --- 2026 (réformé) : sujets ---
for e, name in zip(NEW, ["question_contemporaine", "economie", "droit_public", "questions_a_reponses_courtes", "cas_pratique"]):
    add(2026, "sujet", e, f"{BASE}sujets-2026/voie-generale/voie_generale_concours_externe_2026_{name}.pdf", "reforme")

# --- 2025 (réformé) : sujets puis meilleures copies ---
pre = SCR + "Sujets%202025/Voie%20g%C3%A9n%C3%A9rale/"
for e, f in zip(NEW, ["1_2025_ce_ce_talents_question_contemporaine", "2_2025_ce_ce_talents_question_economie",
                      "3_2025_ce_ce_talents_droit_public", "4_2025_ce_qrc", "5_2025_ce_ce_talents_cas_pratique"]):
    add(2025, "sujet", e, f"{pre}{f}.pdf", "reforme")
for e, f in zip(NEW, ["question_contemporaine", "economie", "droit_public", "questions_a_reponse_courte", "cas_pratique"]):
    add(2025, "meilleure_copie", e, f"{BASE}concours-externe/concours-externe-2025/ce_2025_{f}.pdf", "reforme")

# --- 2024 (réformé) ---
for e, f in zip(NEW, ["question_contemporaine_light", "economie_light_1", "droit_public_light_1",
                      "questions_a_reponses_courtes_light_1", "cas_pratique_light_1"]):
    add(2024, "sujet", e, f"{SCR}sujet_ce_2024_{f}.pdf", "reforme")
for e, f in zip(NEW, ["question_contemporaine", "note_operationnelle_en_economie", "note_operationnelle_en_droit_public",
                      "questions_a_reponses_courtes", "cas_pratique"]):
    add(2024, "meilleure_copie", e, f"{BASE}concours-externe/concours-externe-2024/ce_2024_meilleure_copie_{f}.pdf", "reforme")

# --- 2023 (ancien format) ---
OLD = ["droit_public", "economie", "question_contemporaine", "questions_sociales", "finances_publiques"]
suffix23 = {"droit_public": "_0", "economie": "_0"}
for e in OLD:
    add(2023, "sujet", e, f"{SCR}sujet-concours-externe-2023-{e.replace('_', '-')}{suffix23.get(e, '')}.pdf", "ancien")
for e, f in [("droit_public", "droit_public_0"), ("economie", "economie"), ("finances_publiques", "finances_publiques"),
             ("question_contemporaine", "question_contemporaine"), ("questions_sociales", "questions_sociales")]:
    add(2023, "meilleure_copie", e, f"{SCR}ce_2023_meilleure_copie_{f}.pdf", "ancien")

# --- 2022 (ancien format) ---
for e in OLD:
    add(2022, "sujet", e, f"{SCR}sujet-concours-externe-2022-{e.replace('_', '-')}.pdf", "ancien")
for e in OLD:
    add(2022, "meilleure_copie", e, f"{SCR}ce_meilleure_copie_{e}_2022.pdf", "ancien")

# --- Rapports des jurys ---
add(2025, "rapport_jury", "toutes", SCR + "rapport_de_la_presidente_des_jurys_de_concours_insp_2025.pdf", "reforme")
add(2024, "rapport_jury", "toutes", SCR + "rapport_du_president_des_jurys_2024.pdf", "reforme")
add(2023, "rapport_jury", "toutes", SCR + "rapport_de_mme_hayet_zeggar_presidente_des_jurys_de_concours_2023.pdf", "ancien")
add(2022, "rapport_jury", "toutes", SCR + "rapport_de_m_alain_beretz_president_des_jurys_de_concours_de_2022_0.pdf", "ancien")
add(2021, "rapport_jury", "toutes", SCR + "rapport_de_la_presidente_des_concours_d_entree_ena_2021.pdf", "ancien")

# --- Sujets fictifs V0 (format réformé, publiés par l'INSP) ---
V0 = "https://insp.gouv.fr/files/files/concours/voie-generale/concours-externe/"
add(None, "sujet_fictif", "qrc", V0 + "v0_question_a_reponses_courtes_voie_generale.pdf", "reforme")
add(None, "sujet_fictif", "cas_pratique", V0 + "v0_cas-pratique-transition-ecologique-ou-numerique_voie-generale_concours-externe_insp.pdf", "reforme")


def main() -> int:
    files = OUT / "files"
    files.mkdir(parents=True, exist_ok=True)
    failed = 0
    for d in docs:
        name = f"{d['annee'] or 'fictif'}_{d['type']}_{d['epreuve']}.pdf"
        d["fichier"] = f"files/{name}"
        dest = files / name
        if not dest.exists():
            try:
                req = urllib.request.Request(d["url"], headers={"User-Agent": "prepa-insp-fetch/1.0"})
                data = urllib.request.urlopen(req, timeout=60).read()
                if not data.startswith(b"%PDF"):
                    raise ValueError("réponse non PDF")
                dest.write_bytes(data)
            except Exception as e:  # noqa: BLE001 - on consigne l'échec et on continue
                d["erreur"] = str(e)
                failed += 1
                print(f"ÉCHEC {d['url']} : {e}", file=sys.stderr)
                continue
        d["sha256"] = hashlib.sha256(dest.read_bytes()).hexdigest()
    (OUT / "index.json").write_text(json.dumps(
        {"source": "https://insp.gouv.fr/concours/ressources-utiles/sujets-meilleures-copies-et-rapports-des-jurys/voie-generale",
         "releve_le": "2026-10-30", "documents": docs}, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"{len(docs)} documents, {failed} échec(s)")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
