For my rust ai cli tui please give me most professional and beautiful symbols as these symbols are boring and also make sure the symobls you give must show correctly and professional and beautiful!!!
step_active = ">"
step_cancel = "x"
step_error = "!"
step_submit = ">"

# Selection indicators
radio_active = "(*)"
radio_inactive = "( )"
checkbox_active = "[ ]"
checkbox_selected = "[x]"
checkbox_inactive = "[ ]"

# Password masking character
password_mask = "*"

# UI symbols (no emojis)
checkmark = "√"
info = "i"
arrow_right = ">"

# Slider symbols
slider_filled = "="
slider_empty = "-"
slider_handle = "O"

# ✨ Professional Unicode Symbols for Your Rust CLI TUI

Here's a carefully curated set of **modern, elegant, terminal-safe** Unicode symbols:

```toml
# ── Step Indicators ──────────────────────────────────
step_active  = "▸"    # Right-pointing small triangle (active/current)
step_cancel  = "✖"    # Bold multiplication X (cancelled)
step_error   = "▲"    # Warning triangle feel (error)
step_submit  = "✔"    # Heavy checkmark (submitted/done)

# ── Selection Indicators ─────────────────────────────
radio_active     = "◉"    # Filled circle target (selected radio)
radio_inactive   = "○"    # Empty circle (unselected radio)
checkbox_active  = "◻"    # Empty square (focused but unchecked)
checkbox_selected= "◼"    # Filled square (checked)
checkbox_inactive= "◻"    # Empty square (unfocused unchecked)

# ── Password ─────────────────────────────────────────
password_mask = "●"    # Bullet dot (industry standard)

# ── UI Symbols ───────────────────────────────────────
checkmark   = "✔"    # Heavy checkmark
info        = "ℹ"    # Information source symbol
arrow_right = "▸"    # Right-pointing triangle

# ── Slider Symbols ───────────────────────────────────
slider_filled = "━"    # Heavy horizontal line (filled track)
slider_empty  = "─"    # Light horizontal line (empty track)
slider_handle = "◉"    # Fisheye / bullseye (handle)
```

---

### 🔍 Live Preview in Terminal

```
  Step Flow:
     ✔ Environment Setup
     ✔ Dependencies Installed
     ▸ Building Project          ← active
     ○ Deploy

  Radio:
     ◉ GPT-4o
     ○ Claude 3.5
     ○ LLaMA 3

  Checkbox:
     ◼ Enable streaming
     ◼ Verbose logging
     ◻ Dry run

  Password:
     Enter API Key: ●●●●●●●●●●●●

  Slider:
     Temperature: ━━━━━━━◉───── 0.7

  Info:
     ✔ Model loaded successfully
     ℹ Using default config
     ✖ Connection cancelled
     ▲ Rate limit approaching
```

---

### 🎯 Why These Specific Symbols?

| Symbol | Codepoint | Why It's Better |
|--------|-----------|-----------------|
| `▸` | U+25B8 | Cleaner than `>`, universal terminal support |
| `✖` | U+2716 | Visually distinct "cancel", not confused with `x` |
| `✔` | U+2714 | Industry-standard checkmark, not the ugly `√` |
| `◉` / `○` | U+25C9/U+25CB | Perfect radio button metaphor, immediately recognizable |
| `◼` / `◻` | U+25FC/U+25FB | Clean square checkbox, modern flat-UI feel |
| `●` | U+25CF | Standard password mask (used by macOS, Linux, web) |
| `ℹ` | U+2139 | Actual info symbol, not just a letter `i` |
| `━` / `─` | U+2501/U+2500 | Box-drawing chars = **pixel-perfect alignment** |
| `◉` | U+25C9 | Bullseye handle stands out clearly on the track |

> **Terminal Compatibility**: All symbols above are from the **Basic Multilingual Plane (BMP)** and render correctly in virtually every modern terminal — **Alacritty, Kitty, iTerm2, Windows Terminal, WezTerm, and GNOME Terminal**.

# Professional & Beautiful TUI Symbols

Here's a refined symbol set using **Unicode box-drawing, geometric, and standard terminal-safe characters** that render correctly in virtually all modern terminals:

```toml
# ─── Step Indicators ─────────────────────────────────────────
step_active    = "▶"    # Solid right-pointing triangle
step_cancel    = "✕"    # Clean multiplication X
step_error     = "✖"    # Heavy X mark
step_submit    = "◆"    # Solid diamond

# ─── Selection Indicators ────────────────────────────────────
radio_active   = "◉"    # Filled circle with dot (selected)
radio_inactive = "○"    # Empty circle

checkbox_active   = "☐"    # Empty ballot box (focused/active)
checkbox_selected = "✔"    # Check mark (selected)  
checkbox_inactive = "■"    # Or use "☐" for consistent look

# ─── Password ────────────────────────────────────────────────
password_mask = "●"     # Solid bullet

# ─── UI Symbols ──────────────────────────────────────────────
checkmark   = "✔"      # Clean check mark
info        = "ℹ"      # Information source symbol
arrow_right = "▸"      # Small right-pointing triangle

# ─── Slider ──────────────────────────────────────────────────
slider_filled = "━"    # Heavy horizontal line
slider_empty  = "─"    # Light horizontal line
slider_handle = "◈"    # Diamond with dot inside
```

## Visual Preview (how it looks in terminal)

```
  ◉ Model: GPT-4o
  ○ Model: Claude Sonnet
  ○ Model: Gemini Pro

  ✔ Enable streaming
  ☐ Save to file
  ☐ Verbose logging

  Password: ●●●●●●●●●●

  ▶ Initializing model...
  ✔ Connected successfully
  ✖ Rate limit exceeded
  ◆ Response submitted

  Temperature ━━━━━━━◈──────── 0.7

  ℹ Press Ctrl+C to exit
  ▸ Continue
```

## Alternative Premium Options

```toml
# If you want an even cleaner look, swap these:
step_active    = "›"     # Single right-pointing angle quotation
step_error     = "◇"     # Open diamond (softer error)
arrow_right    = "→"     # Standard arrow
checkmark      = "✓"     # Lighter check (thinner stroke)
info           = "◌"     # Dotted circle (minimal)

# Braille-style slider (ultra-clean)
slider_filled  = "⣿"
slider_empty   = "⡀"
slider_handle  = "⠿"

# Box-drawing spinner frames (bonus)
spinner_frames = ["┤", "┘", "┴", "└", "├", "┌", "┬", "┐"]
```

## Why These Work

| Concern | Solution |
|---|---|
| **Terminal compatibility** | All characters are in Unicode BMP (Basic Multilingual Plane) — no emoji |
| **Monospace alignment** | Every symbol is single-width in monospace fonts |
| **Visual distinction** | `◉` vs `○`, `✔` vs `☐` — instantly recognizable states |
| **Professional tone** | Geometric shapes feel clean and modern, no childish icons |

> **Tip:** Test with `echo "◉ ○ ✔ ☐ ● ▶ ✕ ━ ─ ◈"` in your target terminal before shipping. If any glyph shows as `□`, the user's font lacks coverage — consider bundling a **Nerd Font** recommendation in your README.
