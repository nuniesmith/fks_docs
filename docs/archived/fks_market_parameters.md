# FKS MARKET-SPECIFIC PARAMETERS & SETUP GUIDE

## 📋 NINJATRADER 8 INDICATOR SETUP

### 1. FKS_AI CORE INDICATOR
```
Primary Panel Settings:
- Asset Type: [Select per market below]
- Support/Resistance Length: 150
- Show Entry Zones: TRUE
- Show Signal Labels: TRUE
- Show Market Phase: FALSE (clean chart)
- Show ATR Bands: FALSE
- Clean Chart Mode: FALSE
```

### 2. FKS_AO (Lower Panel)
```
Awesome Oscillator Settings:
- Fast Period: 5
- Slow Period: 34
- Signal Period: 7
- Use AO Zero Cross: TRUE
- Use AO Signal Cross: TRUE
```

### 3. CHART SETUP
```
- Primary: 5-minute chart (main trading)
- Secondary: 15-minute chart (context)
- Data Series: Last (not bid/ask)
- Session Template: CME US Index Futures RTH
```

---

## 🥇 GOLD FUTURES (GC) PARAMETERS

### Indicator Settings:
```
FKS_AI Settings:
- Asset Type: "Gold"
- Max Length: 20
- Accelerator Multiplier: 0.015
- Lookback Period: 200
- Min Wave Ratio: 1.5
- Exit Momentum: 0.7
- Signal Sensitivity: 0.5
- Momentum Lookback: 3
- Signal Quality Threshold: 0.6
```

### Contract Specifications:
- **Tick Size**: $0.10 = $10/contract
- **Typical Daily Range**: $15-25 ($1,500-2,500)
- **Best Trading Hours**: 8:00 AM - 12:00 PM EST
- **Key Levels**: Round numbers ($1950, $2000, etc.)

### Position Sizing Guide:
| Account Risk | Normal (1-2) | Strong (2-3) | Exceptional (4-5) |
|--------------|--------------|--------------|-------------------|
| Per Trade | $750-1,500 | $1,500-2,250 | $3,000-3,750 |
| Stop Range | $5-10 | $7.50-12.50 | $7.50-10 |

---

## 📊 NASDAQ FUTURES (NQ) PARAMETERS

### Indicator Settings:
```
FKS_AI Settings:
- Asset Type: "Stocks" 
- Max Length: 20
- Accelerator Multiplier: 0.01
- Lookback Period: 150
- Min Wave Ratio: 1.5
- Exit Momentum: 0.7
- Signal Sensitivity: 0.6
- Momentum Lookback: 3
- Signal Quality Threshold: 0.65
```

### Contract Specifications:
- **Tick Size**: 0.25 = $5/contract
- **Point Value**: $20/point
- **Typical Daily Range**: 150-250 points
- **Best Trading Hours**: 9:30-10:30 AM, 3:00-4:00 PM EST
- **Key Levels**: 100-point increments

### Position Sizing Guide:
| Account Risk | Normal (1) | Strong (2) | Exceptional (3-4) |
|--------------|------------|------------|-------------------|
| Per Trade | $800-1,000 | $1,600-2,000 | $2,400-3,200 |
| Stop Range | 40-50 pts | 40-50 pts | 30-40 pts |

---

## 🛢️ CRUDE OIL FUTURES (CL) PARAMETERS

### Indicator Settings:
```
FKS_AI Settings:
- Asset Type: "Forex" (similar volatility)
- Max Length: 20
- Accelerator Multiplier: 0.02
- Lookback Period: 150
- Min Wave Ratio: 1.5
- Exit Momentum: 0.7
- Signal Sensitivity: 0.5
- Momentum Lookback: 3
- Signal Quality Threshold: 0.6
```

### Contract Specifications:
- **Tick Size**: $0.01 = $10/contract
- **Typical Daily Range**: $1.50-2.50
- **Best Trading Hours**: 9:00 AM - 2:30 PM EST
- **Key Events**: Wed 10:30 AM (EIA)
- **Key Levels**: Whole dollars ($70, $75, etc.)

### Position Sizing Guide:
| Account Risk | Normal (1-2) | Strong (2-3) | Exceptional (4-5) |
|--------------|--------------|--------------|-------------------|
| Per Trade | $400-800 | $800-1,200 | $1,600-2,000 |
| Stop Range | $0.40-0.50 | $0.40-0.50 | $0.30-0.40 |

---

## ₿ BITCOIN FUTURES PARAMETERS

### Indicator Settings:
```
FKS_AI Settings:
- Asset Type: "Crypto"
- Max Length: 20
- Accelerator Multiplier: 0.03
- Lookback Period: 100
- Min Wave Ratio: 1.5
- Exit Momentum: 0.7
- Signal Sensitivity: 0.4
- Momentum Lookback: 3
- Signal Quality Threshold: 0.7
```

### Contract Specifications:
- **Micro BTC (MBT)**: 0.1 BTC
- **Tick Size**: $5 = $0.50/contract
- **Typical Daily Range**: 3-5%
- **Best Hours**: 8 PM - 2 AM EST (Asia)
- **Key Levels**: Psychological ($40k, $50k)

### Position Sizing Guide:
| Account Risk | Normal (1 micro) | Strong (2 micro) | Exceptional (3 micro) |
|--------------|-----------------|------------------|----------------------|
| Per Trade | $500-750 | $1,000-1,500 | $1,500-2,250 |
| Stop Range | 1-1.5% | 1-1.5% | 0.75-1% |

---

## ⚙️ ADVANCED SETTINGS BY MARKET CONDITION

### TRENDING MARKETS (All Assets):
```
- Signal Sensitivity: -0.1 (more selective)
- Min Wave Ratio: 1.7 (higher threshold)
- Regime MA Length: 200
- Exit Momentum: 0.8 (hold longer)
```

### VOLATILE MARKETS (All Assets):
```
- Signal Sensitivity: +0.1 (more signals)
- Min Wave Ratio: 1.3 (lower threshold)
- ATR Length: 20 (more responsive)
- Position Size: -50%
```

### RANGING MARKETS (All Assets):
```
- Signal Sensitivity: 0.0 (default)
- Support/Resistance Length: 100 (shorter)
- Exit Momentum: 0.6 (exit quicker)
- Position Size: -30%
```

---

## 📊 MULTI-TIMEFRAME CONFIRMATION

### Timeframe Combinations by Market:

**GOLD (GC):**
- Primary: 5-min
- Confirmation: 15-min
- Context: 60-min

**NASDAQ (NQ):**
- Primary: 3-min
- Confirmation: 15-min
- Context: 60-min

**CRUDE (CL):**
- Primary: 5-min
- Confirmation: 30-min
- Context: 4-hour

**BITCOIN:**
- Primary: 15-min
- Confirmation: 1-hour
- Context: 4-hour

---

## 🔧 OPTIMIZATION SCHEDULE

### Weekly Adjustments:
- Monday: Review wave ratio thresholds
- Wednesday: Adjust signal sensitivity
- Friday: Optimize exit momentum levels

### Monthly Reviews:
- Backtest accelerator multiplier changes
- Review K-means volatility clusters
- Adjust position sizing matrix
- Update support/resistance lengths

---

## 🚨 MARKET-SPECIFIC WARNINGS

### GOLD:
- ⚠️ Avoid: FOMC days (extreme volatility)
- ⚠️ Watch: DXY correlation (inverse)
- ⚠️ Thin: Asian session (use caution)

### NASDAQ:
- ⚠️ Avoid: First 5 min of open
- ⚠️ Watch: FAANG earnings impact
- ⚠️ Gap Risk: Overnight positions

### CRUDE:
- ⚠️ Avoid: API Tuesday evenings
- ⚠️ Watch: OPEC announcements
- ⚠️ Volatility: Wednesday 10:30 AM

### BITCOIN:
- ⚠️ Weekend: Lower liquidity
- ⚠️ Funding: Check rates (perps)
- ⚠️ News: 24/7 sensitivity

---

## 💾 NINJATRADER WORKSPACE SETUP

### Recommended Layout:
```
┌─────────────────┬─────────────────┐
│                 │                 │
│   5-min Chart   │  15-min Chart   │
│   with FKS_AI   │  with FKS_AI    │
│                 │                 │
├─────────────────┼─────────────────┤
│                 │                 │
│    FKS_AO       │   DOM/Orders    │
│  (Lower Panel)  │                 │
│                 │                 │
└─────────────────┴─────────────────┘
```

### Color Scheme:
- Background: Dark (reduce eye strain)
- Support: Green (#00ffbb)
- Resistance: Red (#ff1100)
- Middle Band: Blue
- Candles: Use heatmap colors

---

## 📱 ALERTS CONFIGURATION

### Critical Alerts (All Markets):
1. "G" or "Top" signal appears
2. Signal quality > 80%
3. Daily P&L approaching limits
4. Market regime change

### Optional Alerts:
1. Potential signals (diamonds)
2. Wave ratio > 2.0x
3. Volatility cluster change
4. Session start/end

---

*Remember: These parameters are starting points. Track your results and adjust based on actual performance. The market's personality changes - your settings should too.*