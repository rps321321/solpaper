# Domain glossary — solpaper

Terms only. No implementation detail.

## Agent

The user-session background process that owns the wallpaper cycle: fetch, cache, apply, schedule. Lives in the tray. Continues while the TUI is closed.

## TUI

Terminal control plane. Configures the Agent, shows status, does not need to stay open for cycling.

## Source

An internet (or later local) provider of wallpaper images. v1 sources: Wallhaven, Bing, Unsplash.

## Priority fallback

When fetching, try Sources in order (default Wallhaven → Bing → Unsplash). On failure or empty result, try the next. Skip Unsplash if no API key is configured.

## Cycle

One scheduled tick: obtain an image (or images) and apply wallpaper(s). Driven by the Schedule.

## Schedule

When Cycles fire. Represented as a cron expression; friendly presets write that expression.

## Cache

On-disk store of previously fetched images under the user's local app data. Used for apply paths and when all Sources fail.

## Monitor surface

One physical or logical display the Agent can target with `IDesktopWallpaper`. v1 applies a different image per Monitor surface.

## Credential

A secret API key stored in Windows Credential Manager (e.g. Unsplash; optional Wallhaven).

## Skip now

User action that advances wallpaper immediately, outside waiting for the next Schedule fire.
