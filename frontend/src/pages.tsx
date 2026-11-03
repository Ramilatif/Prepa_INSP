import { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import Markdown from "react-markdown";
import remarkGfm from "remark-gfm";
import remarkCallouts from "./remarkCallouts";
import { api, uploadPdf, type Annale, type Card, type Copie, type FicheSummary, type Grille, type Me } from "./api";
import { EPREUVES, PARCOURS, TYPE_ANNALE, formatDuration, matiereLabel, statutLevel } from "./labels";

/** Cours rédigés, classés par parcours puis par ordre de lecture. */
export function Cours() {
  const [list, setList] = useState<FicheSummary[]>([]);
  useEffect(() => {
    api<FicheSummary[]>("/fiches?type=cours").then(setList).catch(() => setList([]));
  }, []);
  const parcours = [...new Set(list.map((c) => c.parcours ?? "autres"))];
  return (
    <section>
      <h1>Cours</h1>
      <p className="muted">Chapitres rédigés à partir de sources officielles citées. Le statut de chaque cours indique son niveau de vérification.</p>
      {list.length === 0 && <p className="muted">Aucun cours pour l'instant.</p>}
      {parcours.map((p) => (
        <div key={p}>
          <h2>{PARCOURS[p] ?? p}</h2>
          <ol>
            {list.filter((c) => (c.parcours ?? "autres") === p).map((c) => (
              <li key={c.slug}>
                <Link to={`/fiches/${c.slug}`}>{c.titre.replace(/^Cours de [^:]+ : /, "")}</Link>
                <span className={`badge level${statutLevel(c.statut)}`}>{c.statut}</span>
              </li>
            ))}
          </ol>
        </div>
      ))}
    </section>
  );
}

/** Sujets complets chronométrés (format réformé), dépôt du PDF et lien vers l'auto-évaluation. */
export function Entrainement() {
  const [sujets, setSujets] = useState<Annale[]>([]);
  const [copies, setCopies] = useState<Copie[]>([]);
  const [error, setError] = useState("");
  const load = () => {
    api<Annale[]>("/annales?format=reforme")
      .then((a) => setSujets(a.filter((x) => x.type === "sujet" || x.type === "sujet_fictif")))
      .catch(() => setSujets([]));
    api<Copie[]>("/copies").then(setCopies).catch(() => setCopies([]));
  };
  useEffect(load, []);
  const active = copies.find((c) => c.statut === "en_cours");
  const start = async (a: Annale) => {
    setError("");
    try {
      await api("/copies/start", { method: "POST", body: JSON.stringify({ epreuve_code: a.epreuve_code, sujet_url: a.url_source }) });
      load();
    } catch (e) {
      setError((e as Error).message);
    }
  };
  return (
    <section>
      <h1>Sujets chronométrés</h1>
      <p className="muted">Durée réelle de l'épreuve, lue dans la configuration de la session. Le chronomètre tourne côté serveur : il continue si tu fermes la page.</p>
      {error && <p className="error">{error}</p>}
      {active && <Epreuve key={active.id} copie={active} onChange={load} />}
      {!active && (
        <>
          <h2>Choisir un sujet</h2>
          <ul className="list">
            {sujets.map((a) => (
              <li key={a.url_source}>
                {a.annee ?? "Fictif"} · {a.epreuve.replaceAll("_", " ")}
                <a href={a.url_source} target="_blank" rel="noreferrer"> (voir le sujet)</a>{" "}
                <button onClick={() => start(a)}>Démarrer le chrono</button>
              </li>
            ))}
          </ul>
        </>
      )}
      <h2>Mes copies</h2>
      {copies.length === 0 && <p className="muted">Aucune copie pour l'instant.</p>}
      <ul className="list">
        {copies.map((c) => (
          <li key={c.id}>
            {c.epreuve_code} · {c.statut.replace("_", " ")}
            {c.pdf_pages ? ` · ${c.pdf_pages} pages` : ""}
            {c.hors_delai && <span className="badge warn">hors délai</span>}
            {c.note !== null && <span className="badge">auto-évaluation indicative : {c.note}/20</span>}
            {c.statut !== "en_cours" && <> <Link to={`/copies/${c.id}/autoeval`}>{c.note === null ? "M'auto-évaluer" : "Refaire l'auto-évaluation"}</Link></>}
          </li>
        ))}
      </ul>
    </section>
  );
}

function Epreuve({ copie, onChange }: { copie: Copie; onChange: () => void }) {
  // Le serveur est la référence : l'échéance est calculée une fois au montage à partir de son
  // décompte, puis l'affichage suit l'horloge (insensible aux ticks manqués d'un onglet en veille).
  const [deadline] = useState(() => Date.now() + (copie.remaining_seconds ?? 0) * 1000);
  const [now, setNow] = useState(() => Date.now());
  const [error, setError] = useState("");
  useEffect(() => {
    const t = setInterval(() => setNow(Date.now()), 1000);
    return () => clearInterval(t);
  }, []);
  const remaining = Math.max(0, (deadline - now) / 1000);
  const deposer = async (file: File | undefined) => {
    if (!file) return;
    setError("");
    try {
      await uploadPdf(`/copies/${copie.id}/pdf`, file);
      onChange();
    } catch (e) {
      setError((e as Error).message);
    }
  };
  return (
    <div className="card">
      <h2>{copie.epreuve_code} — {copie.duree_minutes / 60} h</h2>
      <p style={{ fontSize: "2rem" }}>{remaining > 0 ? formatDuration(remaining) : "Temps écoulé"}</p>
      <p className="muted">Dépose ta copie en PDF (texte tapé ou scan de ta copie manuscrite).</p>
      <input type="file" accept="application/pdf" onChange={(e) => deposer(e.target.files?.[0])} />
      {error && <p className="error">{error}</p>}
    </div>
  );
}

/** Auto-évaluation d'une copie déposée : un score par critère de la grille. */
export function AutoEval() {
  const { id } = useParams();
  const [copie, setCopie] = useState<Copie | null>(null);
  const [grille, setGrille] = useState<Grille | null>(null);
  const [scores, setScores] = useState<Record<string, number>>({});
  const [note, setNote] = useState<number | null>(null);
  const [error, setError] = useState("");
  useEffect(() => {
    api<Copie>(`/copies/${id}`).then((c) => {
      setCopie(c);
      api<Grille>(`/grilles/${c.epreuve_code}`).then(setGrille).catch(() => setGrille(null));
    });
  }, [id]);
  if (!copie || !grille) return <p className="muted">Chargement…</p>;
  const envoyer = async () => {
    setError("");
    try {
      const r = await api<{ note: number }>(`/copies/${id}/autoeval`, { method: "POST", body: JSON.stringify({ scores }) });
      setNote(r.note);
    } catch (e) {
      setError((e as Error).message);
    }
  };
  return (
    <article>
      <h1>Auto-évaluation : {grille.nom}</h1>
      <p className="muted">{grille.note_sur_les_poids}</p>
      {grille.criteres.map((c) => (
        <section key={c.id}>
          <h2>{c.libelle} <span className="muted">(sur {c.poids})</span></h2>
          <ul>{c.attendus.map((a) => <li key={a}>{a}</li>)}</ul>
          <input type="number" min={0} max={c.poids} step={0.5} value={scores[c.id] ?? ""}
            onChange={(e) => setScores({ ...scores, [c.id]: Number(e.target.value) })} />
        </section>
      ))}
      <button onClick={envoyer}>Calculer ma note</button>
      {error && <p className="error">{error}</p>}
      {note !== null && <p><strong>Note indicative : {note}/20.</strong> Elle ne préjuge pas de la note d'un jury.</p>}
    </article>
  );
}

/** Annales : le format réformé est présenté en premier ; l'ancien format sert de matière thématique. */
export function Annales() {
  const [list, setList] = useState<Annale[]>([]);
  const [epreuve, setEpreuve] = useState("");
  useEffect(() => {
    const p = new URLSearchParams();
    if (epreuve) p.set("epreuve", epreuve);
    api<Annale[]>(`/annales?${p}`).then(setList).catch(() => setList([]));
  }, [epreuve]);
  const row = (a: Annale) => (
    <li key={a.url_source}>
      <a href={a.url_source} target="_blank" rel="noreferrer">
        {a.annee ?? "Fictif"} · {TYPE_ANNALE[a.type] ?? a.type} · {a.epreuve.replaceAll("_", " ")}
      </a>
    </li>
  );
  return (
    <section>
      <h1>Annales et rapports du jury</h1>
      <p className="muted">Source : insp.gouv.fr (concours externe, voie générale).</p>
      <select value={epreuve} onChange={(e) => setEpreuve(e.target.value)}>
        <option value="">Toutes les épreuves</option>
        {EPREUVES.map(([c, l]) => <option key={c} value={c}>{l}</option>)}
      </select>
      <h2>Format réformé (2024 et après)</h2>
      <ul className="list">{list.filter((a) => a.format === "reforme").map(row)}</ul>
      <h2>Ancien format</h2>
      <p className="muted">Matière thématique uniquement, pas un modèle de format.</p>
      <ul className="list">{list.filter((a) => a.format === "ancien").map(row)}</ul>
      <h2>Grilles d'évaluation</h2>
      <ul className="list">
        {EPREUVES.map(([c, l]) => <li key={c}><Link to={`/grilles/${c}`}>{l}</Link></li>)}
      </ul>
    </section>
  );
}

export function GrillePage() {
  const { code } = useParams();
  const [g, setG] = useState<Grille | null>(null);
  useEffect(() => { api<Grille>(`/grilles/${code}`).then(setG).catch(() => setG(null)); }, [code]);
  if (!g) return <p className="muted">Chargement…</p>;
  return (
    <article>
      <h1>{g.nom}</h1>
      <p className="muted">Source : {g.source.document}, p. {g.source.pages}. {g.note_sur_les_poids}</p>
      {g.criteres.map((c) => (
        <section key={c.id}>
          <h2>{c.libelle} <span className="muted">({c.poids}/{g.total}{c.poids_officiel ? ", barème officiel" : ", indicatif"})</span></h2>
          <ul>{c.attendus.map((a) => <li key={a}>{a}</li>)}</ul>
        </section>
      ))}
      <h2>Erreurs fréquentes</h2>
      <ul>{g.erreurs_frequentes.map((e) => <li key={e}>{e}</li>)}</ul>
      <h2>Conseils du jury</h2>
      <ul>{g.conseils.map((e) => <li key={e}>{e}</li>)}</ul>
    </article>
  );
}

/** Bouton « signaler une erreur » présent sur chaque fiche, flashcard et corrigé. */
export function SignalButton({ type, id }: { type: string; id: string }) {
  const [sent, setSent] = useState(false);
  const send = async () => {
    const message = window.prompt("Décris l'erreur constatée :");
    if (!message?.trim()) return;
    await api("/signalements", {
      method: "POST",
      body: JSON.stringify({ cible_type: type, cible_id: id, message }),
    });
    setSent(true);
  };
  return (
    <button className="link" onClick={send} disabled={sent}>
      {sent ? "Signalement envoyé" : "Signaler une erreur"}
    </button>
  );
}

export function Fiches() {
  const [list, setList] = useState<FicheSummary[]>([]);
  const [q, setQ] = useState("");
  const [matiere, setMatiere] = useState("");
  useEffect(() => {
    const p = new URLSearchParams();
    if (q) p.set("q", q);
    if (matiere) p.set("matiere", matiere);
    p.set("type", "fiche");
    api<FicheSummary[]>(`/fiches?${p}`).then(setList).catch(() => setList([]));
  }, [q, matiere]);
  return (
    <section>
      <h1>Fiches de cours</h1>
      <div className="row">
        <input placeholder="Rechercher un titre" value={q} onChange={(e) => setQ(e.target.value)} />
        <select value={matiere} onChange={(e) => setMatiere(e.target.value)}>
          <option value="">Toutes les matières</option>
          {["droit_public", "economie", "finances_publiques", "questions_sociales", "europe_international", "transitions"].map((m) => (
            <option key={m} value={m}>{matiereLabel(m)}</option>
          ))}
        </select>
      </div>
      {list.length === 0 && <p className="muted">Aucune fiche pour ces critères.</p>}
      <ul className="list">
        {list.map((f) => (
          <li key={f.slug}>
            <Link to={`/fiches/${f.slug}`}>{f.titre}</Link>
            <span className="muted"> {matiereLabel(f.matiere)} · mise à jour {f.maj}</span>
            <span className={`badge level${statutLevel(f.statut)}`}>{f.statut}</span>
            {f.a_verifier && <span className="badge warn">à vérifier, actualité récente</span>}
          </li>
        ))}
      </ul>
    </section>
  );
}

interface FicheFull extends FicheSummary {
  id: string;
  sources: string[];
  body_md: string;
}

export function FichePage() {
  const { slug } = useParams();
  const [f, setF] = useState<FicheFull | null>(null);
  useEffect(() => {
    api<FicheFull>(`/fiches/${slug}`).then(setF).catch(() => setF(null));
  }, [slug]);
  if (!f) return <p className="muted">Chargement…</p>;
  return (
    <article>
      <p className="muted">
        {matiereLabel(f.matiere)} · mise à jour {f.maj} · <span className={`badge level${statutLevel(f.statut)}`}>{f.statut}</span>
      </p>
      <div className="cours"><Markdown
        remarkPlugins={[remarkGfm, remarkCallouts]}
        remarkRehypeOptions={{ footnoteLabel: "Notes de bas de page", footnoteBackLabel: "Retour au texte" }}
      >{f.body_md}</Markdown></div>
      <h2>Sources</h2>
      <ul>{f.sources.map((s) => <li key={s}>{s}</li>)}</ul>
      <SignalButton type="fiche" id={f.slug} />
    </article>
  );
}

/** Révision des cartes dues : l'utilisateur s'auto-évalue de 0 à 5 (SM-2 côté serveur). */
export function Cartes() {
  const [cards, setCards] = useState<Card[]>([]);
  const [shown, setShown] = useState(false);
  const load = () => api<Card[]>("/flashcards/due").then(setCards).catch(() => setCards([]));
  useEffect(() => { load(); }, []);
  const card = cards[0];
  const grade = async (quality: number) => {
    await api(`/flashcards/${encodeURIComponent(card.id)}/review`, { method: "POST", body: JSON.stringify({ quality }) });
    setShown(false);
    setCards(cards.slice(1));
  };
  if (!card) return <section><h1>Flashcards</h1><p className="muted">Aucune carte à réviser pour le moment.</p></section>;
  return (
    <section>
      <h1>Flashcards <span className="muted">({cards.length} à réviser)</span></h1>
      <div className="card">
        <p>{card.recto}</p>
        {shown ? (
          <>
            <p><strong>{card.verso}</strong></p>
            <p className="muted">Source : {card.source_ref}</p>
            <div className="row">
              {[["À revoir", 1], ["Difficile", 3], ["Bien", 4], ["Facile", 5]].map(([l, q]) => (
                <button key={l} onClick={() => grade(q as number)}>{l}</button>
              ))}
            </div>
          </>
        ) : (
          <button onClick={() => setShown(true)}>Afficher la réponse</button>
        )}
        <SignalButton type="flashcard" id={card.id} />
      </div>
    </section>
  );
}

export function Reglages() {
  const [me, setMe] = useState<Me | null>(null);
  useEffect(() => { api<Me>("/me").then(setMe); }, []);
  if (!me) return null;
  const save = async (majeure: string) => {
    const value = majeure || null;
    await api("/me", { method: "PUT", body: JSON.stringify({ majeure: value, session: me.session }) });
    setMe({ ...me, majeure: value, mineure: value === "droit_public" ? "economie" : value === "economie" ? "droit_public" : null });
  };
  return (
    <section>
      <h1>Réglages</h1>
      <label>Matière majeure
        <select value={me.majeure ?? ""} onChange={(e) => save(e.target.value)}>
          <option value="">Pas encore choisie (égalité)</option>
          <option value="droit_public">Droit public</option>
          <option value="economie">Économie</option>
        </select>
      </label>
      <p className="muted">Mineure : {me.mineure ? matiereLabel(me.mineure) : "traitée à égalité avec la majeure"}</p>
    </section>
  );
}

interface Signalement { id: string; cible_type: string; cible_id: string; message: string; status: string; created_at: string; pseudo: string }

export function Signalements() {
  const [list, setList] = useState<Signalement[]>([]);
  useEffect(() => { api<Signalement[]>("/signalements").then(setList).catch(() => setList([])); }, []);
  return (
    <section>
      <h1>Signalements</h1>
      {list.length === 0 && <p className="muted">Aucun signalement.</p>}
      <ul className="list">
        {list.map((s) => (
          <li key={s.id}>
            <strong>{s.cible_type} {s.cible_id}</strong> — {s.message}
            <span className="muted"> par {s.pseudo}, {s.created_at.slice(0, 10)} ({s.status})</span>
          </li>
        ))}
      </ul>
    </section>
  );
}
