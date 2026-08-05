# Wayfinder home (in-repo)

This effort uses **two homes** so nothing lives only in a local checkout:

| Role | Location |
|------|----------|
| **Canonical tracker** | GitHub Issues — map label `wayfinder:map`, tickets `wayfinder:research` / `grilling` / `prototype` / `task` |
| **In-repo mirror** | This directory (`docs/wayfinder/`) — committed and pushed so the map is readable on `main` |

## Rules

1. **Issues own workflow** — claim, block, resolve, assignee, and comments happen on GitHub Issues.
2. **Repo owns durable narrative** — when a ticket resolves, update:
   - the issue (Answer + close),
   - [`map.md`](map.md) Decisions so far,
   - research assets under [`docs/research/`](../research/) when applicable,
   - then **commit and push** so GH `main` matches.
3. Prefer ticket **titles + links** over bare `#N` in prose.

## Map issue

https://github.com/rps321321/solpaper/issues/1

## Files

- [`map.md`](map.md) — destination, notes, decisions, fog, out of scope
- [`tickets.md`](tickets.md) — child ticket index with links
