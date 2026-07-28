import { clsx } from "clsx";
import type { ButtonHTMLAttributes } from "react";

type Variant = "primary" | "secondary" | "ghost" | "danger";
type Size = "sm" | "md" | "lg";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: Variant;
  size?: Size;
}

const variantClasses: Record<Variant, string> = {
  primary:
    "bg-[var(--color-accent-500)] text-slate-950 hover:bg-[var(--color-accent-400)] shadow-[0_0_0_1px_rgba(255,255,255,0.06)]",
  secondary:
    "bg-[var(--color-surface-2)] text-slate-100 hover:bg-[var(--color-surface-3)] border border-[var(--color-border-subtle)]",
  ghost: "bg-transparent text-slate-300 hover:bg-[var(--color-surface-2)]",
  danger: "bg-[var(--color-loss)]/90 text-white hover:bg-[var(--color-loss)]",
};

const sizeClasses: Record<Size, string> = {
  sm: "text-xs px-2.5 py-1.5 rounded-lg gap-1.5",
  md: "text-sm px-3.5 py-2 rounded-xl gap-2",
  lg: "text-base px-5 py-2.5 rounded-2xl gap-2.5",
};

export function Button({ variant = "primary", size = "md", className, ...props }: ButtonProps) {
  return (
    <button
      className={clsx(
        "inline-flex items-center justify-center font-medium transition-colors duration-150",
        "disabled:opacity-40 disabled:pointer-events-none",
        "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--color-accent-400)]/60",
        variantClasses[variant],
        sizeClasses[size],
        className,
      )}
      {...props}
    />
  );
}
