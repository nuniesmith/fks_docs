# Frontend Metrics & Prometheus Integration

This React frontend exposes Prometheus-compatible metrics for use in Grafana dashboards.

## Provider

`PrometheusMetricsProvider` (in `hooks/usePrometheusMetrics.ts`) wraps the app (configured in `main.tsx`).

## Automatically Collected Metrics

Contract test hook (`useServiceContractTests`):

- `fks_contract_test_latency_ms` (histogram) — per spec latency in ms
- `fks_contract_test_failures_total` (counter) — failures/errors
- `fks_contract_test_sla_breaches_total` (counter) — SLA breaches
- `fks_contract_test_schema_failures_total` (counter) — shallow/deep schema validation failures (label `kind`)
- `fks_contract_latency_aggregate_ms` (histogram) — cross-service aggregate latency samples (aggregate dashboard)
- `fks_contract_open_circuits` (gauge) — number of currently open circuit breakers (per service)
- `fks_contract_circuit_open` (gauge) — 1/0 per spec (labels: `service`, `spec`)
- `fks_contract_circuit_failure_streak` (gauge) — current consecutive failure streak per spec

Frontend performance (`useFrontendMetrics`):

- `frontend_nav_timing_dom_content_loaded_ms` (gauge)
- `frontend_nav_timing_page_load_ms` (gauge)
- `frontend_ttfb_ms` (histogram)
- `frontend_route_change_duration_ms` (histogram)
- `frontend_route_changes_total` (counter)
- `frontend_errors_total` (counter)
- `frontend_error_events_total` (counter) — all error/failure events captured
- `frontend_error_fingerprint_events_total` (counter) — events per distinct fingerprint (labels: `fingerprint`, `type`)
- `frontend_error_unique_fingerprints` (gauge) — number of unique error fingerprints observed in session
- `frontend_error_fingerprints_total_active_window` (gauge) — total error events counted (sum of counts) within active retention window

## Manual Instrumentation

Use the `usePrometheus()` hook to access:

```ts
const { inc, set, observe } = usePrometheus();
inc('custom_counter_total', { feature: 'x' });
observe('custom_latency_ms', 123, { action: 'y' });
```

## Pushgateway (Optional)

Mount a Pushgateway and set an environment variable (e.g. `VITE_PUSHGATEWAY_URL`). Then in a top-level component add:

```ts
const url = import.meta.env.VITE_PUSHGATEWAY_URL;
if (url) usePrometheusPushGateway({ url, job: 'frontend', instance: window.location.host });
```

## /metrics Route

Navigate to `/metrics` in the SPA to view current in-memory metrics (client rendered). For production scraping prefer the Pushgateway or a server endpoint.

## Grafana Dashboard Ideas

| Panel | Metric | Notes |
|-------|--------|-------|
| Contract Latency p95 | `histogram_quantile(0.95, sum(rate(fks_contract_test_latency_ms_bucket[5m])) by (le))` | Overall |
| SLA Breaches Rate | `sum(increase(fks_contract_test_sla_breaches_total[15m])) / sum(increase(fks_contract_test_latency_ms_count[15m]))` | Rolling window |
| Route Change Durations | `histogram_quantile(0.90, sum(rate(frontend_route_change_duration_ms_bucket[5m])) by (le))` | UX perf |
| Frontend Errors | `increase(frontend_errors_total[1h])` | Error spikes |

### Importable Dashboard

An example Grafana dashboard JSON is provided at `docs/grafana/frontend-observability-dashboard.json`. Update the Prometheus datasource UID (`PROM_DS`) after import.

## Caveats

- Metrics live in browser memory; a refresh resets counters (unless pushed).
- High-cardinality labels (like full paths with IDs) should be avoided — sanitize before labeling.
- Fingerprinting hashes first 2000 chars of stack/message to base36; fingerprints persist in `localStorage` for a 24h rolling window (expired entries pruned on new events). Gauges reflect only active (unexpired) fingerprints.
- Circuit breaker skips executions with status 'skip' message 'circuit-open'; these are not failures. Open & streak state surfaced via UI badges and Prometheus gauges.
- Runtime overrides (e.g. editing latency budgets or enabling/disabling specs in the UI) affect subsequent test runs immediately but do not retroactively modify stored historical samples; they are ephemeral per browser session unless persisted separately.

## Web Vitals

Collected via `useWebVitals` hook:

- `frontend_web_vital_lcp_ms` (gauge)
- `frontend_web_vital_cls` (gauge)
- `frontend_web_vital_inp_ms` (gauge)

