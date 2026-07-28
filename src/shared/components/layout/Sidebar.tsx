import { NavLink } from "react-router-dom";
import { clsx } from "clsx";

const NAV_ITEMS = [
  { to: "/", label: "Dashboard", icon: "◆", end: true },
  { to: "/accounts", label: "Comptes", icon: "👤" },
  { to: "/profile", label: "Profil", icon: "☺" },
  { to: "/champion-select", label: "Sélection", icon: "⚔" },
  { to: "/team-analysis", label: "Équipes", icon: "⚑" },
  { to: "/history", label: "Historique", icon: "☰" },
  { to: "/objectives", label: "Objectifs", icon: "◎" },
  { to: "/coaching", label: "Coaching", icon: "✦" },
  { to: "/comparison", label: "Comparer", icon: "⇄" },
  { to: "/search", label: "Recherche", icon: "🔍" },
] as const;

export function Sidebar() {
  return (
    <aside className="flex w-60 shrink-0 flex-col border-r border-[var(--color-border-subtle)] bg-[var(--color-surface-1)]/60 px-3 py-4">
      <div className="mb-6 flex items-center gap-2 px-2">
        <div className="h-7 w-7 rounded-lg bg-[var(--color-accent-500)]" />
        <span className="text-base font-semibold tracking-tight text-slate-100">Wardstone</span>
      </div>

      <nav className="flex flex-1 flex-col gap-1">
        {NAV_ITEMS.map((item) => (
          <NavLink
            key={item.to}
            to={item.to}
            end={"end" in item ? item.end : false}
            className={({ isActive }) =>
              clsx(
                "flex items-center gap-3 rounded-xl px-3 py-2 text-sm font-medium transition-colors",
                isActive
                  ? "bg-[var(--color-accent-500)]/15 text-[var(--color-accent-400)]"
                  : "text-slate-400 hover:bg-[var(--color-surface-2)] hover:text-slate-100",
              )
            }
          >
            <span aria-hidden className="w-4 text-center">
              {item.icon}
            </span>
            {item.label}
          </NavLink>
        ))}
      </nav>

      <NavLink
        to="/settings"
        className={({ isActive }) =>
          clsx(
            "flex items-center gap-3 rounded-xl px-3 py-2 text-sm font-medium transition-colors",
            isActive
              ? "bg-[var(--color-accent-500)]/15 text-[var(--color-accent-400)]"
              : "text-slate-400 hover:bg-[var(--color-surface-2)] hover:text-slate-100",
          )
        }
      >
        <span aria-hidden className="w-4 text-center">
          ⚙
        </span>
        Paramètres
      </NavLink>
    </aside>
  );
}
