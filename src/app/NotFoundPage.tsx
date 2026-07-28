import { Link } from "react-router-dom";

export function NotFoundPage() {
  return (
    <div className="flex h-full flex-col items-center justify-center gap-3 text-slate-400">
      <p className="text-lg">Page introuvable.</p>
      <Link to="/" className="text-[var(--color-accent-400)] hover:underline">
        Retour au dashboard
      </Link>
    </div>
  );
}
