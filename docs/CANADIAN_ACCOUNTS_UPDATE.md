# Canadian Investment Account Types - React App Update

## Overview
Updated the FKS Trading Platform React application to use Canadian investment account types instead of US IRA accounts, specifically tailored for use with Questrade and other Canadian brokers.

## Changes Made

### 1. Type System Updates (`src/types/index.ts`)
- Added comprehensive Canadian account type system with `AccountType` enum
- Added detailed account information with `CanadianAccountInfo` interface
- Includes all major Canadian account types:
  - **Retirement Accounts**: RRSP, Spousal RRSP, LIRA, Locked-in RSP, RIF, LIF
  - **Tax-Free Accounts**: TFSA, FHSA
  - **Taxable Accounts**: Personal Margin, Personal Cash, FX & CFD
  - **Corporate Accounts**: Corporate Margin, Corporate Cash, Partnership, Trust
  - **Specialized**: RESP, RDSP, Prop Trading, Demo
- Added metadata for each account type including tax advantages, contribution limits, withdrawal restrictions

### 2. AccountsManager Component Updates (`src/components/Accounts/AccountsManager.tsx`)
- Replaced IRA account with Canadian account types (RRSP, TFSA, FHSA, etc.)
- Added Canadian-specific mock data for Questrade accounts
- Enhanced UI to show Canadian tax account details:
  - Contribution room display
  - Annual contribution limits for 2024
  - Tax advantage indicators
  - Withdrawal restriction warnings
- Added Canadian account guide banner
- Enhanced account type colors and icons based on account categories
- Added support for contribution room and annual limits in account interface

### 3. PortfolioManager Component Updates (`src/components/Portfolio/PortfolioManager.tsx`)
- Updated account types to use Canadian system
- Modified account filtering to work with Canadian account categories
- Enhanced account display to show Canadian account type names and tax advantages
- Updated mock data with realistic Canadian account examples

### 4. New Canadian Account Guide Component (`src/components/Accounts/AccountGuide.tsx`)
- Comprehensive guide to all Canadian investment account types
- Interactive filtering by account category
- Search functionality across account types
- 2024 contribution limits and key information
- Questrade-specific features and benefits for each account type
- Educational content about tax optimization strategies

### 5. Account Type Info Component (`src/components/Accounts/AccountTypeInfo.tsx`)
- Reusable component for displaying Canadian account type information
- Shows account category, tax advantages, and restrictions
- Consistent styling across the application

### 6. Navigation Updates (`src/components/Navigation/MainNavigation.tsx`)
- Added "Canadian Accounts Guide" to main navigation with Canada flag badge
- Uses MapPin icon to indicate geographic relevance

### 7. App Router Updates (`src/App.tsx`)
- Added route for Canadian Account Guide
- Fixed duplicate route case for strategy development
- Integrated new component with lazy loading

## Canadian Account Types Supported

### Retirement Accounts (Tax-Deferred)
- **RRSP**: Registered Retirement Savings Plan - 18% of income (max ~$30,780 in 2024)
- **Spousal RRSP**: Income splitting strategy
- **LIRA**: Locked-in Retirement Account (from pension transfers)
- **Locked-in RSP**: Locked-In Retirement Savings Plan
- **RIF**: Retirement Income Fund (withdrawal phase)
- **LIF**: Life Income Fund (from locked-in accounts)

### Tax-Free Accounts
- **TFSA**: Tax-Free Savings Account - $7,000 annual limit (2024)
- **FHSA**: First Home Savings Account - $8,000 annual limit, $40,000 lifetime max

### Taxable Investment Accounts
- **Personal Margin**: Leveraged trading account
- **Personal Cash**: Standard investment account
- **FX & CFD**: Foreign Exchange and Contract for Difference trading

### Corporate Accounts
- **Corporate Margin/Cash**: Business investment accounts
- **Partnership**: Partnership investment accounts
- **Trust**: Formal and informal trust accounts

### Specialized Accounts
- **RESP**: Registered Education Savings Plan
- **RDSP**: Registered Disability Savings Plan
- **Prop Trading**: Professional trading accounts
- **Demo**: Paper trading accounts

## Questrade Integration Features

The app now showcases Questrade-specific benefits:
- No commission ETF purchases
- Low stock trading fees ($4.95-$9.95)
- Advanced trading platform support
- Options trading capabilities
- FX and CFD trading
- Wide range of account types

## Tax Optimization Strategy

The app promotes the following Canadian tax optimization approach:
1. Max out TFSA first (tax-free growth and withdrawals)
2. Use RRSP for immediate tax deductions
3. Consider FHSA for first-time home buyers
4. Use taxable accounts for additional investing
5. Corporate accounts for business investments

## Visual Enhancements

- Canadian flag emoji and maple leaf symbols throughout
- Color-coded account categories:
  - Purple: Retirement accounts
  - Green: Tax-free accounts
  - Blue: Taxable accounts
  - Orange: Corporate accounts
  - Indigo/Red: Specialized accounts
- Tax advantage indicators
- Contribution room and limit displays
- Interactive account guide with search and filtering

## Build Status

✅ Successfully builds without errors
✅ All TypeScript types properly defined
✅ Lazy loading implemented for performance
✅ Canadian account guide is fully integrated

## Next Steps

1. **Real Integration**: Connect to actual Questrade API for live account data
2. **Tax Calculations**: Add Canadian tax calculation features
3. **Contribution Tracking**: Implement actual contribution room tracking
4. **Multi-Broker Support**: Extend to other Canadian brokers (Interactive Brokers, TD Direct, etc.)
5. **Real-Time Data**: Add real-time account balance and performance tracking

This update makes the FKS Trading Platform specifically tailored for Canadian investors and traders, providing comprehensive support for the unique Canadian investment account landscape.
