# Domain glossary — solpaper

Terms only. No implementation detail.

Product destination locked by Issue #17 (2026-08-05). Overlay spike (#18) recommends independent widget HWNDs (Approach A); renderer and Cargo crate boundaries remain provisional until Issue #16’s ADR + scaffold.

## Runtime

The user-session process that owns desktop widget surfaces, productivity state (e.g. Pomodoro), tray and settings interaction, and the wallpaper subsystem. Not a Windows SCM service. Continues while settings UI is closed.

## Surface

A desktop-hosted visual region Solpaper manages. Spike #18 recommends widget-sized top-level windows (Approach A); production topology is confirmed in #16. Surfaces host Widgets; they are UI, never baked into wallpaper image files.

## Widget

A self-contained productivity or information card rendered on a Surface (e.g. Pomodoro timer, Calendar agenda). Has layout, opacity, and Normal vs Edit Mode behaviour.

## Edit Mode

Interaction mode where the user can drag, resize, and arrange Widgets. Distinct from Normal Mode, where Widgets are largely passive and desktop/app input remains usable.

## Widget Layout

Persisted arrangement of Widgets: positions, sizes, opacity, monitor binding, and related placement state. Restored after process restart.

## Monitor Binding

Association of a Widget (or Surface) to a specific display identity so layout survives topology changes where possible.

## Pomodoro Session

A timed focus/break cycle owned by the Runtime. Required for Alpha 1 and v1. State and recovery semantics are designed in #19; rendered as a live Widget, not as wallpaper pixels.

## Calendar Projection

Read-only view of the user’s Google Calendar agenda for display as a Widget. Alpha 2 scope; intended for v1. Never writes to Calendar. Privacy: default shows ordinary event titles and replaces private details with `Private`; a Busy-only mode must also exist.

## Wallpaper Provider

A source of wallpaper images. Local folders are the first Provider. At most one remote Provider may enter v1. Apply path is a peer subsystem of the Runtime, not the product root.

## Wallpaper subsystem

Peer responsibility of the Runtime: select and apply wallpapers (via documented APIs such as `IDesktopWallpaper` after research). Distinct from Surfaces and Widgets; live content is never composited into wallpaper files.

## Tray

Primary always-available entry point for the Runtime: status, quick actions, and access to settings / Edit Mode. Part of the v1 interaction model with direct Edit Mode and a visual settings surface.

## Settings surface

Visual configuration UI (not a TUI as primary control plane). Exact toolkit TBD after architecture is known.

## TUI

Terminal UI. Explicitly **not** the primary v1 interface. May appear post-v1 for diagnostics or power-user commands only.

## Credential

A secret (API key, OAuth refresh token) stored outside plaintext config—Windows Credential Manager or equivalent after #6. Never committed or logged.
