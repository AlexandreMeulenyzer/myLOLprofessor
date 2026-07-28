import type { ReactNode } from "react";

import { Card } from "@/shared/components/ui/Card";

interface FeaturePlaceholderProps {
  title: string;
  description: string;
  epic: string;
  children?: ReactNode;
}

/**
 * Utilise par les pages dont l'implementation complete arrive dans une tache
 * ulterieure de la roadmap (voir docs/ROADMAP.md). Remplace au fur et a
 * mesure par le contenu reel de chaque feature.
 */
export function FeaturePlaceholder({
  title,
  description,
  epic,
  children,
}: FeaturePlaceholderProps) {
  return (
    <div className="mx-auto flex max-w-2xl flex-col items-start gap-4 py-16">
      <span className="text-xs font-medium uppercase tracking-wider text-[var(--color-accent-400)]">
        {epic}
      </span>
      <h1 className="text-2xl font-semibold text-slate-100">{title}</h1>
      <p className="text-sm leading-relaxed text-slate-400">{description}</p>
      {children && (
        <Card className="w-full" glass>
          {children}
        </Card>
      )}
    </div>
  );
}
