import { useState } from "react";
import { BrowserRouter, NavLink, Navigate, Route, Routes } from "react-router-dom";
import { api, getToken, setToken } from "./api";
import { Annales, AutoEval, Cartes, Entrainement, FichePage, Fiches, GrillePage, Reglages, Signalements } from "./pages";

function Login({ onDone }: { onDone: () => void }) {
  const [pseudo, setPseudo] = useState("");
  const [error, setError] = useState("");
  const submit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const r = await api<{ token: string }>("/login", { method: "POST", body: JSON.stringify({ pseudo }) });
      setToken(r.token);
      onDone();
    } catch (err) {
      setError((err as Error).message);
    }
  };
  return (
    <form className="login" onSubmit={submit}>
      <h1>Préparation INSP</h1>
      <p className="muted">Entre ton pseudo. Le compte est créé à la première connexion.</p>
      <input value={pseudo} onChange={(e) => setPseudo(e.target.value)} placeholder="Pseudo" autoFocus />
      <button type="submit">Se connecter</button>
      {error && <p className="error">{error}</p>}
    </form>
  );
}

export default function App() {
  const [logged, setLogged] = useState(!!getToken());
  if (!logged) return <Login onDone={() => setLogged(true)} />;
  return (
    <BrowserRouter>
      <header>
        <strong>Prépa INSP</strong>
        <nav>
          <NavLink to="/fiches">Fiches</NavLink>
          <NavLink to="/cartes">Flashcards</NavLink>
          <NavLink to="/annales">Annales</NavLink>
          <NavLink to="/entrainement">Sujets chrono</NavLink>
          <NavLink to="/signalements">Signalements</NavLink>
          <NavLink to="/reglages">Réglages</NavLink>
          <button className="link" onClick={() => { setToken(null); setLogged(false); }}>Quitter</button>
        </nav>
      </header>
      <main>
        <Routes>
          <Route path="/" element={<Navigate to="/fiches" replace />} />
          <Route path="/fiches" element={<Fiches />} />
          <Route path="/fiches/:slug" element={<FichePage />} />
          <Route path="/cartes" element={<Cartes />} />
          <Route path="/annales" element={<Annales />} />
          <Route path="/entrainement" element={<Entrainement />} />
          <Route path="/copies/:id/autoeval" element={<AutoEval />} />
          <Route path="/grilles/:code" element={<GrillePage />} />
          <Route path="/signalements" element={<Signalements />} />
          <Route path="/reglages" element={<Reglages />} />
        </Routes>
      </main>
    </BrowserRouter>
  );
}
