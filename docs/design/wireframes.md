# Wireframes / low-cost prototype

**Issue:** [#34](https://github.com/rps321321/solpaper/issues/34)  
**Form:** text wireframes (interactive Figma optional later). Sufficient for Alpha 1 IA lock.

## Tray menu (Alpha 1)

```text
┌─────────────────────────────────┐
│ Solpaper · Focus 12:04          │  ← status (non-activating summary)
├─────────────────────────────────┤
│ Start focus                     │
│ Pause                           │
│ Resume                          │
│ Skip phase                      │
│ Reset timer                     │
├─────────────────────────────────┤
│ Edit layout                 F2* │  ← *shows Ctrl+Alt+F2 in status/help
│ Done editing                    │  ← visible only in Edit Mode
├─────────────────────────────────┤
│ Settings…                       │
│ Reset layout…                   │
│ Diagnostics…                    │
├─────────────────────────────────┤
│ Quit Solpaper                   │
└─────────────────────────────────┘
```

Disabled items when illegal for domain state (e.g. Pause when Idle).

## Onboarding (first run)

```text
┌──────── First run — Solpaper ────────┐
│                                      │
│  Local-first desktop surface         │
│  • Runs in your user session         │
│  • No Solpaper cloud account         │
│  • Secrets stay in Windows           │
│    Credential Manager when used      │
│                                      │
│  [ Next ]                            │
└──────────────────────────────────────┘

┌──────── Pomodoro ────────────────────┐
│  A Pomodoro widget will be placed    │
│  on your primary display.            │
│  [ Back ]  [ Next ]                  │
└──────────────────────────────────────┘

┌──────── Wallpaper (optional) ────────┐
│  Local folder for wallpapers         │
│  [ Choose folder… ]  [ Skip ]        │
│  [ Back ]  [ Finish ]                │
└──────────────────────────────────────┘

Tray tip: “Solpaper is running in the system tray.”
```

## Edit Mode — widget chrome

```text
        ┌─ drag strip 24 DIP ─────────────┐
        │ ≡ Pomodoro                      │
   ┌────┴─────────────────────────────────┴────┐
   │                                           │
   │              12:04                        │
   │              Focus                        │
   │                                           │
   │                                    ┌────┐ │
   │                                    │grip│ │  12 DIP
   │                                    └────┘ │
   └───────────────────────────────────────────┘
        clear border (thickness + contrast, not color alone)
```

## Settings — page shell

```text
┌─ Solpaper Settings ──────────────────────────┐
│ General | Widgets | Pomodoro | Wallpaper |   │
│ Calendar | Diagnostics                       │
│                                              │
│  (page body: standard Win32 controls)        │
│                                              │
│              [ Apply ]  [ Close ]            │
└──────────────────────────────────────────────┘
```

### Widgets page

- List widgets; opacity slider; **Reset layout…**; **Add Pomodoro**; Hide/Remove with confirm.

### Pomodoro page

- Durations (validated ranges from #19); auto-start next = off default; no analytics in Alpha 1.

### Wallpaper page

- Folder path; Choose; status of last apply.

### Calendar page (Alpha 2)

- Connect / Disconnect; calendar multi-select; privacy radios: Ordinary titles / Busy-only; last sync; Retry.

### Diagnostics / About

- Version, source SHA if dev, open logs folder, shortcuts list, links to license.

## Pomodoro Normal Mode (no buttons)

```text
┌──────────────────┐
│ 12:04            │  click-through
│ Focus            │
└──────────────────┘
```

## Calendar Normal Mode (Alpha 2)

```text
┌──────────────────┐
│ Agenda           │
│ • 10:00 Private  │  projected strings only
│ • 14:00 Sync     │
│ Updated 12:01    │  or Stale — Retry in settings
└──────────────────┘
```

## Error pattern

```text
┌─ Couldn't apply wallpaper ───────────┐
│ The folder is empty or unreadable.   │
│                                      │
│        [ Choose folder ]  [ Close ]  │
└──────────────────────────────────────┘
```

One primary recovery action always.
