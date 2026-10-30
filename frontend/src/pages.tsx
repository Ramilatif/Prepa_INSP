import { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import Markdown from "react-markdown";
import { api, type Card, type FicheSummary, type Me } from "./api";
import { matiereLabel, statutLevel } from "./labels";

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
      <Markdown>{f.body_md}</Markdown>
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
