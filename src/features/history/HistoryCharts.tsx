import {
  Bar,
  BarChart,
  CartesianGrid,
  LabelList,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";

import { useLpHistory } from "@/features/profile/hooks";
import { useChampionsById } from "@/features/static-data/hooks";
import { Card } from "@/shared/components/ui/Card";

import type { MatchHistoryEntry } from "./types";

const GRID_COLOR = "rgba(148, 163, 184, 0.15)";
const AXIS_COLOR = "rgb(100, 116, 139)";
const ACCENT_COLOR = "#37cfb4";

function tooltipContentStyle() {
  return {
    background: "var(--color-surface-2)",
    border: "1px solid var(--color-border-subtle)",
    borderRadius: "0.75rem",
    fontSize: "0.75rem",
    color: "#e2e8f0",
  };
}

function formatDate(iso: string): string {
  const date = new Date(iso);
  return date.toLocaleDateString("fr-FR", { day: "2-digit", month: "2-digit" });
}

export function LpProgressionChart({ puuid }: { puuid: string }) {
  const { data: snapshots } = useLpHistory(puuid, "RANKED_SOLO_5x5");

  if (!snapshots || snapshots.length < 2) {
    return (
      <Card heading="Progression LP (Solo/Duo)" glass>
        <p className="text-sm text-slate-400">
          Pas encore assez de données. Un point est enregistré à chaque consultation du profil —
          revenez après quelques parties.
        </p>
      </Card>
    );
  }

  const points = snapshots.map((snapshot) => ({
    date: formatDate(snapshot.capturedAt),
    lp: snapshot.leaguePoints,
    label: `${snapshot.tier} ${snapshot.rank}`,
  }));

  return (
    <Card heading="Progression LP (Solo/Duo)" glass>
      <div className="h-48 w-full">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={points} margin={{ top: 8, right: 12, bottom: 0, left: -12 }}>
            <CartesianGrid stroke={GRID_COLOR} vertical={false} />
            <XAxis
              dataKey="date"
              tick={{ fill: AXIS_COLOR, fontSize: 11 }}
              axisLine={{ stroke: GRID_COLOR }}
              tickLine={false}
            />
            <YAxis
              tick={{ fill: AXIS_COLOR, fontSize: 11 }}
              axisLine={false}
              tickLine={false}
              width={36}
            />
            <Tooltip
              contentStyle={tooltipContentStyle()}
              labelStyle={{ color: "#94a3b8" }}
              formatter={(value, _name, item) => [
                `${value} LP (${item.payload.label})`,
                "Progression",
              ]}
            />
            <Line
              type="monotone"
              dataKey="lp"
              stroke={ACCENT_COLOR}
              strokeWidth={2}
              dot={{ r: 4, fill: ACCENT_COLOR, strokeWidth: 0 }}
              activeDot={{ r: 5 }}
            />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </Card>
  );
}

const CHAMPION_POOL_LIMIT = 8;

export function ChampionPoolChart({ matches }: { matches: MatchHistoryEntry[] }) {
  const championsById = useChampionsById();
  const counts = new Map<string, number>();
  matches.forEach((match) => counts.set(match.champion, (counts.get(match.champion) ?? 0) + 1));

  const data = [...counts.entries()]
    .map(([champion, games]) => ({
      champion: championsById.get(champion)?.name ?? champion,
      games,
    }))
    .sort((a, b) => b.games - a.games)
    .slice(0, CHAMPION_POOL_LIMIT);

  if (data.length === 0) {
    return (
      <Card heading="Pool de champions" glass>
        <p className="text-sm text-slate-400">Aucune partie synchronisée pour l'instant.</p>
      </Card>
    );
  }

  return (
    <Card heading="Pool de champions" glass>
      <div className="h-56 w-full">
        <ResponsiveContainer width="100%" height="100%">
          <BarChart
            data={data}
            layout="vertical"
            margin={{ top: 4, right: 24, bottom: 4, left: 4 }}
          >
            <CartesianGrid stroke={GRID_COLOR} horizontal={false} />
            <XAxis type="number" hide />
            <YAxis
              type="category"
              dataKey="champion"
              tick={{ fill: "#cbd5e1", fontSize: 12 }}
              axisLine={false}
              tickLine={false}
              width={90}
            />
            <Tooltip
              contentStyle={tooltipContentStyle()}
              cursor={{ fill: "rgba(255,255,255,0.04)" }}
              formatter={(value) => [`${value} partie(s)`, "Parties jouées"]}
            />
            <Bar dataKey="games" fill={ACCENT_COLOR} radius={[0, 4, 4, 0]} barSize={14}>
              <LabelList dataKey="games" position="right" fill="#cbd5e1" fontSize={11} />
            </Bar>
          </BarChart>
        </ResponsiveContainer>
      </div>
    </Card>
  );
}
