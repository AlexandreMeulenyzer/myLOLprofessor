import { Outlet } from "react-router-dom";

import { useNotificationOrchestrator } from "@/features/notifications/useNotificationOrchestrator";
import { useChampSelectRosterCapture } from "@/features/team-analysis/useChampSelectRosterCapture";
import { Sidebar } from "@/shared/components/layout/Sidebar";
import { Topbar } from "@/shared/components/layout/Topbar";
import { useGamePhaseSync } from "@/shared/hooks/useGamePhaseSync";

export function AppShell() {
  useGamePhaseSync();
  useNotificationOrchestrator();
  useChampSelectRosterCapture();

  return (
    <div className="flex h-screen w-screen overflow-hidden bg-[var(--color-surface-0)] text-slate-100">
      <Sidebar />
      <div className="flex min-w-0 flex-1 flex-col">
        <Topbar />
        <main className="flex-1 overflow-y-auto p-6">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
