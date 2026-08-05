# ADR-0003: Rendering path

## Status

**Accepted** as **provisional** for the initial host (Issue #16).  
Do not freeze a large UI toolkit until accessibility feasibility (#41) and Alpha UX (#34) need it.

## Context

Spike #18 painted with GDI into layered windows and global window alpha (`SetLayeredWindowAttributes`). That was enough for timer/calendar cards at 1 Hz with low CPU.

Owner provisional: smallest native placeholder for initial host; do **not** commit WebView2, wgpu, egui, or a large UI stack in the scaffold. Global opacity OK; per-pixel alpha deferred.

## Decision

1. **Scaffold / Alpha host renderer:** native Win32 + GDI (or equivalent small documented painting) behind `solpaper-windows` adapters.
2. **Opacity:** global per-window alpha initially; per-pixel ARGB / `UpdateLayeredWindow` deferred until a product need is proven.
3. **Out of scope for scaffold:** WebView2, wgpu, egui, iced, tauri, and similar large stacks.
4. **Revisit trigger:** settings surface complexity, accessibility requirements, or non-rectangular chrome that GDI cannot meet honestly.

## Consequences

- Placeholder surfaces in production crates stay intentionally dull.
- Spike paint code is not copied wholesale; production adapters stay minimal.
- Toolkit choice remains reversible without rewriting domain state in `solpaper-core`.
