import { defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";

// En dev, les appels /api sont relayés vers le backend ; en production, nginx fait le même relais.
export default defineConfig({
  plugins: [react()],
  server: { proxy: { "/api": process.env.API_URL ?? "http://localhost:8080" } },
  test: { environment: "jsdom" },
});
