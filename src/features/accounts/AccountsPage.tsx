import { useState } from "react";

import { Badge } from "@/shared/components/ui/Badge";
import { Button } from "@/shared/components/ui/Button";
import { Card } from "@/shared/components/ui/Card";
import { useActiveAccountStore } from "@/shared/stores/active-account-store";

import {
  useAccounts,
  useDeleteRiotApiKey,
  useHasRiotApiKey,
  useLinkAccount,
  useRemoveAccount,
  useSaveRiotApiKey,
  useSetPrimaryAccount,
  useSyncActiveAccount,
} from "./hooks";
import { PLATFORMS } from "./types";

function ApiKeyStep() {
  const [apiKey, setApiKey] = useState("");
  const saveKey = useSaveRiotApiKey();

  return (
    <Card heading="Clé API Riot Games" glass className="w-full">
      <p className="mb-3 text-sm text-slate-400">
        Créez une clé personnelle gratuite sur{" "}
        <span className="text-slate-200">developer.riotgames.com</span>, puis collez-la ci-dessous.
        Elle est chiffrée dans le trousseau sécurisé de votre système et n'est jamais envoyée à un
        serveur tiers.
      </p>
      <form
        className="flex gap-2"
        onSubmit={(event) => {
          event.preventDefault();
          if (apiKey.trim()) saveKey.mutate(apiKey.trim());
        }}
      >
        <input
          type="password"
          value={apiKey}
          onChange={(event) => setApiKey(event.target.value)}
          placeholder="RGAPI-xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
          className="flex-1 rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-3 py-2 text-sm text-slate-100 outline-none focus:border-[var(--color-accent-500)]"
        />
        <Button type="submit" disabled={saveKey.isPending || !apiKey.trim()}>
          {saveKey.isPending ? "Enregistrement…" : "Enregistrer"}
        </Button>
      </form>
      {saveKey.isError && (
        <p className="mt-2 text-xs text-[var(--color-loss)]">
          {saveKey.error instanceof Error ? saveKey.error.message : String(saveKey.error)}
        </p>
      )}
    </Card>
  );
}

function LinkAccountForm() {
  const [gameName, setGameName] = useState("");
  const [tagLine, setTagLine] = useState("");
  const [platform, setPlatform] = useState(PLATFORMS[0]?.value ?? "euw1");
  const linkAccount = useLinkAccount();

  return (
    <Card heading="Lier un compte Riot" glass className="w-full">
      <form
        className="flex flex-col gap-3"
        onSubmit={(event) => {
          event.preventDefault();
          if (!gameName.trim() || !tagLine.trim()) return;
          linkAccount.mutate(
            { gameName: gameName.trim(), tagLine: tagLine.trim(), platform },
            {
              onSuccess: () => {
                setGameName("");
                setTagLine("");
              },
            },
          );
        }}
      >
        <div className="flex gap-2">
          <input
            value={gameName}
            onChange={(event) => setGameName(event.target.value)}
            placeholder="Nom d'invocateur"
            className="flex-1 rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-3 py-2 text-sm text-slate-100 outline-none focus:border-[var(--color-accent-500)]"
          />
          <span className="flex items-center text-slate-500">#</span>
          <input
            value={tagLine}
            onChange={(event) => setTagLine(event.target.value)}
            placeholder="TAG"
            className="w-24 rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-3 py-2 text-sm text-slate-100 outline-none focus:border-[var(--color-accent-500)]"
          />
        </div>
        <select
          value={platform}
          onChange={(event) => setPlatform(event.target.value)}
          className="rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-3 py-2 text-sm text-slate-100 outline-none focus:border-[var(--color-accent-500)]"
        >
          {PLATFORMS.map((p) => (
            <option key={p.value} value={p.value}>
              {p.label}
            </option>
          ))}
        </select>
        <Button type="submit" disabled={linkAccount.isPending}>
          {linkAccount.isPending ? "Vérification…" : "Lier ce compte"}
        </Button>
        {linkAccount.isError && (
          <p className="text-xs text-[var(--color-loss)]">
            {linkAccount.error instanceof Error
              ? linkAccount.error.message
              : String(linkAccount.error)}
          </p>
        )}
      </form>
    </Card>
  );
}

export function AccountsPage() {
  const { data: hasApiKey, isLoading: loadingKeyStatus } = useHasRiotApiKey();
  const { data: accounts } = useAccounts();
  const activeAccount = useActiveAccountStore((state) => state.activeAccount);
  const setActiveAccount = useActiveAccountStore((state) => state.setActiveAccount);
  const removeAccount = useRemoveAccount();
  const setPrimary = useSetPrimaryAccount();
  const deleteKey = useDeleteRiotApiKey();
  useSyncActiveAccount(accounts);

  return (
    <div className="mx-auto flex max-w-2xl flex-col gap-6 py-8">
      <div>
        <h1 className="text-2xl font-semibold text-slate-100">Comptes Riot</h1>
        <p className="mt-1 text-sm text-slate-400">
          Wardstone lit vos statistiques directement via l'API officielle Riot Games — aucune donnée
          n'est envoyée à un service tiers.
        </p>
      </div>

      {!loadingKeyStatus && !hasApiKey && <ApiKeyStep />}
      {hasApiKey && <LinkAccountForm />}

      {accounts && accounts.length > 0 && (
        <Card heading="Comptes liés" glass className="w-full">
          <ul className="flex flex-col gap-2">
            {accounts.map((account) => (
              <li
                key={account.puuid}
                className="flex items-center justify-between rounded-xl border border-[var(--color-border-subtle)] px-3 py-2"
              >
                <div className="flex items-center gap-2">
                  <span className="text-sm text-slate-100">
                    {account.gameName}
                    <span className="text-slate-500">#{account.tagLine}</span>
                  </span>
                  <Badge tone="neutral">{account.platform.toUpperCase()}</Badge>
                  {account.isPrimary && <Badge tone="accent">Principal</Badge>}
                  {activeAccount?.puuid === account.puuid && <Badge tone="win">Actif</Badge>}
                </div>
                <div className="flex gap-1.5">
                  {activeAccount?.puuid !== account.puuid && (
                    <Button size="sm" variant="ghost" onClick={() => setActiveAccount(account)}>
                      Afficher
                    </Button>
                  )}
                  {!account.isPrimary && (
                    <Button
                      size="sm"
                      variant="ghost"
                      onClick={() => setPrimary.mutate(account.puuid)}
                    >
                      Définir principal
                    </Button>
                  )}
                  <Button
                    size="sm"
                    variant="danger"
                    onClick={() => removeAccount.mutate(account.puuid)}
                  >
                    Retirer
                  </Button>
                </div>
              </li>
            ))}
          </ul>
        </Card>
      )}

      {hasApiKey && (
        <Button variant="ghost" size="sm" className="self-start" onClick={() => deleteKey.mutate()}>
          Supprimer la clé API du trousseau
        </Button>
      )}
    </div>
  );
}
