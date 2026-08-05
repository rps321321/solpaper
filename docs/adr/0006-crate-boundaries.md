# ADR-0006: Production crate boundaries

## Status

**Accepted** (Issue #16).

## Context

Issue #16 forbids a crate per future feature without evidence. Spike was a single disposable package under `spikes/`.

Owner provisional max start: `solpaper-app`, `solpaper-core`, `solpaper-windows`, `solpaper-storage`; features as modules; split later only with evidence.

## Decision

```text
crates/
├── solpaper-app       # binary / composition root
├── solpaper-core      # platform-neutral domain (layout types, future Pomodoro SM)
├── solpaper-windows   # Win32 / COM / tray / overlay adapters (unsafe boundaries)
└── solpaper-storage   # paths, settings, future DB migrations
```

Workspace root `Cargo.toml` members = these four.  
`spikes/desktop-overlay/` remains **outside** the production workspace (disposable).

Pomodoro, calendar, wallpaper begin as **modules** inside these crates. Split only when independent testing, platform isolation, or compile-time boundaries justify it.

## Consequences

- Platform-neutral unit tests live in `solpaper-core` without linking Win32.
- All `unsafe` Win32 encapsulation targets `solpaper-windows`.
- Dependency on `windows` crate is confined to `solpaper-windows` (+ app only if composition requires it; prefer not).
