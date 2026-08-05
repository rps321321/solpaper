# Issue leases

Atomic issue leases for autonomous agents. Schema and rules: [`docs/engineering/agent-governance.md`](../../docs/engineering/agent-governance.md).

```powershell
powershell -NoProfile -File scripts/agent-lease.ps1 claim -Issue N -Owner 'agent:…' -Branch 'issue-N-…' -Unit '…' -RiskClass LOW
powershell -NoProfile -File scripts/agent-lease.ps1 heartbeat -Issue N -Owner 'agent:…'
powershell -NoProfile -File scripts/agent-lease.ps1 release -Issue N -Owner 'agent:…'
powershell -NoProfile -File scripts/agent-lease.ps1 status -Issue N
```

`DEV_STATE.md` mirrors the active lease but is not authoritative for conflict resolution.
