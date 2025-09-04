# FKS Trading Systems - Quick Reference Card

## 🌅 **DAILY STARTUP CHECKLIST**

```
□ 07:45 AM - Start Python server
  └─ python3 fks_server.py
  
□ 07:50 AM - Open NinjaTrader
  └─ Load "FKS_Production" workspace
  
□ 07:55 AM - Pre-market checks
  ├─ Data connection: Green ✓
  ├─ All components: CONNECTED
  ├─ Python API: Connected
  └─ Yesterday's P&L noted
  
□ 08:00 AM - Enable strategy
  └─ Strategies tab → FKS_Strategy → Enable
```

---

## 📊 **KEY METRICS TO MONITOR**

### **Dashboard Quick Check:**
```
TREND INFO
├─ Direction: [BULL/BEAR/NEUTRAL]
├─ Signal: [Current signal or "None"]
└─ Quality: [>65% = Good]

PERFORMANCE  
├─ Daily P&L: [Target: +$2,250]
├─ Trades: [X/6 max]
└─ Win Rate: [Target: >60%]

RISK STATUS
├─ Soft Limit: $2,250 profit
└─ Hard Limit: $3,000 loss
```

---

## 🚦 **SIGNAL QUALITY GUIDE**

| Quality | Action | Position Size |
|---------|--------|--------------|
| 85-100% | **TAKE** | 4-5 contracts |
| 70-85%  | **TAKE** | 2-3 contracts |
| 65-70%  | **TAKE** | 1-2 contracts |
| <65%    | **SKIP** | 0 contracts |

---

## ⚡ **QUICK COMMANDS**

### **Enable/Disable Trading:**
```
F5 = Enable Strategy
F6 = Disable Strategy
Ctrl+F = Flatten Everything
```

### **Chart Controls:**
```
Space = Crosshair
Ctrl+R = Reload NinjaScript
F5 (on chart) = Refresh
```

### **Emergency Stop:**
```
1. Strategies Tab → Disable
2. Ctrl+F (Flatten)
3. Cancel all orders
```

---

## 🎯 **THE 4 SETUPS AT A GLANCE**

### **Setup 1: Bullish Breakout**
```
✓ Price > EMA9 > VWAP
✓ "G" signal appears
✓ AO turns positive
✓ Volume > 1.2x avg
```

### **Setup 2: Bearish Breakdown**
```
✓ Price < EMA9 < VWAP
✓ "Top" signal appears
✓ AO turns negative
✓ Volume > 1.2x avg
```

### **Setup 3: VWAP Bounce**
```
✓ Price touches middle band
✓ Rejection candle forms
✓ Signal appears (any)
✓ AO confirms direction
```

### **Setup 4: S/R + AO Cross**
```
✓ At support/resistance
✓ AO crosses zero
✓ Quality > 70%
✓ Volume surge
```

---

## 📈 **POSITION MANAGEMENT**

### **Entry Rules:**
- Signal quality ≥ 65%
- Components aligned
- Risk limits OK
- Optimal session time

### **Exit Rules:**
- Stop: 2 ATR (×1.5 if volatile)
- Target: 3 ATR minimum
- Trail: After 1:1 R:R
- Time: Close by 3:45 PM

---

## 🕐 **OPTIMAL TRADING TIMES**

### **Gold (GC):**
```
BEST:  08:00 - 12:00 EST
GOOD:  12:00 - 14:00 EST  
AVOID: After 15:00 EST
```

### **ES/NQ:**
```
BEST:  09:30 - 11:30 EST
GOOD:  14:00 - 15:30 EST
AVOID: 12:00 - 13:00 EST (lunch)
```

---

## 🔍 **QUICK DIAGNOSTICS**

### **No Trades?**
1. Check signal quality threshold
2. Verify time filter
3. Review daily limits
4. Check component health

### **Poor Performance?**
1. Review signal quality average
2. Check market regime
3. Verify setup distribution
4. Analyze time of day

---

## 📊 **END OF DAY ROUTINE**

```
□ 3:45 PM - Close all positions
  └─ Strategy handles automatically
  
□ 4:00 PM - Note final P&L
  ├─ Screenshot dashboard
  └─ Export today's trades
  
□ 4:15 PM - Disable strategy
  └─ Prepare for overnight
  
□ 4:30 PM - Review session
  ├─ Best/worst trades
  ├─ Signal quality average
  └─ Tomorrow's plan
```

---

## 🚨 **WARNING SIGNS**

**Stop Trading If:**
- 3 consecutive losses ❌
- Daily loss > $2,500 ❌
- Components disconnected ❌
- Unusual market behavior ❌
- Technical issues ❌

---

## 💡 **PRO TIPS**

1. **Quality > Quantity**
   - Better 2 great trades than 6 mediocre

2. **Trust the System**
   - Don't override signals
   - Let stops work

3. **Market Awareness**
   - Check regime on dashboard
   - Note volatility status

4. **Session Discipline**
   - Best trades: First 4 hours
   - Avoid afternoon chop

---

## 📱 **QUICK STATS CHECK**

```python
# Terminal command for stats:
curl http://localhost:5000/api/stats | python -m json.tool

# Returns:
{
  "signals": {
    "total": 24,
    "avg_quality": 0.72
  },
  "trades": {
    "total": 6,
    "total_pnl": 2340.0,
    "win_rate": 0.67
  }
}
```

---

## 🎯 **REMEMBER**

**The 3 Rules:**
1. **Signal Quality ≥ 65%**
2. **Respect Risk Limits**
3. **Trade Optimal Hours**

**Daily Goals:**
- Target: $2,250 (1.5%)
- Max Loss: $3,000 (2%)
- Trades: 2-4 quality setups

**Your Edge:**
- Systematic approach
- Quality filtering
- Risk management
- No emotions

---

**Keep this card visible during trading!** 📌