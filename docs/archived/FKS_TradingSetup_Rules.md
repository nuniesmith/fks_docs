# FKS Trading Setup Rules - Bulletproof Signal Generation

## Core Trading Setups

### **Setup 1: EMA9 + VWAP Bullish Breakout**

**Entry Conditions:**

- Price > EMA9 > VWAP (stacked alignment)
- AO showing bullish momentum (rising above zero OR bullish saucer)
- FKS_AI Support/Resistance shows price near support level
- Volume > 1.2x average (confirmation)
- RSI between 40-70 (not overbought)

**Entry Trigger:**

- Price breaks above recent swing high with volume
- AO confirms with green bar or zero-line cross

**Exit Rules:**

- Stop: Below EMA9 or support level (whichever is closer)
- Target 1: Next resistance level or 2x ATR
- Target 2: Major resistance or 3x ATR

### **Setup 2: EMA9 + VWAP Bearish Breakdown**

**Entry Conditions:**

- Price < EMA9 < VWAP (bearish stack)
- AO showing bearish momentum (falling below zero OR bearish saucer)
- FKS_AI shows price near resistance level
- Volume > 1.2x average
- RSI between 30-60 (not oversold)

**Entry Trigger:**

- Price breaks below recent swing low with volume
- AO confirms with red bar or zero-line cross

**Exit Rules:**

- Stop: Above EMA9 or resistance level
- Target 1: Next support level or 2x ATR
- Target 2: Major support or 3x ATR

### **Setup 3: VWAP Rejection Setup**

**Entry Conditions:**

- Price approaches VWAP from above/below
- AO shows divergence or momentum exhaustion
- FKS_AI shows confluence with support/resistance
- Volume spikes on rejection

**Entry Logic:**

- Long: Price bounces off VWAP from above, AO shows bullish divergence
- Short: Price rejected at VWAP from below, AO shows bearish divergence

### **Setup 4: AO Twin Peaks/Saucers**

**Entry Conditions:**

- AO forms twin peaks (bearish) or saucer (bullish)
- Price confirms with break of structure
- EMA9 and VWAP alignment supports direction
- Clean support/resistance levels identified

## Signal Filtering Rules

### **Market Condition Filters:**

1. **Trending Markets** (ADX > 25):
   - Only trade in direction of EMA9 slope
   - Require VWAP alignment
   - Higher confidence threshold (0.7+)

2. **Ranging Markets** (ADX < 20):
   - Focus on support/resistance bounces
   - Require strong AO confirmation
   - Lower position sizes

3. **High Volatility** (ATR > 150% of 20-period average):
   - Wider stops (2.5x ATR)
   - Reduce position size by 50%
   - Require stronger confluence

### **Time-Based Filters:**

- **Active Hours**: 9:30-11:30 AM, 1:30-3:30 PM EST
- **Avoid**: First/last 15 minutes of session
- **News Events**: No trades 30 min before/after major releases

### **Component Agreement Rules:**

- **Minimum 2 of 3 components** must agree (AI, AO, confluence)
- **Confidence threshold**: 0.65 for regular trades, 0.8 for strong trades
- **Signal timeout**: Maximum 3 bars old

## Risk Management Rules

### **Position Sizing:**

```
Base Size = Account * Risk% / (Entry - Stop) / Point Value
Max Size = 3 contracts
Volatility Adjustment = Base Size * (1 - (ATR Ratio - 1))
```

### **Stop Loss Placement:**

1. **Technical Stops**: Below/above key levels (EMA9, S/R, swing points)
2. **ATR Stops**: 1.5x ATR in normal volatility, 2.5x in high volatility
3. **Time Stops**: Exit if no movement in 5 bars
4. **Use tightest of the three**

### **Profit Targets:**

- **Target 1**: 1.5-2x risk (partial exit 50%)
- **Target 2**: 3x risk (remaining position)
- **Trailing Stop**: Move to breakeven after 1x ATR profit

### **Daily Limits:**

- **Max Trades**: 5 per day
- **Max Risk**: 2% of account per day
- **Max Consecutive Losses**: 3 (then reduce size by 50%)

## Quality Control Checklist

### **Before Entry:**

- [ ] All 3 components show agreement
- [ ] Market condition identified correctly
- [ ] Time/session filter passed
- [ ] Confluence level identified
- [ ] Risk/reward minimum 1:2
- [ ] Position size calculated correctly

### **During Trade:**

- [ ] Monitor for invalidation signals
- [ ] Track against profit targets
- [ ] Adjust stops as planned
- [ ] Watch for early exit signals

### **After Trade:**

- [ ] Record trade details and reasoning
- [ ] Analyze what worked/didn't work
- [ ] Update signal strength scoring
- [ ] Adjust parameters if needed

## Implementation Priority

### **Immediate (Week 1):**

1. Implement proper EMA9 calculation (not SMA)
2. Add VWAP alignment checks
3. Fix AO cross detection logic
4. Add basic confluence scoring

### **Short-term (Week 2-3):**

1. Implement market condition detection
2. Add time-based filtering
3. Improve stop/target logic
4. Add position sizing adjustments

### **Medium-term (Week 4-6):**

1. Add pattern recognition improvements
2. Implement adaptive parameters
3. Add advanced risk management
4. Performance tracking enhancements
