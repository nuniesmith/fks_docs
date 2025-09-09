# Frontend Refactor & Performance Optimization Plan

**Last updated:** 2025-08-19

## 1. Executive Summary
We are migrating the React UI from a legacy folder-centric ("components/*") layout toward a sustainable **feature-sliced architecture** with a populated **shared layer**, while simultaneously tightening bundle performance. Key wins so far:

- Feature directory established (`features/trading`) with successful migrations: `TradingDashboard`, `TradingChart`, `TradingInterface`, `TradingModeController`.
- Clean one-line legacy shims now in place for `TradingModeController`, `OrderManagement`, `SignalsPanel` (placeholders in feature layer for the last two still need real logic).
- Vendor bundle reduced from ~513 kB to ~337 kB via refined `manualChunks` splitting (`react-dom` isolated; vendor < 500 kB target achieved).
- Aliases (`@features`, `@shared`) wired in Vite config to encourage decoupling and consistent import style.

Immediate gaps:
- `OrderManagement` and `SignalsPanel` feature files are placeholders (functionality not yet restored).
- Heavy visualization chunks (`viz`, `charts-core`) still eagerly loaded.
- `shared/` is structurally present but mostly empty; hooks/services/utils not moved yet.
- Remaining large legacy subtrees (e.g. `components/Trading/`) increase cognitive load and drift risk.

## 2. Current Architecture Snapshot

```text
src/
  components/              (legacy — gradually shrinking)
  features/
    trading/
      components/{dashboard,charts,interface,execution,strategy,...}
      index.ts (barrel)
  shared/
    components/ hooks/ services/ types/ utils/ (placeholders)
  services/ (to be redistributed)
  hooks/    (to be partially moved to shared/ or feature-local)
  context/  (global contexts — candidates for shared/contexts)
  pages/    (route-level composition only)
```
Chunking (latest build):

- react-dom ~174 kB (separate)
- vendor ~337 kB (OK)
- viz ~156 kB (candidate for lazy)
- charts ~139 kB (candidate for lazy)

## 3. Guiding Principles

1. **Incremental Safety**: Move → Shim → Validate → Commit (no large bang rewrite).
2. **Colocation**: Keep component + tests + styles + feature services together.
3. **Shared Layer Discipline**: Only cross-domain elements go into `shared/`.
4. **Explicit Boundaries**: Each feature has: `components/`, optional `hooks/`, `services/`, `types/`, `__tests__/`.
5. **Performance Budget**: Keep vendor < 350 kB (stretch), any single non-lazy chunk < 180 kB.
6. **Observability**: After each migration, run build + minimal smoke test.

## 4. High-Priority Work (Ordered)

| Priority | Task | Outcome |
|----------|------|---------|
| 1 | Migrate real `OrderManagement` logic into feature file | Restore execution UI functionality |
| 2 | Migrate real `SignalsPanel` logic into feature file | Restore signals functionality |
| 3 | Extract shared trading types/utils (`orders`, `signals`, formatters) | Reduce duplication & prep for reuse |
| 4 | Populate `shared/services` (apiClient, WebSocket, metrics) & update imports | Centralized stable API surface |
| 5 | Lazy-load heavy viz/chart panels (dynamic import) | Faster initial paint / TTI |
| 6 | Further split large viz chunk (separate `chart.js`, `lightweight-charts`) | Granular cache / smaller diff updates |
| 7 | Dep audit & prune (`recharts?`, `moment?`) | Shrink vendor baseline |
| 8 | Migrate remaining `components/Trading/*` subdomains (strategy, risk, etc.) | Eliminate legacy drift |
| 9 | Colocate tests under features & raise coverage | Confidence in refactor velocity |
| 10 | Flatten / migrate static assets to `public/` where appropriate | Simpler asset pipeline |
| 11 | Remove emptied legacy folders & add ESLint guard rules | Prevent regressions |
| 12 | Update docs / contributor guidelines | Onboard clarity |

## 5. Detailed Phase Plan
 
### Phase 1 – Execution Components Restoration

- Implement `features/trading/components/execution/OrderManagement.tsx` with original UI + extracted hook `useDemoOrders()`.
- Implement `features/trading/components/execution/SignalsPanel.tsx` with hook `useDemoSignals()`.
- Add provisional types (until shared trading types extracted).
- Build + smoke test: open interface, verify tabs / filters / counters render.

### Phase 2 – Shared Layer Population

Artifacts to create:

- `shared/types/trading.ts` (Order, Trade, TradingSignal, SignalProvider, enums)
- `shared/utils/tradingFormat.ts` (formatCurrency, formatTime, color/status helpers)
- `shared/services/realtime.ts` (or consolidate existing WebSocket service)
- Migrate generic hooks (e.g., `useWebVitals`, `useRealtime`) → `shared/hooks/`
Update imports via search/replace, commit each logical grouping.

### Phase 3 – Performance Enhancements

- Convert visualization-heavy components to lazy: `const VizPanel = React.lazy(() => import('./VizPanel'));`
- Gate rendering behind user interaction / route.
- Add skeleton fallbacks.
- Split `viz` into `chartjs`, `recharts` (if still used), `markdown` (already separated), `lightweight-charts` (now `charts-core`).
- Verify chunk report; enforce budgets with a simple CI script (optional later).

### Phase 4 – Broad Feature Migration
 
For each legacy domain under `components/Trading/` (strategy, risk, portfolio overlap):
 
1. Create subfolder in `features/trading/components/<domain>`.
2. Copy implementation (not move) → rename imports to aliases.
3. Replace legacy file content with one-line shim.
4. Build & smoke, then remove duplication when stable.
5. Commit: `refactor(trading): migrate <domain>`.

### Phase 5 – Testing Alignment
 
- Move or create tests under `features/trading/__tests__/` (Vitest patterns).
- Add tests: OrderManagement basic render, filter logic; SignalsPanel filter + modal open.
- Ensure mocks (signals/orders) live in `__fixtures__` or inline inside test directory.
- Increment coverage threshold gradually (avoid brittle jumps).

### Phase 6 – Cleanup & Governance
 
- Remove unused libs (run `grep -R recharts src/`; if unused, uninstall & remove chunk rule).
- Delete empty legacy folders once import graph is clean (grep old relative paths = none).
- Add ESLint rule / custom lint script: disallow importing from `src/components/Trading/` (except shims) after migration.
- Update docs: architecture diagram + migration guidelines.

## 6. Acceptance Criteria per Priority
 
| Task | Acceptance |
|------|------------|
| OrderManagement migration | Feature file mirrors legacy UI; shim untouched; build passes |
| SignalsPanel migration | All filters & modal functional; no placeholder text |
| Shared trading types | No duplicated inline interfaces across execution components |
| Lazy viz loading | Initial route load excludes `viz` & `charts-core` chunks until accessed |
| Dep pruning | Vendor further reduced or stable ≤ 340 kB |
| Legacy removal | `components/Trading/*` only contains shims or is deleted |

## 7. Risks & Mitigations
 
| Risk | Mitigation |
|------|------------|
| Silent functional regression in execution panels | Add minimal interaction tests before removing legacy code |
| Circular import when extracting utils | Keep helpers pure & dependency-light; avoid importing feature code inside shared |
| Over-splitting causing too many small requests | Monitor network waterfall; group micro-chunks if < 5 kB frequently used |
| Developer confusion during hybrid period | Keep this plan updated; short PR descriptions referencing steps |

## 8. Metrics & Tracking
 
| Metric | Baseline | Target |
|--------|----------|--------|
| Vendor chunk | ~337 kB | ≤ 340 kB (maintain) |
| Largest eager non-framework chunk | 156 kB (viz) | < 110 kB (post-split/lazy) |
| First Paint (subjective local) | Current baseline | Improve after lazy viz |
| Test coverage (feature: trading) | TBD | +15% over baseline after Phase 5 |

## 9. Operational Checklist (Quick Reference)
 
- [ ] Migrate `OrderManagement`
- [ ] Migrate `SignalsPanel`
- [ ] Extract shared trading types & utils
- [ ] Populate shared services/hooks
- [ ] Lazy-load heavy visualization
- [ ] Further slice viz/chart chunks
- [ ] Dependency audit & prune
- [ ] Migrate remaining trading subdomains
- [ ] Colocate / add tests
- [ ] Remove legacy component folders
- [ ] Update docs & contributor guide

## 10. Commit Strategy
 
Small, narrative commits:
 
- `refactor(trading): migrate order management component`
- `feat(shared): add trading types & format utils`
- `perf(build): lazy load visualization panels`
- `chore(deps): remove unused recharts` (if applicable)

## 11. Future Enhancements (Post-Core Migration)
 
- Route-level code splitting map.
- Prefetch heuristics (idle-time `link rel="prefetch"`).
- Performance budget CI check (fail on threshold breach).
- Storybook (or alternative) fed from `features/`.

## 12. How to Execute a Single Migration (Template)
 
1. Identify legacy component(s).
2. Copy code into `features/<domain>/components/...` (keep relative asset paths stable).
3. Fix imports to use aliases (`@shared`, `@features`).
4. Replace legacy file with single-line re-export.
5. Build + run smoke interaction.
6. Commit & push.
7. (Later) Remove shim once all importers updated (grep confirms).

## 13. Rollback Strategy
 
Because each step is additive + shimmed, rollback = revert the single commit that introduced the feature version; legacy shim still points to original if not yet deleted. Avoid batch deleting legacy code until a final pass with grep validation.

## 14. Open Questions (Track & Resolve)
 
| Question | Action |
|----------|--------|
| Are `moment` or `recharts` actually used? | Grep + remove if unused |
| Should trading environment context move to shared or remain feature-local? | Decide after full trading migration |
| Which panels justify prefetch? | Measure actual user navigation patterns |

---
**Next Action Right Now:** Migrate real `OrderManagement` implementation into the feature file and extract types into `shared/types/trading.ts`.

(Keep this document updated per phase; add a changelog section if team size grows.)
