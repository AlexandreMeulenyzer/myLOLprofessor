import { existsSync } from "node:fs";

import { defineConfig, devices } from "@playwright/test";

// Certains environnements sandboxes fournissent un Chromium pre-installe a
// ce chemin (variable PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD) ; ailleurs (CI, poste
// de developpement classique) Playwright utilise le Chromium installe via
// `playwright install`.
const SANDBOX_CHROMIUM_PATH = "/opt/pw-browsers/chromium";
const executablePath = existsSync(SANDBOX_CHROMIUM_PATH) ? SANDBOX_CHROMIUM_PATH : undefined;

/**
 * Wardstone est une application desktop Tauri : ces tests E2E exercent le
 * frontend seul (via le serveur de dev Vite), sans le shell Tauri ni le
 * backend Rust. Toute commande IPC (`invoke`) est protegee par
 * `isTauriRuntime()` cote frontend et se degrade silencieusement hors de ce
 * runtime — ces tests couvrent donc la navigation, l'etat local (themes,
 * widgets, notifications) et le rendu des pages, pas les flux dependant du
 * client League ou de l'API Riot (qui necessitent le binaire desktop reel).
 */
export default defineConfig({
  testDir: "./tests/e2e",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  reporter: "list",
  use: {
    baseURL: "http://localhost:1420",
    trace: "on-first-retry",
  },
  webServer: {
    command: "npm run dev",
    url: "http://localhost:1420",
    reuseExistingServer: !process.env.CI,
    timeout: 30_000,
  },
  projects: [
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        launchOptions: executablePath ? { executablePath } : {},
      },
    },
  ],
});
