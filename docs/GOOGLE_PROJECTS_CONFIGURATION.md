# FKS Google Cloud Projects Configuration

## Overview
You've created two separate Google Cloud projects for better security isolation and scope management:

1. **fks-trading-systems** - Main trading platform authentication
2. **fks-calendar** - Calendar-specific features and extended calendar access

## Project Configuration

### FKS Trading Systems Project
**Purpose**: Primary authentication for the trading platform
**Project ID**: `fks-trading-systems`
**Scopes**: Minimal required permissions for user authentication
- `https://www.googleapis.com/auth/userinfo.profile`
- `https://www.googleapis.com/auth/userinfo.email`
- `https://www.googleapis.com/auth/calendar.readonly` (view only)

### FKS Calendar Project
**Purpose**: Full calendar integration and management
**Project ID**: `fks-calendar`
**Scopes**: Complete calendar access for calendar app features
- `https://www.googleapis.com/auth/userinfo.profile`
- `https://www.googleapis.com/auth/userinfo.email`
- `https://www.googleapis.com/auth/calendar.readonly`
- `https://www.googleapis.com/auth/calendar.events`
- `https://www.googleapis.com/auth/calendar`

## Environment Variables Setup

Add these to your `.env` file:

```bash
# FKS Trading Systems (Primary Authentication)
REACT_APP_GOOGLE_TRADING_CLIENT_ID=your_fks_trading_systems_client_id_here
REACT_APP_GOOGLE_TRADING_CLIENT_SECRET=your_fks_trading_systems_client_secret_here
REACT_APP_GOOGLE_TRADING_REDIRECT_URI=http://localhost:3001/auth/google/callback

# FKS Calendar (Calendar Features)
REACT_APP_GOOGLE_CALENDAR_CLIENT_ID=your_fks_calendar_client_id_here
REACT_APP_GOOGLE_CALENDAR_CLIENT_SECRET=your_fks_calendar_client_secret_here
REACT_APP_GOOGLE_CALENDAR_REDIRECT_URI=http://localhost:3001/auth/google/calendar/callback
```

## Google Cloud Console Setup

### For Both Projects:

1. **Enable APIs**:
   - Google+ API (for user info)
   - Calendar API (for calendar access)

2. **OAuth 2.0 Credentials**:
   - Create OAuth 2.0 Client IDs
   - Set authorized redirect URIs

3. **Authorized Redirect URIs**:
   ```
   # Development
   http://localhost:3001/auth/google/callback
   http://localhost:3001/auth/google/calendar/callback
   
   # Production (replace with your domain)
   https://your-domain.com/auth/google/callback
   https://your-domain.com/auth/google/calendar/callback
   ```

## Authentication Flow Priority

### Primary: Authentik (Open Source)
- **Default**: Authentik with passkey support
- **Fallback**: Authentik with password
- **Benefits**: Self-hosted, privacy-focused, open source

### Secondary: Google OAuth
- **Trading Project**: Basic authentication with minimal permissions
- **Calendar Project**: Full calendar access when needed
- **Benefits**: Familiar to users, reliable uptime

## Usage in Code

### Authentication Service
```typescript
// Primary authentication (Authentik first)
const authUrl = await login(true, 'authelia'); // Prefer passkey

// Alternative authentication (Google)
const authUrl = await login(false, 'google'); // Google OAuth
```

### Calendar Service
```typescript
// Uses fks-calendar project credentials
const calendarService = GoogleCalendarService.getInstance();
const events = await calendarService.getTradingCalendarEvents(userId);
```

### Project Selection
```typescript
// Trading platform authentication
const tradingAuthUrl = googleService.getAuthUrl('trading', state);

// Calendar feature authentication
const calendarAuthUrl = googleService.getAuthUrl('calendar', state);
```

## Security Benefits

### Separation of Concerns
- **Trading System**: Minimal Google permissions, focused on core platform
- **Calendar App**: Dedicated calendar permissions, isolated scope

### Principle of Least Privilege
- Users only grant permissions they actually need
- Trading users don't need full calendar access unless using calendar features

### Authentik Priority
- **Open Source**: Full control over authentication logic
- **Self-Hosted**: No dependency on external OAuth providers
- **Privacy**: User data stays within your infrastructure

## Production Deployment

### Environment Configuration
1. Update redirect URIs for production domain
2. Configure OAuth consent screens for both projects
3. Set up proper CORS policies
4. Enable audit logging

### Security Considerations
1. Use different client secrets for each environment
2. Implement proper token rotation
3. Monitor OAuth usage and quotas
4. Set up alerts for authentication failures

## Project URLs

After setup, you'll have:
- **Trading Auth**: `https://console.cloud.google.com/apis/credentials?project=fks-trading-systems`
- **Calendar Auth**: `https://console.cloud.google.com/apis/credentials?project=fks-calendar`

This dual-project approach provides maximum flexibility while maintaining security isolation between your core trading platform and extended calendar features.
