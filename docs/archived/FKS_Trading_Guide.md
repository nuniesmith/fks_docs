# FKS Trading Systems - Complete Guide

## 🎯 **SYSTEM OVERVIEW**

FKS is a bulletproof trading system that combines AI-enhanced signals with traditional technical analysis for futures trading. The system uses three core components that must agree before generating trades.

### **Core Components:**
- **FKS_AI**: Support/resistance detection with signal quality scoring
- **FKS_AO**: Awesome Oscillator with momentum confirmation  
- **FKS_Dashboard**: Market regime and performance dashboard

---

## 📊 **SIGNAL HIERARCHY (Trade Only These)**

### 🟢 **TIER 1 - PREMIUM SIGNALS** (4-5 contracts)
```
LONG:  "G" signal + AO cross > 0 + Quality > 85% + Wave > 2.0x
SHORT: "Top" signal + AO cross < 0 + Quality > 85% + Wave > 2.0x
```

### 🟡 **TIER 2 - STRONG SIGNALS** (2-3 contracts)  
```
LONG:  "G" or "^" + AO > signal line + Quality 70-85% + Wave 1.5-2.0x
SHORT: "Top" or "v" + AO < signal line + Quality 70-85% + Wave 1.5-2.0x
```

### ⚪ **TIER 3 - STANDARD SIGNALS** (1-2 contracts)
```
LONG:  "^" + AO bullish + Quality 60-70% + Wave > 1.5x
SHORT: "v" + AO bearish + Quality 60-70% + Wave > 1.5x
```

---

## 🎯 **THE 4 BULLETPROOF SETUPS**

### **Setup 1: EMA9 + VWAP Bullish Breakout**
**Entry Conditions (ALL must be met):**
- Price > EMA9 > VWAP (stacked alignment)
- "G" signal appears at support level
- AO crosses above zero OR shows bullish momentum
- Volume > 1.2x average
- Signal quality ≥ 65%

**Entry Trigger:** Price breaks above recent swing high with volume

### **Setup 2: EMA9 + VWAP Bearish Breakdown**
**Entry Conditions (ALL must be met):**
- Price < EMA9 < VWAP (bearish stack)
- "Top" signal appears at resistance level  
- AO crosses below zero OR shows bearish momentum
- Volume > 1.2x average
- Signal quality ≥ 65%

**Entry Trigger:** Price breaks below recent swing low with volume

### **Setup 3: VWAP Rejection Bounce**
**Entry Conditions:**
- Price approaches VWAP and bounces with "G" signal
- AO shows bullish divergence or momentum
- Strong support confluence
- High-quality rejection candle (hammer, pin bar)

### **Setup 4: Support/Resistance + AO Zero Cross**
**Entry Conditions:**
- Price at key S/R level from FKS_AI
- AO crosses zero line (bullish/bearish)
- Signal quality > 70%
- Clear breakout with volume confirmation

---

## 📈 **POSITION SIZING MATRIX**

| Signal Quality | Wave Ratio | Market Regime | GC | NQ | CL | BTC |
|---------------|------------|---------------|----|----|----|----- |
| **85%+** | **>2.0x** | **TRENDING** | 4-5 | 3-4 | 4-5 | 3 |
| **70-85%** | **1.5-2.0x** | **TRENDING** | 2-3 | 2 | 2-3 | 2 |
| **60-70%** | **>1.5x** | **TRENDING** | 1-2 | 1 | 1-2 | 1 |
| **Any** | **Any** | **VOLATILE** | -50% | -50% | -50% | -50% |
| **Any** | **Any** | **RANGING** | -30% | -30% | -30% | -30% |

---

## 🛑 **RISK MANAGEMENT RULES**

### **Stop Loss Calculation:**
```
Stop = Entry ± (ATR × Volatility Multiplier × Market Phase Multiplier)
```

**Volatility Multipliers:**
- 🔴 HIGH = 0.8× (tighter stops)
- 🟡 MEDIUM = 1.0× (normal stops)  
- 🟢 LOW = 1.2× (wider stops)

**Market Phase Multipliers:**
- TREND = 1.5× (more room)
- RANGE = 0.8× (tight stops)

### **Profit Targets:**
```
T1 (50% exit): 2× risk or middle band
T2 (25% exit): 3-4× risk or opposite band  
T3 (25% trail): EMA9 trail or momentum < 0.7
```

### **Daily Limits:**
```
PROFIT LIMITS:          LOSS LIMITS:
✅ < $2,000 = NORMAL    ✅ < $1,000 = NORMAL
⚠️ $2,000+ = REDUCE 50% ⚠️ $1,000+ = REDUCE 50%
🛑 $3,000 = STOP        🛑 $1,500 = STOP
```

---

## ⏰ **TRADING SCHEDULE**

### **Active Trading Hours:**
- **Morning Session**: 9:30-11:30 AM EST
- **Afternoon Session**: 1:30-3:30 PM EST

### **Avoid Trading:**
- First/last 15 minutes of session
- 30 minutes before/after major news
- FOMC days (for Gold)
- EIA inventory days (for Crude Oil)

---

## 🎪 **MARKET-SPECIFIC NOTES**

### **Gold (GC):**
- **Best Hours**: London/NY overlap (8AM-12PM EST)
- **Watch**: DXY inverse correlation
- **Typical Range**: $10-20/day
- **Stop Range**: $5-10 ($500-1000 risk)

### **Nasdaq (NQ):**
- **Best Hours**: First hour & last hour  
- **Watch**: VIX for volatility
- **Typical Range**: 100-200 pts/day
- **Stop Range**: 20-40 pts ($400-800 risk)

### **Crude Oil (CL):**
- **Best Hours**: 9AM-2:30PM EST
- **Watch**: Wednesday 10:30 AM EIA report
- **Typical Range**: $1-2/day
- **Stop Range**: $0.30-0.50 ($300-500 risk)

### **Bitcoin Futures:**
- **Best Hours**: US/Asia overlap (8PM-2AM EST)
- **Watch**: Spot price divergence
- **Typical Range**: 3-5% daily
- **Stop Range**: 1-2% of price

---

## ⚠️ **WARNING SIGNALS (Exit Immediately)**

### 🚨 **IMMEDIATE EXIT TRIGGERS:**
- Opposite "G" or "Top" signal appears
- Signal quality drops below 40%
- AO divergence (price up, AO down)
- Market regime changes mid-trade

### ⚡ **REDUCE POSITION TRIGGERS:**
- Wave ratio drops below 1.0x
- Momentum ratio < 0.7 (weakening)
- Volatility cluster shifts to HIGH
- Multiple consecutive losses (3+)

---

## 🔄 **DAILY WORKFLOW**

### **Pre-Market (30 min before open):**
1. Check market regime status
2. Note volatility level & ATR
3. Identify key S/R levels
4. Review economic calendar

### **During Session:**
1. Wait for signals at S/R zones
2. Confirm with AO momentum  
3. Verify signal quality ≥ 65%
4. Check wave ratio > 1.5x
5. Calculate position size
6. Execute with predetermined stops

### **End of Day:**
1. Log all trades and reasoning
2. Review signal quality vs results
3. Check P&L against daily limits
4. Plan tomorrow's key levels

---

## 💡 **SUCCESS PRINCIPLES**

1. **Quality > Quantity**: 3 good trades beat 10 mediocre ones
2. **Trust the Weather**: Color gradient shows market temperature
3. **Wave Ratio is King**: Below 1.5x = skip the trade
4. **Respect the Regime**: Don't force trades in NEUTRAL markets
5. **Honor the Limits**: Hard stops protect accounts

---

## 🎯 **PERFORMANCE TARGETS**

### **Weekly Goals:**
- **Win Rate**: >55% (with proper filtering)
- **Risk/Reward**: Average >2:1
- **Max Drawdown**: <5% of account
- **Trade Frequency**: 10-15 quality setups per week

### **Monthly Targets:**
- **Return**: 8-15% with risk management
- **Sharpe Ratio**: >2.0
- **Max Consecutive Losses**: <3
- **Signal Quality**: >70% of trades from Tier 1-2 setups

---

*"The FKS system shows you WHERE (S/R), WHEN (signals), and HOW STRONG (quality %). Your job is position sizing and discipline."*
