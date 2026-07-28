import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";

import { isTauriRuntime } from "@/shared/lib/tauri-bridge";
import { useNotificationSettingsStore } from "@/shared/stores/notification-settings-store";

export type NotificationCategory = keyof Omit<
  ReturnType<typeof useNotificationSettingsStore.getState>,
  "toggle"
>;

let permissionPromise: Promise<boolean> | null = null;

async function ensurePermission(): Promise<boolean> {
  if (!isTauriRuntime()) return false;
  permissionPromise ??= (async () => {
    if (await isPermissionGranted()) return true;
    return (await requestPermission()) === "granted";
  })();
  return permissionPromise;
}

/** Envoie une notification OS native si la categorie est activee dans les
 * parametres et que la permission systeme est accordee. Best-effort : ne
 * doit jamais faire echouer l'appelant. */
export async function notify(
  category: NotificationCategory,
  title: string,
  body?: string,
): Promise<void> {
  if (!useNotificationSettingsStore.getState()[category]) return;
  if (!(await ensurePermission())) return;
  sendNotification({ title, body });
}
