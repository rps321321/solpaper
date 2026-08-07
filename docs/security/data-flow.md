# Data-flow and trust-boundary diagrams

**Issue:** [#36](https://github.com/rps321321/solpaper/issues/36)  
**Companion:** [`threat-model.md`](./threat-model.md)

## System context

```mermaid
flowchart TB
  subgraph Untrusted["Untrusted external"]
    Google["Google OAuth + Calendar API"]
    Provider["Optional remote wallpaper provider"]
    Browser["System browser"]
    Release["Release / download channel"]
  end

  subgraph OS["Windows 11 user session"]
    CM["Credential Manager\n(TB2 secrets)"]
    FS["LocalAppData + user folders\n(TB5)"]
    Shell["Win32 / COM adapters\n(TB3 unsafe)"]
    Runtime["Solpaper Runtime\ntray · domain · workers"]
  end

  Browser -->|"auth code to loopback 127.0.0.1\n(TB6)"| Runtime
  Runtime -->|"open auth URL"| Browser
  Runtime -->|"HTTPS token + Calendar JSON\n(TB1)"| Google
  Runtime -->|"HTTPS image/metadata\n(TB1)"| Provider
  Runtime <-->|"refresh token only"| CM
  Runtime <-->|"settings layout cache logs"| FS
  Runtime --> Shell
  Release -.->|"signed binary / checksums\n(TB7 no updater v1)"| Runtime
```

## OAuth connect (F1)

```mermaid
sequenceDiagram
  participant U as User
  participant R as Runtime
  participant L as Loopback listener<br/>127.0.0.1:ephemeral
  participant B as System browser
  participant G as Google token endpoint

  U->>R: Connect Calendar (Settings)
  R->>L: Bind TcpListener before browser
  R->>R: Generate PKCE S256 + state
  R->>B: Open authorize URL (no embedded webview)
  B->>G: User consents
  G->>B: Redirect to http://127.0.0.1:port/oauth/callback
  B->>L: GET callback (code + state)
  L->>R: First valid callback only
  Note over R: Reject state mismatch, wrong path,<br/>oversized headers, timeout 120s
  R->>G: Code exchange + PKCE verifier (HTTPS)
  G->>R: refresh + access tokens
  R->>R: Access token memory only
  R->>R: Refresh token → Credential Manager
  Note over R: Never log URL/query/code/state/verifier/tokens
```

## Calendar sync (F2)

```mermaid
flowchart LR
  API["Google Calendar API HTTPS"] -->|"JSON pages"| Worker["Calendar worker"]
  Worker -->|"validate bounds"| Norm["Normalize AgendaItem"]
  Norm -->|"TB4 privacy projection"| Proj["Projected titles"]
  Proj --> Store["Transactional cache + syncToken"]
  Proj --> UI["Widget / tray / UIA"]
  Proj --> Log["Logs allowlist only"]
  Store -->|"failure isolation"| Tray["Tray/Pomodoro unaffected"]
```

## Wallpaper apply (F3 / F4)

```mermaid
flowchart TB
  Local["User folder paths"] --> Canon["Canonicalize + enumerate"]
  Remote["Provider HTTPS"] --> Bounds["HTTPS + redirect + size limits"]
  Bounds --> Cache["Cache file = generated ID"]
  Canon --> Decode
  Cache --> Decode["Compressed + pixel limits"]
  Decode -->|"success"| Apply["Desktop wallpaper API"]
  Decode -->|"failure"| Keep["Keep current wallpaper\ntyped error no retry loop"]
```

## Secrets and non-secrets placement

| Data | Credential Manager | Settings file | SQLite/runtime | Memory | Logs (default) |
|------|:-----------------:|:-------------:|:--------------:|:------:|:--------------:|
| Refresh token | Yes | No | No | Load only | No |
| Access token | No | No | No | Yes | No |
| PKCE/state/code | No | No | No | Connect only | No |
| Event titles | No | No | Yes (cache) | Yes | No (raw) |
| Projected title | No | No | Optional | Yes | Prefer codes only |
| Layout / settings | No | Yes | Optional | Yes | Redacted |
| Wallpaper cache path IDs | No | No | Optional | Yes | IDs only |

## Boundary checklist for implementers

1. Crossing **TB1**: use the single shared HTTPS client config; no second HTTP stack in Alpha 2.
2. Crossing **TB2**: only through `CredentialStore` trait; test fakes in-memory.
3. Crossing **TB3**: no `unsafe` outside `solpaper-windows` (or documented adapter module).
4. Crossing **TB4**: projection function is the only path to UI/UIA/notifications/export.
5. Crossing **TB5**: path helpers canonicalize; atomic write pattern for settings.
6. Opening **TB8 IPC**: blocked until ADR + threat-model update.
