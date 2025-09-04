# Futures Beta Setup (REST + WebSocket)

This project includes a minimal proxy and demo scripts to explore a Futures Beta provider (e.g., Polygon.io).

## ENV configuration

Add the following to your `.env` or shell:

- FUTURES_BETA_REST_URL: Base REST URL (e.g., <https://api.polygon.io>)
- FUTURES_BETA_VERSION: Version segment for REST (e.g., vX or v3)
- FUTURES_BETA_API_KEY: API key for REST
- FUTURES_BETA_WS_URL: WebSocket URL (e.g., wss://socket.polygon.io/futures)
- FUTURES_BETA_NAME: Optional display name in /providers

Optionally set POLYGON_API_KEY, ALPHA_VANTAGE_API_KEY for other features.

## REST proxy routes

The data service forwards requests to `${FUTURES_BETA_REST_URL}/futures/${FUTURES_BETA_VERSION}` with your apiKey:

- GET `/futures/beta/aggs/{ticker}`
- GET `/futures/beta/trades/{ticker}`
- GET `/futures/beta/quotes/{ticker}`
- GET `/futures/beta/contracts`
- GET `/futures/beta/contracts/{ticker}`
- GET `/futures/beta/products`
- GET `/futures/beta/products/{product_code}`
- GET `/futures/beta/schedules`
- GET `/futures/beta/products/{product_code}/schedules`

You can override version via `?version=vX` on any call.

Examples:

- `/futures/beta/aggs/ESZ5?from=2025-07-01&to=2025-07-10&multiplier=1&timespan=minute`
- `/futures/beta/trades/ESZ5?limit=50`

## WebSocket demo

Use `scripts/dev/futures_ws_demo.py` with:

```bash
export FUTURES_BETA_WS_URL='wss://socket.polygon.io/futures'
export FUTURES_BETA_API_KEY='YOUR_KEY'   # optional header if provider requires
export FUTURES_BETA_SUBSCRIBE='{"action":"subscribe","params":"XA.ESZ5"}'
python scripts/dev/futures_ws_demo.py
```

Adjust the subscribe message according to provider docs.

## Notes

- The proxy adds `apiKey` to your REST calls if available.
- Endpoints are versioned; ensure your `FUTURES_BETA_VERSION` matches provider docs.
- Be mindful of beta rate limits and data availability (history currently back to 2025-01-20 per announcement).
