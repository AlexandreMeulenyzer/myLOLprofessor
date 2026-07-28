import { createHashRouter } from "react-router-dom";

import { AppShell } from "@/shared/components/layout/AppShell";
import { NotFoundPage } from "@/app/NotFoundPage";
import { AccountsPage } from "@/features/accounts/AccountsPage";
import { DashboardPage } from "@/features/dashboard/DashboardPage";
import { ProfilePage } from "@/features/profile/ProfilePage";
import { ChampionSelectPage } from "@/features/champion-select/ChampionSelectPage";
import { TeamAnalysisPage } from "@/features/team-analysis/TeamAnalysisPage";
import { HistoryPage } from "@/features/history/HistoryPage";
import { ObjectivesPage } from "@/features/objectives/ObjectivesPage";
import { CoachingPage } from "@/features/coaching/CoachingPage";
import { ComparisonPage } from "@/features/comparison/ComparisonPage";
import { SearchPage } from "@/features/search/SearchPage";
import { SettingsPage } from "@/features/settings/SettingsPage";
import { OverlayPage } from "@/overlay/OverlayPage";

/**
 * `createHashRouter` est utilise plutot qu'un routeur base sur l'historique
 * navigateur : Tauri sert le frontend build via un protocole custom
 * (tauri://, https://tauri.localhost) sans serveur capable de reecrire les
 * routes profondes — le hash routing evite tout 404 au rechargement.
 */
export const router = createHashRouter([
  {
    path: "/overlay",
    element: <OverlayPage />,
  },
  {
    path: "/",
    element: <AppShell />,
    children: [
      { index: true, element: <DashboardPage /> },
      { path: "accounts", element: <AccountsPage /> },
      { path: "profile", element: <ProfilePage /> },
      { path: "champion-select", element: <ChampionSelectPage /> },
      { path: "team-analysis", element: <TeamAnalysisPage /> },
      { path: "history", element: <HistoryPage /> },
      { path: "objectives", element: <ObjectivesPage /> },
      { path: "coaching", element: <CoachingPage /> },
      { path: "comparison", element: <ComparisonPage /> },
      { path: "search", element: <SearchPage /> },
      { path: "settings", element: <SettingsPage /> },
      { path: "*", element: <NotFoundPage /> },
    ],
  },
]);
