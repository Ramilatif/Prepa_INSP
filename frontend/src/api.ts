// Client de l'API. Le jeton est conservé dans localStorage (connexion par pseudo, sans mot de passe).

const KEY = "insp_token";

export const getToken = (): string | null => {
  try {
    return localStorage.getItem(KEY);
  } catch {
    return null;
  }
};
export const setToken = (t: string | null) => {
  try {
    if (t) localStorage.setItem(KEY, t);
    else localStorage.removeItem(KEY);
  } catch {
    /* stockage indisponible : la connexion ne survivra pas au rechargement */
  }
};

export async function api<T>(path: string, init: RequestInit = {}): Promise<T> {
  const token = getToken();
  const res = await fetch(`/api${path}`, {
    ...init,
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
  });
  if (!res.ok) throw new Error((await res.text()) || res.statusText);
  return res.json() as Promise<T>;
}

export interface FicheSummary {
  slug: string;
  titre: string;
  matiere: string;
  maj: string;
  statut: string;
  a_verifier: boolean;
}
export interface Card {
  id: string;
  recto: string;
  verso: string;
  kind: string;
  source_ref: string;
}
export interface Annale {
  annee: number | null;
  type: string;
  epreuve: string;
  epreuve_code: string | null;
  format: "reforme" | "ancien";
  url_source: string;
}
export interface Grille {
  nom: string;
  total: number;
  note_sur_les_poids: string;
  source: { document: string; url: string; pages: string };
  criteres: { id: string; libelle: string; categorie: string; poids: number; poids_officiel: boolean; attendus: string[] }[];
  erreurs_frequentes: string[];
  conseils: string[];
}
export interface Me {
  pseudo: string;
  majeure: string | null;
  mineure: string | null;
  session: string | null;
}
