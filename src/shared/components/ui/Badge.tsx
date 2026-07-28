import { clsx } from "clsx";
import type { HTMLAttributes } from "react";

type Tone = "neutral" | "win" | "loss" | "accent" | "warning";

interface BadgeProps extends HTMLAttributes<HTMLSpanElement> {
  tone?: Tone;
}

const toneClasses: Record<Tone, string> = {
  neutral: "bg-[var(--color-surface-3)] text-slate-200",
  win: "bg-[var(--color-win)]/15 text-[var(--color-win)]",
  loss: "bg-[var(--color-loss)]/15 text-[var(--color-loss)]",
  accent: "bg-[var(--color-accent-500)]/15 text-[var(--color-accent-400)]",
  warning: "bg-amber-500/15 text-amber-400",
};

export function Badge({ tone = "neutral", className, ...props }: BadgeProps) {
  return (
    <span
      className={clsx(
        "inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium",
        toneClasses[tone],
        className,
      )}
      {...props}
    />
  );
}
