import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { listen as tauriListen, type EventCallback, type UnlistenFn } from "@tauri-apps/api/event";

/**
 * Point d'entree unique vers le pont IPC Tauri. Toutes les commandes/evenements
 * typed par feature (voir src/features, fichiers api.ts) passent par ce module
 * afin de garder un seul endroit a adapter si le pont change.
 */

export function isTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauriRuntime()) {
    throw new Error(
      `Commande Tauri "${command}" appelee hors du runtime desktop (mode navigateur/dev web).`,
    );
  }
  return tauriInvoke<T>(command, args);
}

export async function listen<T>(event: string, handler: EventCallback<T>): Promise<UnlistenFn> {
  if (!isTauriRuntime()) {
    return () => {};
  }
  return tauriListen<T>(event, handler);
}
