import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useEffect, useState, type ReactNode } from "react";

import { useAccountSyncListener } from "@/shared/hooks/useAccountSyncListener";
import { useThemeStore } from "@/shared/stores/theme-store";

function makeQueryClient(): QueryClient {
  return new QueryClient({
    defaultOptions: {
      queries: {
        staleTime: 30_000,
        retry: 2,
        refetchOnWindowFocus: false,
      },
    },
  });
}

function ThemeEffect() {
  const theme = useThemeStore((state) => state.theme);
  const accent = useThemeStore((state) => state.accent);

  useEffect(() => {
    document.documentElement.dataset.theme = theme;
    document.documentElement.dataset.accent = accent;
  }, [theme, accent]);

  return null;
}

function AccountSyncEffect() {
  useAccountSyncListener();
  return null;
}

export function AppProviders({ children }: { children: ReactNode }) {
  const [queryClient] = useState(makeQueryClient);

  return (
    <QueryClientProvider client={queryClient}>
      <ThemeEffect />
      <AccountSyncEffect />
      {children}
    </QueryClientProvider>
  );
}
