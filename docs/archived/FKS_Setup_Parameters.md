# FKS Setup & Configuration Parameters

## 🔧 **NINJATRADER 8 SETUP**

### **1. FKS_AI (Primary Panel)**
```
Asset Type: [Select per market - see below]
Support/Resistance Length: 150
Show Entry Zones: TRUE
Show Signal Labels: TRUE  
Show Market Phase: FALSE (clean chart)
Show ATR Bands: FALSE
Clean Chart Mode: FALSE
Max Length: 20
Lookback Period: varies per market
Min Wave Ratio: 1.5
Exit Momentum: 0.7
Signal Quality Threshold: 0.65
```

### **2. FKS_AO (Lower Panel)**
```
Fast Period: 5
Slow Period: 34
Signal Period: 7
Use AO Zero Cross: TRUE
Use AO Signal Cross: TRUE
```

### **3. Chart Configuration**
```
Primary Timeframe: 5-minute (main trading)
Confirmation Timeframe: 15-minute (context)
Data Series: Last (not bid/ask)
Session Template: CME US Index Futures RTH
```

---

## 🥇 **GOLD FUTURES (GC) PARAMETERS**

### **FKS_AI Settings:**
```
Asset Type: "Gold"
Max Length: 20
Accelerator Multiplier: 0.015
Lookback Period: 200
Signal Sensitivity: 0.5
Momentum Lookback: 3
Signal Quality Threshold: 0.6
```

### **Trading Specifications:**
- **Tick Size**: $0.10 = $10/contract
- **Daily Range**: $15-25 ($1,500-2,500)
- **Best Hours**: 8:00 AM - 12:00 PM EST
- **Key Levels**: Round numbers ($1950, $2000, etc.)

### **Position Sizing:**
| Signal Tier | Normal | Strong | Exceptional |
|-------------|--------|--------|-------------|
| Contracts | 1-2 | 2-3 | 4-5 |
| Risk/Trade | $750-1,500 | $1,500-2,250 | $3,000-3,750 |
| Stop Range | $5-10 | $7.50-12.50 | $7.50-10 |

---

## 📊 **NASDAQ FUTURES (NQ) PARAMETERS** 

### **FKS_AI Settings:**
```
Asset Type: "Stocks"
Max Length: 20
Accelerator Multiplier: 0.01
Lookback Period: 150
Signal Sensitivity: 0.6
Momentum Lookback: 3
Signal Quality Threshold: 0.65
```

### **Trading Specifications:**
- **Tick Size**: 0.25 = $5/contract  
- **Point Value**: $20/point
- **Daily Range**: 150-250 points
- **Best Hours**: 9:30-10:30 AM, 3:00-4:00 PM EST
- **Key Levels**: 100-point increments

### **Position Sizing:**
| Signal Tier | Normal | Strong | Exceptional |
|-------------|--------|--------|-------------|
| Contracts | 1 | 2 | 3-4 |
| Risk/Trade | $800-1,000 | $1,600-2,000 | $2,400-3,200 |
| Stop Range | 40-50 pts | 40-50 pts | 30-40 pts |

---

## 🛢️ **CRUDE OIL FUTURES (CL) PARAMETERS**

### **FKS_AI Settings:**
```
Asset Type: "Forex" (similar volatility)
Max Length: 20
Accelerator Multiplier: 0.02
Lookback Period: 150
Signal Sensitivity: 0.5
Momentum Lookback: 3
Signal Quality Threshold: 0.6
```

### **Trading Specifications:**
- **Tick Size**: $0.01 = $10/contract
- **Daily Range**: $1.50-2.50
- **Best Hours**: 9:00 AM - 2:30 PM EST
- **Key Events**: Wed 10:30 AM (EIA inventory)
- **Key Levels**: Whole dollars ($70, $75, etc.)

### **Position Sizing:**
| Signal Tier | Normal | Strong | Exceptional |
|-------------|--------|--------|-------------|
| Contracts | 1-2 | 2-3 | 4-5 |
| Risk/Trade | $400-800 | $800-1,200 | $1,600-2,000 |
| Stop Range | $0.40-0.50 | $0.40-0.50 | $0.30-0.40 |

---

## ₿ **BITCOIN FUTURES (MICRO BTC) PARAMETERS**

### **FKS_AI Settings:**
```
Asset Type: "Crypto"
Max Length: 20
Accelerator Multiplier: 0.03
Lookback Period: 100
Signal Sensitivity: 0.4
Momentum Lookback: 3
Signal Quality Threshold: 0.7
```

### **Trading Specifications:**
- **Contract Size**: 0.1 BTC (Micro Bitcoin)
- **Tick Size**: $5 = $0.50/contract
- **Daily Range**: 3-5% of price
- **Best Hours**: 8 PM - 2 AM EST (Asia overlap)
- **Key Levels**: Psychological ($40k, $50k, etc.)

### **Position Sizing:**
| Signal Tier | Normal | Strong | Exceptional |
|-------------|--------|--------|-------------|
| Contracts | 1 micro | 2 micro | 3 micro |
| Risk/Trade | $500-750 | $1,000-1,500 | $1,500-2,250 |
| Stop Range | 1-1.5% | 1-1.5% | 0.75-1% |

---

## ⚙️ **STRATEGY CONFIGURATION**

### **FKS_Strategy_Clean Settings:**
```
Strategy Mode: MonitoringOnly (start here)
Enable Live Trading: FALSE (enable only after testing)
Debug Level: 2 (detailed logging)

Base Contract Size: 1
Max Contract Size: 5 (per market limits above)
Risk Percent Per Trade: 1.0%
ATR Stop Multiplier: 2.0
ATR Target Multiplier: 3.0

Signal Threshold: 0.65
Strong Signal Threshold: 0.80
Use Time Filters: TRUE

Soft Profit Target: $2,000
Hard Profit Target: $3,000
Soft Loss Limit: $1,000
Hard Loss Limit: $1,500
Max Daily Trades: 10

Show Dashboard: TRUE
```

---

## 🔄 **MARKET CONDITION ADJUSTMENTS**

### **TRENDING MARKETS (All Assets):**
```
Signal Sensitivity: -0.1 (more selective)
Min Wave Ratio: 1.7 (higher threshold)
Exit Momentum: 0.8 (hold longer)
Position Size: +20% (trending bonus)
```

### **VOLATILE MARKETS (All Assets):**
```
Signal Sensitivity: +0.1 (more signals)
Min Wave Ratio: 1.3 (lower threshold)
ATR Multiplier: 1.3× (wider stops)
Position Size: -50% (volatility reduction)
```

### **RANGING MARKETS (All Assets):**
```
Signal Sensitivity: 0.0 (default)
Support/Resistance Length: 100 (shorter period)
Exit Momentum: 0.6 (exit quicker)
Position Size: -30% (range reduction)
```

---

## 📊 **MULTI-TIMEFRAME SETUP**

### **Timeframe Combinations:**

**GOLD (GC):**
- Primary: 5-minute (entry/exit)
- Confirmation: 15-minute (trend context)
- Context: 1-hour (major S/R levels)

**NASDAQ (NQ):**
- Primary: 3-minute (faster moves)
- Confirmation: 15-minute (trend context)
- Context: 1-hour (market structure)

**CRUDE OIL (CL):**
- Primary: 5-minute (entry/exit)
- Confirmation: 30-minute (trend context)
- Context: 4-hour (major levels)

**BITCOIN:**
- Primary: 15-minute (entry/exit)
- Confirmation: 1-hour (trend context)
- Context: 4-hour (crypto structure)

---

## 🚨 **SAFETY SETTINGS**

### **Account Protection:**
```
Max Account Risk Per Day: 2%
Max Position Correlation: 3 related positions
Emergency Stop Loss: 5% account drawdown
Component Health Check: Every 100 bars
```

### **Signal Quality Filters:**
```
Minimum Components Agreeing: 2 of 3
Signal Staleness Timeout: 5 bars maximum
Volume Confirmation Required: 1.2× average
Market Regime Filter: Enabled
Time Session Filter: Enabled
```

### **Risk Management:**
```
Use Dynamic Position Sizing: TRUE
Reduce Size on Drawdown: TRUE (50% at soft limits)
Stop Trading on Hard Limits: TRUE
Trail Stops on Profitable Trades: TRUE
```

---

## 🔧 **OPTIMIZATION SCHEDULE**

### **Weekly Reviews:**
- **Monday**: Adjust signal sensitivity based on weekend analysis
- **Wednesday**: Review wave ratio thresholds vs market conditions
- **Friday**: Optimize exit momentum levels for weekend

### **Monthly Deep Reviews:**
- Backtest accelerator multiplier adjustments
- Review and update support/resistance lengths
- Analyze position sizing effectiveness
- Update volatility cluster parameters

---

## ⚠️ **CRITICAL WARNINGS**

### **GOLD Specific:**
- ⚠️ **AVOID**: FOMC meeting days (extreme volatility)
- ⚠️ **WATCH**: DXY correlation (typically inverse)
- ⚠️ **CAUTION**: Thin Asian session liquidity

### **NASDAQ Specific:**
- ⚠️ **AVOID**: First 5 minutes of market open
- ⚠️ **WATCH**: FAANG earnings season impact
- ⚠️ **RISK**: Overnight gap risk on positions

### **CRUDE OIL Specific:**
- ⚠️ **AVOID**: API Tuesday evening reports
- ⚠️ **WATCH**: OPEC meeting announcements
- ⚠️ **VOLUME**: Lower after-hours liquidity

### **BITCOIN Specific:**
- ⚠️ **VOLATILITY**: Extreme weekend price swings
- ⚠️ **CORRELATION**: Watch spot vs futures divergence
- ⚠️ **TIMING**: Asian session typically most active

---

*Remember: These parameters are starting points. Track your results and adjust based on actual performance. Markets evolve - your settings should too.*
