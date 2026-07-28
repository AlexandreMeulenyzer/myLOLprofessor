import { expect, test } from "@playwright/test";

test.describe("Paramètres", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/#/settings");
  });

  test("change de thème et le persiste après rechargement", async ({ page }) => {
    await page.getByRole("button", { name: "Clair" }).click();
    await expect(page.locator("html")).toHaveAttribute("data-theme", "light");

    await page.reload();
    await expect(page.locator("html")).toHaveAttribute("data-theme", "light");
  });

  test("change la couleur d'accent", async ({ page }) => {
    await page.getByRole("button", { name: "Violet" }).click();
    await expect(page.locator("html")).toHaveAttribute("data-accent", "violet");
  });

  test("désactive un widget d'overlay et le persiste", async ({ page }) => {
    const checkbox = page.getByLabel("Conseil contextuel");
    await expect(checkbox).toBeChecked();

    await checkbox.uncheck();
    await expect(checkbox).not.toBeChecked();

    await page.reload();
    await expect(page.getByLabel("Conseil contextuel")).not.toBeChecked();
  });

  test("désactive une catégorie de notification et la persiste", async ({ page }) => {
    const checkbox = page.getByLabel("Nouveau patch disponible");
    await expect(checkbox).toBeChecked();

    await checkbox.uncheck();
    await expect(checkbox).not.toBeChecked();

    await page.reload();
    await expect(page.getByLabel("Nouveau patch disponible")).not.toBeChecked();
  });
});
