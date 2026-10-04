# Toolbox Slint Design Tokens

This reference defines the design tokens used across all Slint markup in `crates/tb-ui/`.

---

## Typography

All text in Toolbox uses the pure monospace font stack:

```slint
export global Theme {
    in property <string> font-family: "ui-monospace, \"JetBrains Mono\", \"Fira Code\", monospace";
    in property <length> font-size-h1: 18px;
    in property <length> font-size-h2: 14px;
    in property <length> font-size-body: 13px;
    in property <length> font-size-code: 12px;
    in property <length> font-size-caption: 11px;
}
```

---

## Radii & Spacing

Boxy brutalism enforces absolute 0px border-radius everywhere:

```slint
export global Spacing {
    in property <length> radius: 0px; // STRICT 0px everywhere!

    in property <length> space-1: 4px;
    in property <length> space-2: 8px;
    in property <length> space-3: 12px;
    in property <length> space-4: 16px;
    in property <length> space-6: 24px;
    in property <length> space-8: 32px;
    in property <length> space-12: 48px;
}
```

---

## Component Style Tokens

- **Card**:
  - `background`: `Theme.surface`
  - `border-width`: `1px`
  - `border-color`: `Theme.border-subtle`
  - `border-radius`: `0px`

- **Dropzone**:
  - `background`: `Theme.surface`
  - `border-width`: `1px`
  - `border-style`: `dashed`
  - `border-color`: `Theme.border-prominent`
  - `hover border-color`: `Theme.border-focus` (`#FFFFFF`)

- **Primary Button**:
  - `background`: `Theme.accent-primary` (`#FFFFFF`)
  - `text-color`: `Theme.accent-foreground` (`#141618`)
  - `border-radius`: `0px`

- **Secondary Button**:
  - `background`: `transparent`
  - `text-color`: `Theme.text-primary` (`#E2E8F0`)
  - `border-width`: `1px`
  - `border-color`: `Theme.border-prominent` (`#32383E`)
  - `border-radius`: `0px`

- **Preset Pill**:
  - Unselected: `background: transparent`, `border: 1px solid Theme.border-subtle`, `text: Theme.text-secondary`
  - Selected: `background: Theme.accent-primary`, `border: 1px solid Theme.accent-primary`, `text: Theme.accent-foreground`
