# Circuit Breakers & Runtime Overrides

## Overview

The frontend contract test system includes a lightweight client-side circuit breaker per spec to avoid spamming failing endpoints and to surface instability quickly.

## Circuit Breaker Mechanics

- Configured per spec via `circuitBreaker: { failureThreshold, cooldownMs }`.
- Consecutive `fail` or `error` results increment the streak.
- When streak >= threshold and breaker not already open, it opens (timestamp recorded).
- While open and within `cooldownMs`, future runs return a synthetic result:
  - `status: 'skip'`, `message: 'circuit-open'` (does NOT increment failure counters).
- A successful (`pass`) result resets `streak` and closes the breaker.

## UI Badges

Within the Contract Test panel:

- Per spec badge: `CB<streak>`
  - Gray: streak = 0 (closed / idle)
  - Yellow: streak > 0 but not open
  - Rose: open (cooldown active)
- Aggregate badges:
  - `CB Open` — count of open breakers
  - `Max Streak` — highest current failure streak across specs

Tooltips clarify whether breaker is open vs accumulating streak.

## Prometheus Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `fks_contract_open_circuits` | gauge | `service` | Number of currently open breakers for a service |
| `fks_contract_circuit_open` | gauge | `service`, `spec` | 1=open, 0=closed |
| `fks_contract_circuit_failure_streak` | gauge | `service`, `spec` | Current consecutive failure count |

These update after each contract test execution pass.

## Runtime Overrides

Some spec properties (latency budgets, enabling/disabling) can be modified at runtime via the UI editor:

- Changes are held in React state (`runtimeSpecs`) and applied immediately to subsequent runs.
- A localStorage snapshot (`fks-contract-<serviceId>`) retains recent results and latency history but does not permanently store override edits.
- Overrides are session-scoped: a browser refresh will revert to the code-defined defaults unless separate persistence is added later.

### Design Trade-offs

- Client-side breakers avoid backend load but can diverge per user session.
- For centralized behavior, mirror breaker state server-side and expose an API; frontend could then poll and display authoritative status.

## Fingerprint Metrics (Error Deduplication)

Error fingerprinting now persists a 24h rolling window in `localStorage` (`fks-error-fingerprints`):

- Each fingerprint record stores first seen, last seen, and count.
- Expired (>24h) entries are pruned lazily on new events.
- Metrics:
  - `frontend_error_unique_fingerprints` — active distinct fingerprints
  - `frontend_error_fingerprints_total_active_window` — total event count across active fingerprints

## Future Enhancements

- Server push of breaker open/close events for cross-user consistency.
- UI countdown timer badge for remaining cooldown.
- Export/import of runtime overrides.
- Configurable fingerprint retention window via environment variable.

