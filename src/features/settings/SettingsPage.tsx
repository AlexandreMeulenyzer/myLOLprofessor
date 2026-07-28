import { clsx } from "clsx";
import { useState } from "react";
import { Link } from "react-router-dom";

import { Button } from "@/shared/components/ui/Button";
import { Card } from "@/shared/components/ui/Card";
import { invoke } from "@/shared/lib/tauri-bridge";
import { useNotificationSettingsStore } from "@/shared/stores/notification-settings-store";
import { useOverlaySettingsStore } from "@/shared/stores/overlay-settings-store";
import { useThemeStore, type AccentColor, type ThemeMode } from "@/shared/stores/theme-store";

const OVERLAY_WIDGETS = [
  { key: "showGoldAndLevel", label: "Or & niveau" },
  { key: "showObjectiveTimers", label: "Timers d'objectifs (Dragon/Baron/Héraut)" },
  { key: "showContextualTip", label: "Conseil contextuel" },
] as const;

const NOTIFICATION_CATEGORIES = [
  { key: "queueFound", label: "Partie trouvée" },
  { key: "championSelect", label: "Entrée en sélection de champion" },
  { key: "winLoss", label: "Victoire / défaite" },
  { key: "promotion", label: "Promotion de rang" },
  { key: "objectiveReached", label: "Objectif atteint" },
  { key: "patchUpdate", label: "Nouveau patch disponible" },
] as const;

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
  const overlaySettings = useOverlaySettingsStore();
  const notificationSettings = useNotificationSettingsStore();
  const [overlayOpen, setOverlayOpen] = useState(false);
  const [overlayError, setOverlayError] = useState<string | null>(null);

  async function handleToggleOverlay() {
    try {
      const isOpen = await invoke<boolean>("toggle_overlay_window");
      setOverlayOpen(isOpen);
      setOverlayError(null);
    } catch (error) {
      setOverlayError(error instanceof Error ? error.message : String(error));
    }
  }

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

      <Card heading="Overlay in-game" glass>
        <p className="mb-3 text-sm text-slate-400">
          Fenêtre flottante affichée automatiquement en partie. Raccourci clavier global{" "}
          <kbd className="rounded bg-[var(--color-surface-2)] px-1.5 py-0.5 text-xs">
            Ctrl+Shift+O
          </kbd>{" "}
          pour la basculer manuellement, même quand le client League a le focus.
        </p>
        <Button variant="secondary" size="sm" onClick={handleToggleOverlay}>
          {overlayOpen ? "Fermer l'overlay" : "Ouvrir l'overlay"}
        </Button>
        {overlayError && <p className="mt-2 text-xs text-[var(--color-loss)]">{overlayError}</p>}

        <div className="mt-4 flex flex-col gap-2 border-t border-[var(--color-border-subtle)] pt-4">
          <p className="text-xs font-medium uppercase tracking-wide text-slate-500">
            Widgets affichés
          </p>
          {OVERLAY_WIDGETS.map((widget) => (
            <label key={widget.key} className="flex items-center gap-2 text-sm text-slate-300">
              <input
                type="checkbox"
                checked={overlaySettings[widget.key]}
                onChange={() => overlaySettings.toggleWidget(widget.key)}
                className="h-4 w-4 rounded border-[var(--color-border-subtle)] accent-[var(--color-accent-500)]"
              />
              {widget.label}
            </label>
          ))}
        </div>
      </Card>

      <Card heading="Notifications" glass>
        <p className="mb-3 text-sm text-slate-400">
          Notifications système envoyées par Wardstone. Le premier envoi demande la permission du
          système d'exploitation.
        </p>
        <div className="flex flex-col gap-2">
          {NOTIFICATION_CATEGORIES.map((category) => (
            <label key={category.key} className="flex items-center gap-2 text-sm text-slate-300">
              <input
                type="checkbox"
                checked={notificationSettings[category.key]}
                onChange={() => notificationSettings.toggle(category.key)}
                className="h-4 w-4 rounded border-[var(--color-border-subtle)] accent-[var(--color-accent-500)]"
              />
              {category.label}
            </label>
          ))}
        </div>
      </Card>

      <Card heading="Comptes Riot" glass>
        <p className="text-sm text-slate-400">
          La liaison et la bascule entre comptes Riot se gèrent depuis la page{" "}
          <Link to="/accounts" className="text-[var(--color-accent-400)] hover:underline">
            Comptes
          </Link>
          .
        </p>
      </Card>
    </div>
  );
}
