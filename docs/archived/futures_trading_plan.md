# FKS Futures Trading Plan
## Gold (GC), Nasdaq (NQ), Crude Oil (CL), Bitcoin Futures

---

## 1. TRADING SYSTEM OVERVIEW

### Core Indicators Setup (NinjaTrader 8)
1. **FKS_AI (Primary)** - Main indicator combining:
   - Dynamic Support/Resistance bands (Top G logic)
   - Middle band (HMA-smoothed midpoint)
   - EMA 9
   - VWAP
   - Signal quality scoring system

2. **FKS_AO (Confirmation)** - Awesome Oscillator for momentum confirmation
   - Fast period: 5
   - Slow period: 34
   - Signal period: 7

### Signal Quality Requirements
- **Minimum Signal Quality Score**: 60% (0.6)
- **Wave Ratio Requirement**: ≥ 1.5x for entries
- **Exit Momentum Threshold**: 0.7 (70% of peak momentum)

---

## 2. MARKET REGIME ASSESSMENT

### Pre-Trading Checklist
1. **Market Regime Check** (from FKS indicator):
   - ✅ TRENDING BULL/BEAR: Full position sizing allowed
   - ⚠️ VOLATILE: Reduce position size by 50%
   - ⚠️ RANGING: Reduce position size by 30%
   - ❌ NEUTRAL: Skip trading or minimum positions only

2. **Volatility Assessment**:
   - Check volatility percentile (0.2 - 0.8 acceptable range)
   - K-means cluster status (LOW/MEDIUM preferred, HIGH = caution)
   - ATR-based stop placement adjusted by volatility regime

3. **Session Filter**:
   - Gold (GC): Focus on London/NY overlap (8 AM - 12 PM EST)
   - NQ: US session (9:30 AM - 4 PM EST)
   - CL: Watch for inventory reports (10:30 AM EST Wed)
   - BTC: 24/7 but focus on US/Asia overlap

---

## 3. ENTRY CRITERIA

### LONG ENTRY CONDITIONS (All must be met):

1. **Price Location**:
   - Price touches or bounces from support band (solid lower line)
   - OR price near support zone (within 0.5 × adaptive ATR)

2. **Pattern Confirmation**:
   - "G" signal appears (strong support bounce)
   - OR "^" signal (simple support bounce)
   - OR bullish candlestick pattern (hammer, bullish engulfing)

3. **Momentum Confirmation**:
   - AO crosses above zero OR
   - AO > signal line with increasing histogram
   - Wave ratio > 1.5x (current wave vs average)

4. **Signal Quality**:
   - Signal quality score ≥ 60%
   - Trend speed showing acceleration (color brightening)
   - Weather indicator showing warming (yellow → red transition)

5. **Multi-Timeframe Alignment**:
   - Higher timeframe S/R bias confirms direction
   - Price above VWAP for intraday longs

### SHORT ENTRY CONDITIONS (All must be met):

1. **Price Location**:
   - Price touches or rejects from resistance band (dotted upper line)
   - OR price near resistance zone (within 0.5 × adaptive ATR)

2. **Pattern Confirmation**:
   - "Top" signal appears (strong resistance rejection)
   - OR "˅" signal (simple resistance rejection)
   - OR bearish candlestick pattern (shooting star, bearish engulfing)

3. **Momentum Confirmation**:
   - AO crosses below zero OR
   - AO < signal line with decreasing histogram
   - Wave ratio > 1.5x (current wave vs average)

4. **Signal Quality**:
   - Signal quality score ≥ 60%
   - Trend speed showing deceleration (color dimming)
   - Weather indicator showing cooling (yellow → blue transition)

5. **Multi-Timeframe Alignment**:
   - Higher timeframe S/R bias confirms direction
   - Price below VWAP for intraday shorts

---

## 4. POSITION SIZING STRATEGY

### Base Position Sizing (per $150,000 capital):

#### Gold (GC) - $10/point move
- **Normal Setup (Quality 60-70%)**: 1-2 contracts
- **Strong Setup (Quality 70-85%)**: 2-3 contracts
- **Exceptional Setup (Quality >85%)**: 4-5 contracts

#### Nasdaq (NQ) - $20/point move
- **Normal Setup**: 1 contract
- **Strong Setup**: 2 contracts
- **Exceptional Setup**: 3-4 contracts

#### Crude Oil (CL) - $10/tick move
- **Normal Setup**: 1-2 contracts
- **Strong Setup**: 2-3 contracts
- **Exceptional Setup**: 4-5 contracts

#### Bitcoin Futures - Varies by contract
- **Normal Setup**: 1 micro contract
- **Strong Setup**: 2 micro contracts
- **Exceptional Setup**: 3 micro contracts

### Position Size Adjustments:
- **Volatile Market Regime**: -50% contracts
- **Ranging Market**: -30% contracts
- **High Volatility Cluster**: -40% contracts
- **Perfect Multi-TF Alignment**: +1 contract bonus

---

## 5. STOP LOSS PLACEMENT

### Dynamic Stop Loss Calculation:
```
Stop Loss = Entry ± (ATR × Volatility Multiplier × Phase Multiplier × Regime Modifier)
```

#### Multiplier Values:
- **Volatility Multiplier**: 
  - High vol: 0.8
  - Medium vol: 1.0
  - Low vol: 1.2

- **Phase Multiplier**:
  - Trending: 1.5
  - Accumulation/Distribution: 1.2
  - Neutral: 1.0

- **Regime Modifier**:
  - Trending markets: 1.2
  - Volatile markets: 1.5
  - Ranging markets: 0.8

### Stop Loss Guidelines by Market:
- **GC**: Typically $5-10 from entry
- **NQ**: Typically 20-40 points from entry
- **CL**: Typically $0.30-0.50 from entry
- **BTC**: Typically 1-2% from entry

---

## 6. PROFIT TARGET STRATEGY

### Two-Tier Exit System:

#### Target 1 (50% of position):
- **Price Target**: Middle band of FKS indicator
- **Alternative**: 2R (2× initial risk)
- **Action**: Move stop to breakeven on remaining position

#### Target 2 (remaining 50%):
- **Price Target**: Opposite band (resistance for longs, support for shorts)
- **Alternative**: 3-4R or when momentum ratio drops below 0.7
- **Trailing Stop**: Use EMA9 as dynamic trailing stop

### Exit Signals to Watch:
- Opposite "Top" or "G" signal appears
- AO momentum divergence
- Wave ratio drops below exit threshold (0.7)
- Weather indicator shows extreme reversal
- Signal quality drops below 40%

---

## 7. RISK MANAGEMENT RULES

### Daily Loss Limits:
- **Soft Stop**: -$1,000 (reduce position size by 50%)
- **Hard Stop**: -$1,500 (close all positions, stop trading)

### Daily Profit Targets:
- **Soft Target**: +$2,000 (consider reducing size or tightening stops)
- **Hard Target**: +$3,000 (close all positions, stop trading)

### Maximum Exposure Rules:
- Never exceed 15 contracts total across all positions
- Maximum 5 contracts per position (even on exceptional setups)
- No more than 3 correlated positions open simultaneously

### Risk Per Trade:
- Target Risk: 0.5-0.7% of capital per trade ($750-$1,050)
- Maximum Risk: 1% of capital per trade ($1,500)
- Minimum R:R ratio: 2:1 (prefer 3:1 or better)

---

## 8. TRADE MANAGEMENT

### Entry Execution:
1. Wait for signal candle to close (avoid false signals)
2. Enter on next candle open or market order
3. Place stop loss immediately
4. Set target orders (OCO bracket)

### Active Position Management:
1. **Momentum Monitoring**:
   - Watch wave ratio for strength/weakness
   - Monitor AO histogram changes
   - Track signal quality score

2. **Breakeven Management**:
   - Move to BE after 1R profit
   - OR when price reaches middle band

3. **Partial Profits**:
   - Take 25% at 1R if momentum weakening
   - Take 50% at target 1
   - Trail remainder with EMA9 or momentum exit

### Time-Based Exits:
- If no movement within 2 hours: reassess
- End-of-session: close if not in profit
- Friday afternoon: reduce or close positions

---

## 9. TRADING JOURNAL REQUIREMENTS

### Track for Each Trade:
1. **Entry Data**:
   - Signal type (G, Top, simple)
   - Signal quality score
   - Wave ratio at entry
   - Market regime
   - Volatility status

2. **Performance Metrics**:
   - Entry/exit prices and times
   - R:R achieved
   - Maximum favorable/adverse excursion
   - Reason for exit

3. **Weekly Review**:
   - Win rate by signal type
   - Average R:R by market regime
   - Performance by time of day
   - Correlation with signal quality scores

---

## 10. CONTINUOUS IMPROVEMENT

### Weekly Analysis Tasks:
1. Review all trades with signal quality < 70% 
2. Identify best performing market regimes
3. Optimize position sizing based on results
4. Adjust wave ratio thresholds if needed

### Monthly Optimization:
1. Backtest parameter adjustments
2. Review volatility cluster performance
3. Analyze multi-timeframe alignment success
4. Update position sizing matrix

### Key Performance Indicators:
- Target: 50%+ win rate
- Average winner: 2.5R or better
- Profit factor: > 1.5
- Maximum drawdown: < 5% of capital

---

## QUICK REFERENCE CHECKLIST

### Pre-Trade Checklist:
- [ ] Market regime favorable?
- [ ] Volatility within acceptable range?
- [ ] Clear S/R level identified?
- [ ] Signal appeared (G/Top)?
- [ ] AO confirmation present?
- [ ] Signal quality ≥ 60%?
- [ ] Wave ratio ≥ 1.5x?
- [ ] Position size calculated?
- [ ] Stop loss determined?
- [ ] R:R ratio ≥ 2:1?

### Emergency Procedures:
1. **System Failure**: Close all positions at market
2. **Unexpected Volatility**: Reduce to 1 contract max
3. **News Event**: Exit before high-impact news
4. **Technical Issues**: Have backup broker access
5. **Daily Limit Hit**: Stop immediately, review tomorrow

---

*Remember: This system combines discretionary pattern recognition with systematic signal confirmation. Trust the signals but always maintain situational awareness. The FKS indicator provides the edge - your discipline in following the plan ensures profitability.*