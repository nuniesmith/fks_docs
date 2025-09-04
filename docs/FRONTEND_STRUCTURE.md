# Frontend Feature-Based Structure (Refactor Plan)

This document outlines the emerging feature-sliced structure for the React web interface.

## New Top-Level Folders

- `src/features` – Domain-specific slices (auth, trading, portfolio, gamification, monitoring, ai, calendar, notifications, settings, architecture, etc.)
- `src/shared` – Cross-cutting reusable primitives (components, hooks, services, utils, types, constants)

```text
src/
  features/
    auth/
      components/
      hooks/
      services/
      types/
      index.ts
    trading/
    gamification/
    monitoring/
    portfolio/
    ai/
    ...
  shared/
    components/
    hooks/
    services/
    types/
    utils/
```

## Import Aliases

Configured in `tsconfig.json` and `vite.config.ts`:

| Alias | Path |
|-------|------|
| `@features/*` | `src/features/*` |
| `@shared/*`   | `src/shared/*`   |

Example:

```ts
import { LoginPage } from '@features/auth';
import { usePrometheus } from '@shared/hooks/usePrometheusMetrics';
```

## Migration Guidelines

1. Move one feature at a time; update its imports to use `@features/` or `@shared/`.
2. Provide a feature root `index.ts` that re-exports only the public surface (pages, major hooks, contexts). Keep internals private.
3. Global contexts/services (UserContext, NotificationSystem, Prometheus provider) migrate into `shared/`.
4. If a module is used by two features, move it to `shared/` (prefer waiting for the second consumer before promoting).
5. Keep tests colocated or mirror structure under a central `tests/` directory; be consistent.
6. Avoid barrel re-export chains deeper than one level (prevents circular import surprises).

## Metrics for Success

- Reduced average relative import depth.
- Faster navigation to domain code.
- No circular dependency warnings (`madge src --circular`).
- Build size stable or reduced (check vendor duplication).

## Next Steps

- Migrate `auth` slice first (LoginPage, AuthCallback, context & services).
- Introduce lint rule to block cross-feature deep imports (`@features/other-feature/internal/*`).
- Document any runtime config changes after full migration.

