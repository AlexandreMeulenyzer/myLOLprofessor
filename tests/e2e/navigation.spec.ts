import { expect, test } from "@playwright/test";

/**
 * Ces tests s'executent hors du shell Tauri (voir playwright.config.ts) :
 * sans compte lie ni cle API, chaque page retombe sur un etat d'accueil
 * previsible que l'on peut verifier de maniere stable.
 */
test.describe("Navigation principale", () => {
  test("affiche l'ecran de bienvenue sans compte lie", async ({ page }) => {
    await page.goto("/");
    await expect(page.getByRole("heading", { name: "Bienvenue sur Wardstone" })).toBeVisible();
  });

  test("permet de naviguer vers chaque page principale", async ({ page }) => {
    await page.goto("/");

    const routes: { link: string; expectedText: string }[] = [
      { link: "Comptes", expectedText: "Comptes Riot" },
      { link: "Profil", expectedText: "Aucun compte actif" },
      { link: "Sélection", expectedText: "L'assistant s'affichera automatiquement" },
      { link: "Équipes", expectedText: "L'analyse d'équipe s'affiche automatiquement" },
      { link: "Historique", expectedText: "Historique" },
      { link: "Objectifs", expectedText: "Liez un compte pour définir des objectifs" },
      { link: "Coaching", expectedText: "Liez un compte pour accéder au coaching" },
      { link: "Comparer", expectedText: "Comparaison" },
      { link: "Recherche", expectedText: "Recherche globale" },
    ];

    for (const route of routes) {
      await page.getByRole("link", { name: route.link, exact: true }).click();
      await expect(page.getByText(route.expectedText).first()).toBeVisible();
    }
  });

  test("la page Parametres est accessible", async ({ page }) => {
    await page.goto("/");
    await page.getByRole("link", { name: "Paramètres" }).click();
    await expect(page.getByRole("heading", { name: "Paramètres" })).toBeVisible();
  });

  test("affiche une page 404 pour une route inconnue", async ({ page }) => {
    await page.goto("/#/route-qui-nexiste-pas");
    await expect(page.getByText(/404|introuvable/i).first()).toBeVisible();
  });
});
