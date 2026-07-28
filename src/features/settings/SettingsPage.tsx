import { clsx } from "clsx";

import { Card } from "@/shared/components/ui/Card";
import { useThemeStore, type AccentColor, type ThemeMode } from "@/shared/stores/theme-store";

const THEMES: { value: ThemeMode; label: string }[] = [
  { value: "dark", label: "Sombre" },
  { value: "oled", label: "OLED (noir pur)" },
  { value: "light", label: "Clair" },
];

const ACCENTS: { value: AccentColor; label: string; swatch: string }[] = [
  { value: "teal", label: "Turquoise", swatch: "#1fb69c" },
  { value: "violet", label: "Violet", swatch: "#8b5cf6" },
  { value: "amber", label: "Ambre", swatch: "#f59e0b" },
  { value: "rose", label: "Rose", swatch: "#f43f5e" },
];

export function SettingsPage() {
  const theme = useThemeStore((state) => state.theme);
  const accent = useThemeStore((state) => state.accent);
  const setTheme = useThemeStore((state) => state.setTheme);
  const setAccent = useThemeStore((state) => state.setAccent);

  return (
    <div className="mx-auto flex max-w-2xl flex-col gap-6 py-8">
      <h1 className="text-2xl font-semibold text-slate-100">Paramètres</h1>

      <Card heading="Thème" glass>
        <div className="flex gap-2">
          {THEMES.map((t) => (
            <button
              key={t.value}
              onClick={() => setTheme(t.value)}
              className={clsx(
                "rounded-xl border px-4 py-2 text-sm transition-colors",
                theme === t.value
                  ? "border-[var(--color-accent-500)] bg-[var(--color-accent-500)]/10 text-[var(--color-accent-400)]"
                  : "border-[var(--color-border-subtle)] text-slate-300 hover:bg-[var(--color-surface-2)]",
              )}
            >
              {t.label}
            </button>
          ))}
        </div>
      </Card>

      <Card heading="Couleur d'accent" glass>
        <div className="flex gap-3">
          {ACCENTS.map((a) => (
            <button
              key={a.value}
              onClick={() => setAccent(a.value)}
              aria-label={a.label}
              className={clsx(
                "h-9 w-9 rounded-full ring-offset-2 ring-offset-[var(--color-surface-1)] transition-shadow",
                accent === a.value && "ring-2 ring-white/80",
              )}
              style={{ background: a.swatch }}
            />
          ))}
        </div>
      </Card>

      <Card heading="Comptes Riot" glass>
        <p className="text-sm text-slate-400">
          La gestion multi-comptes sera disponible ici une fois l'onboarding implémenté (voir Epic 2
          de la roadmap).
        </p>
      </Card>
    </div>
  );
}
