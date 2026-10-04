# Omarchy Theme Registry for Slint

Toolbox supports 10 themes inspired by Omarchy. In Slint, the active theme updates global properties dynamically.

| # | Theme Name | Canvas | Surface | Border | Text | Accent | Character / Register |
|---|---|---|---|---|---|---|---|
| 1 | **Industrial Graphite** (Default) | `#141618` | `#1E2124` | `#32383E` | `#E2E8F0` | `#FFFFFF` | Precision cold steel, high technical affordance |
| 2 | **Vantablack / Matte** | `#000000` | `#0D0D0D` | `#262626` | `#FAFAFA` | `#FFFFFF` | Teenage Engineering pure stark monochrome |
| 3 | **Tokyo Night** | `#1A1B26` | `#24283B` | `#414868` | `#C0CAF5` | `#7AA2F7` | Deep midnight blue with calm indigo accents |
| 4 | **Nord** | `#2E3440` | `#3B4252` | `#4C566A` | `#ECEFF4` | `#88C0D0` | Arctic frost slate with ice-blue accents |
| 5 | **Gruvbox Dark** | `#1D2021` | `#282828` | `#3C3836` | `#EBDBB2` | `#FE8019` | Warm retro terminal amber/orange |
| 6 | **Hackerman** | `#080C08` | `#0E160E` | `#1A2B1A` | `#00FF66` | `#33FF77` | High-contrast phosphorus green CRT |
| 7 | **Kanagawa** | `#1F1F28` | `#2A2A37` | `#363646` | `#DCD7BA` | `#7E9CD8` | Traditional Japanese ink-wash & autumnal tone |
| 8 | **Catppuccin Mocha** | `#1E1E2E` | `#181825` | `#313244` | `#CDD6F4` | `#CBA6F7` | Soft soothing pastel lavender & dark navy |
| 9 | **Rose Pine** | `#191724` | `#1F1D2E` | `#26233A` | `#E0DEF4` | `#EBBCBA` | Moody pine needle with dusty rose accent |
| 10 | **Everforest** | `#272E33` | `#2D353B` | `#414B50` | `#D3C6AA` | `#A7C080` | Organic earthy sage green & olive grey |

---

## Slint Global Theme Definition Pattern

```slint
export struct Palette {
    canvas: color,
    surface: color,
    surface_hover: color,
    surface_sunken: color,
    border_subtle: color,
    border_prominent: color,
    border_focus: color,
    text_primary: color,
    text_secondary: color,
    accent_primary: color,
    accent_foreground: color,
}

export global Theme {
    in-out property <Palette> active: {
        canvas: #141618,
        surface: #1E2124,
        surface_hover: #262A2E,
        surface_sunken: #0E1012,
        border_subtle: #282C32,
        border_prominent: #32383E,
        border_focus: #FFFFFF,
        text_primary: #E2E8F0,
        text_secondary: #94A3B8,
        accent_primary: #FFFFFF,
        accent_foreground: #141618,
    };

    public pure function set_theme(id: int) {
        // Rust or Slint sets active Palette based on selection index
    }
}
```
