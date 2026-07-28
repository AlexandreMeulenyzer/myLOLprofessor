import { useState } from "react";
import { Link } from "react-router-dom";

import { useMatchHistory } from "@/features/history/hooks";
import { queueName } from "@/features/history/types";
import { Badge } from "@/shared/components/ui/Badge";
import { Card } from "@/shared/components/ui/Card";
import { useActiveAccountStore } from "@/shared/stores/active-account-store";

import { useCoachingReport } from "./hooks";

export function CoachingPage() {
  const activeAccount = useActiveAccountStore((state) => state.activeAccount);
  const { data: matches } = useMatchHistory(activeAccount?.puuid, 15);
  const [selectedMatchId, setSelectedMatchId] = useState<string | undefined>(undefined);
  const { data: report, isLoading } = useCoachingReport(activeAccount?.puuid, selectedMatchId);

  if (!activeAccount) {
    return (
      <div className="mx-auto max-w-lg py-16 text-center">
        <p className="text-slate-400">Liez un compte pour accéder au coaching post-partie.</p>
        <Link
          to="/accounts"
          className="mt-3 inline-block text-[var(--color-accent-400)] hover:underline"
        >
          Aller à la page Comptes →
        </Link>
      </div>
    );
  }

  return (
    <div className="mx-auto flex max-w-3xl gap-6 py-8">
      <div className="w-64 shrink-0">
        <h1 className="mb-3 text-lg font-semibold text-slate-100">Coaching post-partie</h1>
        <div className="flex flex-col gap-1.5">
          {(matches ?? []).map((match) => (
            <button
              key={match.matchId}
              onClick={() => setSelectedMatchId(match.matchId)}
              className={`flex items-center justify-between rounded-lg px-2.5 py-2 text-left text-xs ${
                selectedMatchId === match.matchId
                  ? "bg-[var(--color-accent-500)]/15 text-[var(--color-accent-400)]"
                  : "text-slate-300 hover:bg-[var(--color-surface-2)]"
              }`}
            >
              <span>
                <Badge tone={match.win ? "win" : "loss"}>{match.win ? "V" : "D"}</Badge>{" "}
                {match.champion}
              </span>
              <span className="text-slate-500">{queueName(match.queueId)}</span>
            </button>
          ))}
          {(!matches || matches.length === 0) && (
            <p className="text-xs text-slate-500">
              Aucune partie synchronisée —{" "}
              <Link to="/history" className="text-[var(--color-accent-400)] hover:underline">
                Historique
              </Link>
              .
            </p>
          )}
        </div>
      </div>

      <div className="flex-1">
        {!selectedMatchId && (
          <p className="text-sm text-slate-400">Sélectionnez une partie pour voir son analyse.</p>
        )}
        {isLoading && <p className="text-sm text-slate-400">Analyse en cours…</p>}
        {report && (
          <div className="flex flex-col gap-4">
            <Card heading="Points forts" glass>
              {report.strengths.length === 0 ? (
                <p className="text-sm text-slate-400">Aucun point fort marquant détecté.</p>
              ) : (
                <ul className="list-inside list-disc space-y-1 text-sm text-[var(--color-win)]">
                  {report.strengths.map((s, i) => (
                    <li key={i}>{s}</li>
                  ))}
                </ul>
              )}
            </Card>
            <Card heading="Points faibles" glass>
              {report.weaknesses.length === 0 ? (
                <p className="text-sm text-slate-400">Aucune faiblesse majeure détectée.</p>
              ) : (
                <ul className="list-inside list-disc space-y-1 text-sm text-[var(--color-loss)]">
                  {report.weaknesses.map((w, i) => (
                    <li key={i}>{w}</li>
                  ))}
                </ul>
              )}
            </Card>
            <Card heading="Conseils prioritaires" glass>
              <ul className="list-inside list-disc space-y-1 text-sm text-slate-300">
                {report.tips.map((tip, i) => (
                  <li key={i}>{tip}</li>
                ))}
              </ul>
            </Card>
          </div>
        )}
      </div>
    </div>
  );
}
