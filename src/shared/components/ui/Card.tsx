import { clsx } from "clsx";
import type { HTMLAttributes, ReactNode } from "react";

interface CardProps extends Omit<HTMLAttributes<HTMLDivElement>, "title"> {
  glass?: boolean;
  heading?: ReactNode;
  actions?: ReactNode;
}

export function Card({
  glass = false,
  heading,
  actions,
  className,
  children,
  ...props
}: CardProps) {
  return (
    <div
      className={clsx(
        "rounded-2xl border border-[var(--color-border-subtle)] p-4",
        glass ? "glass-panel" : "bg-[var(--color-surface-1)]",
        className,
      )}
      {...props}
    >
      {(heading || actions) && (
        <div className="mb-3 flex items-center justify-between">
          {heading && <h3 className="text-sm font-semibold text-slate-200">{heading}</h3>}
          {actions}
        </div>
      )}
      {children}
    </div>
  );
}
