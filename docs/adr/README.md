# Architecture Decision Records

ADRs for Solpaper production architecture. Status values: **Accepted** (owner provisional or explicit), **Provisional** (scaffold may proceed; freeze after named evidence), **Superseded**, **Rejected**.

| ADR | Title | Status |
|-----|-------|--------|
| [0001](0001-desktop-overlay-topology.md) | Desktop overlay / window topology | Accepted (provisional freeze pending manual evidence) |
| [0002](0002-process-model.md) | Process model and UI-thread ownership | Accepted |
| [0003](0003-rendering-path.md) | Rendering path | Accepted (provisional toolkit) |
| [0004](0004-widget-layout-persistence.md) | Widget layout persistence and monitor binding | Accepted |
| [0005](0005-storage-split.md) | Storage split | Accepted |
| [0006](0006-crate-boundaries.md) | Production crate boundaries | Accepted |
| [0007](0007-local-ipc-deferred.md) | Local IPC deferred | Accepted |
| [0008](0008-second-launch-activation.md) | Narrow second-launch activation (not general IPC) | Accepted |

Source spike: [`docs/research/overlay-feasibility.md`](../research/overlay-feasibility.md) (Issue #18).  
Product locks: Issue #17, `AGENTS.md`.  
Scaffold issue: #16.
