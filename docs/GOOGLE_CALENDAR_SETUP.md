# Google Calendar API Integration Setup Guide

## Overview
This guide will help you set up Google Calendar API integration for your FKS Trading React app using your dev account `nunie.smith01@gmail.com`.

## Prerequisites
- Google Account (nunie.smith01@gmail.com)
- Google Cloud Console access
- Node.js and npm installed
- React app running locally

## Step 1: Enable Google Calendar API

1. **Go to Google Cloud Console**
   - Visit: https://console.cloud.google.com/
   - Sign in with `nunie.smith01@gmail.com`

2. **Create or Select a Project**
   - Create a new project: "FKS Trading Calendar"
   - Or select an existing project

3. **Enable Google Calendar API**
   - Go to APIs & Services > Library
   - Search for "Google Calendar API"
   - Click "Enable"

## Step 2: Create OAuth 2.0 Credentials

1. **Configure OAuth Consent Screen**
   - Go to APIs & Services > OAuth consent screen
   - Choose "External" (for testing with your personal account)
   - Fill in required information:
     - App name: "FKS Trading Calendar"
     - User support email: nunie.smith01@gmail.com
     - Developer contact: nunie.smith01@gmail.com
   - Add scopes:
     - `https://www.googleapis.com/auth/calendar.readonly`
     - `https://www.googleapis.com/auth/calendar.events`
     - `https://www.googleapis.com/auth/calendar`

2. **Create OAuth 2.0 Client ID**
   - Go to APIs & Services > Credentials
   - Click "Create Credentials" > "OAuth 2.0 Client ID"
   - Application type: "Web application"
   - Name: "FKS Trading Calendar Client"
   - Authorized JavaScript origins:
     - `http://localhost:3000`
     - `https://fkstrading.xyz` (for production)
   - Authorized redirect URIs:
     - `http://localhost:3000/auth/google/callback`
     - `https://fkstrading.xyz/auth/google/callback`

3. **Download Credentials**
   - Download the JSON file or copy the Client ID and Client Secret

## Step 3: Update Environment Variables

1. **Edit your .env file**:
   ```bash
   # Google Calendar API Configuration
   REACT_APP_GOOGLE_CLIENT_ID=your_actual_client_id_here
   REACT_APP_GOOGLE_CLIENT_SECRET=your_actual_client_secret_here
   REACT_APP_GOOGLE_REDIRECT_URI=http://localhost:3000/auth/google/callback
   ```

2. **Replace the placeholder values** with your actual credentials from Google Cloud Console

## Step 4: Test the Integration

1. **Start your React app**:
   ```bash
   cd /home/jordan/fks/src/web/react
   npm run dev
   ```

2. **Navigate to Calendar section**
   - Open http://localhost:3000
   - Go to Calendar section
   - Click "Connect Google" button

3. **Authorize the application**
   - You'll be redirected to Google OAuth
   - Sign in with `nunie.smith01@gmail.com`
   - Grant calendar permissions

4. **Test sync functionality**
   - Click "Sync to Google" to push FKS events to Google Calendar
   - Click "Load from Google" to pull events from Google Calendar

## Step 5: Calendar Features

### What's Included:
- **Bi-directional sync**: Push FKS events to Google Calendar and pull Google events
- **Event categorization**: Foundation, Development, Testing, Integration, Polish
- **Status tracking**: Pending, In-Progress, Completed, Blocked
- **Priority levels**: High, Medium, Low
- **Deliverables**: Each event can have associated deliverables
- **Color coding**: Events are color-coded by category in Google Calendar

### Usage:
1. **Sync FKS Development Schedule to Google**:
   - Click "Sync to Google" to create your development calendar in Google Calendar
   - Events will be prefixed with category tags: [Foundation], [Development], etc.

2. **View Combined Calendar**:
   - Your React app shows both FKS events and Google Calendar events
   - Easy to see your development schedule alongside other commitments

3. **Update Status**:
   - As you complete development tasks, update the status in your React app
   - Sync again to update Google Calendar

## Step 6: Production Setup

For production deployment on `fkstrading.xyz`:

1. **Update OAuth Redirect URIs**:
   - Add: `https://fkstrading.xyz/auth/google/callback`

2. **Update Environment Variables**:
   ```bash
   REACT_APP_GOOGLE_REDIRECT_URI=https://fkstrading.xyz/auth/google/callback
   ```

3. **Publish OAuth App** (optional):
   - Go to OAuth consent screen
   - Submit for verification if needed for public use

## Security Considerations

1. **Client Secret Protection**:
   - In production, consider using a backend service for OAuth flow
   - Current implementation stores tokens in localStorage (suitable for development)

2. **Scope Minimization**:
   - Only requests necessary calendar permissions
   - Users can revoke access anytime in Google Account settings

3. **Token Management**:
   - Tokens are automatically refreshed when needed
   - Users can sign out to clear stored credentials

## Troubleshooting

### Common Issues:

1. **"Invalid Client" Error**:
   - Check that Client ID matches exactly
   - Verify authorized origins include your domain

2. **"Redirect URI Mismatch"**:
   - Ensure redirect URI in code matches Google Cloud Console
   - Check for trailing slashes or http vs https

3. **"Access Blocked"**:
   - Make sure OAuth consent screen is configured
   - Check that your email is added as a test user

4. **Calendar Not Syncing**:
   - Check browser console for errors
   - Verify API is enabled in Google Cloud Console
   - Ensure sufficient scopes are granted

## Next Steps

1. **Add your actual Google Calendar credentials** to the .env file
2. **Test the integration** with your development environment
3. **Create your FKS development calendar** in Google Calendar
4. **Set up recurring events** for daily development work
5. **Share calendar** with team members if needed

## Support

For additional help:
- Google Calendar API Documentation: https://developers.google.com/calendar
- OAuth 2.0 Setup: https://developers.google.com/identity/protocols/oauth2
- Google Cloud Console: https://console.cloud.google.com/

The integration is designed to enhance your development workflow by keeping your FKS trading system development schedule synchronized with your personal Google Calendar.
