# ADR-0001: Desktop overlay / window topology

## Status

**Accepted** for scaffold and Alpha 1 direction (Issue #16).  
Physical freeze deferred until manual evidence debt from #18 is reduced under #33/#24.

## Context

Issue #18 compared:

- **Approach A:** one top-level widget-sized HWND per widget (`WS_POPUP` + layered + toolwindow + noactivate).
- **Approach B:** one monitor work-area-sized HWND containing multiple widgets.

Spike findings (`docs/research/overlay-feasibility.md`):

- Both approaches work on documented Win32 without WorkerW/Progman.
- Idle baseline ~7–8 MB for two sample widgets.
- Approach A matches product language (Widget ≈ window), cheaper invalidation, failure isolation.
- Approach B better for selective holes and dense clusters; larger always-present surface.
- Global alpha proven; per-pixel alpha not required for pass.
- Manual debt remains: sleep/resume, multi-monitor, Explorer restart, Win+D, mixed DPI, prolonged idle.

Owner provisional policy (bootstrap): widget-sized HWND default; monitor-sized surface = fallback not default; WorkerW/Progman never sole/default.

## Decision

1. **Default production topology:** one documented top-level **widget-sized HWND per Widget** (Approach A).
2. **Fallback:** monitor-sized transparent surface (Approach B) remains a validated option if N HWNDs become awkward; not the default.
3. **Never** adopt WorkerW, Progman parenting, or other undocumented Explorer techniques as the sole or default path.
4. Z-order: not permanent topmost; prefer documented bottom/non-topmost policy so normal apps cover Solpaper (re-assert heuristics as needed).
5. Normal Mode: click-through; Edit Mode: interactive hit-testing on the widget HWND.

## Consequences

- Production code lives under `crates/`, not `spikes/desktop-overlay/`.
- Spike may remain as disposable evidence until archived.
- Manual evidence debt stays open; do not claim physical topology tests passed.
- Accessibility feasibility (#41) before freezing final visual toolkit; topology decision does not freeze toolkit.
