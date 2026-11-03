import { renderToStaticMarkup } from "react-dom/server";
import Markdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { describe, expect, it } from "vitest";
import remarkCallouts from "./remarkCallouts";

const render = (md: string) =>
  renderToStaticMarkup(
    <Markdown
      remarkPlugins={[remarkGfm, remarkCallouts]}
      remarkRehypeOptions={{ footnoteLabel: "Notes de bas de page", footnoteBackLabel: "Retour au texte" }}
    >
      {md}
    </Markdown>,
  );

describe("rendu d'un cours", () => {
  it("affiche une note de bas de page avec son libellé français", () => {
    const html = render("Texte[^1].\n\n[^1]: CE, Sarran, https://exemple.fr");
    expect(html).toContain("Notes de bas de page");
    expect(html).toContain("Sarran");
  });
  it("transforme un encadré annoté et un tableau", () => {
    const html = render("> **Piège** : attention.\n\n| a | b |\n|---|---|\n| 1 | 2 |");
    expect(html).toContain('class="callout piege"');
    expect(html).toContain("<table>");
  });
});
