# FKS Trading Systems - Comprehensive Troubleshooting Guide

## 🚨 **COMMON ISSUES AND SOLUTIONS**

### **1. COMPILATION ERRORS**

#### **Error: "The type or namespace name 'FKSCore' could not be found"**
```csharp
// Problem: FKS_Core not loaded first
// Solution: Ensure file order in .csproj:
<Compile Include="AddOns\FKS_Core.cs" />  <!-- MUST BE FIRST -->
<Compile Include="AddOns\FKS_Infrastructure.cs" />
<Compile Include="AddOns\FKS_Market.cs" />
<!-- Then indicators/strategies -->
```

#### **Error: "Cannot implicitly convert type"**
```csharp
// Problem: Property type mismatch
// Solution: Check property declarations match interface
public string SignalType => currentSignalType;  // Not string SignalType { get; set; }
```

#### **Error: "Method not found"**
```csharp
// Problem: Old method signatures
// Solution: Update to new signatures
// Old: fksAI.GetSignalType()
// New: fksAI.SignalType  // Property access
```

---

### **2. RUNTIME ERRORS**

#### **Issue: "Object reference not set to an instance of an object"**
```csharp
// Common locations and fixes:

// 1. Indicator not initialized
if (fksAI == null)
{
    Log("FKS_AI indicator not found", LogLevel.Error);
    return;
}

// 2. Series not initialized
if (CurrentBar < BarsRequiredToTrade) return;

// 3. Component not registered
try
{
    FKSCore.RegisterComponent(ComponentId, this);
}
catch (Exception ex)
{
    // Continue without registration
}
```

#### **Issue: "Index was outside the bounds of the array"**
```csharp
// Problem: Accessing historical data too early
// Solution: Add guards
if (CurrentBar < lookbackPeriod) return;

// Check before accessing
if (bullishWaves.Count > 0)
{
    double avg = bullishWaves.Average();
}
```

---

### **3. INDICATOR DISPLAY ISSUES**

#### **Problem: Indicators Not Showing**

**Checklist:**
```
□ Right panel selected? (Price panel vs new panel)
□ Sufficient historical data loaded?
□ Calculate = OnBarClose or OnPriceChange?
□ IsOverlay = true for price panel indicators?
□ Clean Chart Mode disabled?
```

**Debug Steps:**
```csharp
// Add debug output in OnBarUpdate
protected override void OnBarUpdate()
{
    Print($"FKS_AI Bar {CurrentBar}: Signal={currentSignalType}");
    // If no output, indicator not running
}
```

#### **Problem: Dashboard Not Visible**

**Solutions:**
1. **Check position settings:**
   ```
   Dashboard Position: Top Right
   X Offset: 10 (increase if cut off)
   Y Offset: 10
   ```

2. **Verify rendering:**
   ```csharp
   // In OnRender method
   if (RenderTarget == null)
   {
       Print("RenderTarget is null");
       return;
   }
   ```

3. **Chart scaling issues:**
   ```
   Right-click chart → Properties → 
   Scale: Fixed
   Margins: Increase right margin
   ```

---

### **4. STRATEGY NOT TAKING TRADES**

#### **Diagnostic Flowchart:**

```
Strategy Enabled? ──No──> Enable in Strategies tab
       │
      Yes
       ↓
Account Connected? ──No──> Connect to data feed
       │
      Yes
       ↓
Signals Generated? ──No──> Check indicators loaded
       │
      Yes
       ↓
Quality > Threshold? ──No──> Lower threshold or wait
       │
      Yes
       ↓
Risk Limits OK? ──No──> Check daily P&L limits
       │
      Yes
       ↓
Time Filter OK? ──No──> Check session times
       │
      Yes
       ↓
Should Execute Trade
```

#### **Debug Output Analysis:**

Enable debug mode and check Output window:

```
// Good sequence:
[09:30:00] Signal generated: G | Quality: 72%
[09:30:01] Setup 1 detected
[09:30:02] Position size: 2 contracts
[09:30:03] Order submitted: Buy 2 GC

// Problem sequences:
[09:30:00] Signal generated: G | Quality: 55%
[09:30:01] Signal rejected - quality below threshold

[14:00:00] Max daily trades reached
[14:00:01] Trading disabled for today
```

---

### **5. PYTHON BRIDGE CONNECTION ISSUES**

#### **Problem: "Python API: Disconnected"**

**Step-by-step fix:**

1. **Check Python server running:**
   ```bash
   # Terminal 1: Start server
   cd /home/ordan/fks/python
   python3 fks_server.py
   
   # Should see:
   # Starting FKS Python Server on http://localhost:5000
   ```

2. **Test connection manually:**
   ```bash
   # Terminal 2: Test endpoint
   curl http://localhost:5000/api/health
   
   # Should return:
   # {"status":"healthy","timestamp":"2025-07-07T10:30:00"}
   ```

3. **Check firewall:**
   ```bash
   # Allow local connections
   sudo ufw allow from 127.0.0.1 to any port 5000
   ```

4. **Verify in NinjaTrader:**
   ```
   FKS_PythonBridge settings:
   - Enabled: ✓
   - API Endpoint: http://localhost:5000/api
   - Show Connection Status: ✓
   ```

#### **Problem: "Failed to send logs"**

**Common causes:**
- Queue overflow (check batch size)
- Server timeout (increase timeout)
- Invalid JSON data

**Solution:**
```csharp
// In FKS_PythonBridge settings:
Batch Size: 50 → 20  // Reduce if failing
Connection Timeout: 5000 → 10000  // Increase timeout
```

---

### **6. PERFORMANCE ISSUES**

#### **Problem: Strategy Running Slowly**

**Optimization checklist:**

1. **Reduce indicator calculations:**
   ```csharp
   // Bad: Calculate every bar
   if (CurrentBar % 5 == 0)  // Good: Calculate every 5 bars
   {
       UpdateComplexCalculation();
   }
   ```

2. **Disable unnecessary visuals:**
   ```
   Clean Chart Mode: True
   Show Entry Zones: False
   Show Wave Info: False
   ```

3. **Optimize data series:**
   ```csharp
   // Don't store unnecessary history
   if (signalHistory.Count > 100)
       signalHistory.RemoveAt(0);
   ```

#### **Problem: Memory Usage Growing**

**Common memory leaks:**

1. **Drawing objects:**
   ```csharp
   // Bad: Creates new object every bar
   Draw.Text(this, "Signal" + CurrentBar, ...);
   
   // Good: Reuse tag
   Draw.Text(this, "CurrentSignal", ...);
   ```

2. **Event handlers:**
   ```csharp
   // Always unsubscribe in State.Terminated
   protected override void OnStateChange()
   {
       if (State == State.Terminated)
       {
           UnsubscribeFromEvents();
       }
   }
   ```

---

### **7. SIGNAL QUALITY ISSUES**

#### **Problem: Low Quality Signals**

**Diagnostic process:**

1. **Check market conditions:**
   ```
   Dashboard → Market Regime
   - VOLATILE: Expect lower quality
   - RANGING: Expect fewer signals
   - TRENDING: Best quality signals
   ```

2. **Verify component alignment:**
   ```
   Required for high quality:
   □ AI Signal present (G/Top)
   □ AO confirmation (same direction)
   □ Wave ratio > 1.5
   □ Volume > 1.2x average
   □ Optimal session time
   ```

3. **Review threshold:**
   ```csharp
   // Temporary quality analysis
   if (currentSignalQuality > 0)
   {
       Print($"Signal Quality: {currentSignalQuality:P} | " +
             $"Trend: {trendScore:P} | " +
             $"Momentum: {momentumScore:P} | " +
             $"Wave: {waveScore:P}");
   }
   ```

---

### **8. DATA FEED ISSUES**

#### **Problem: "No data received"**

**Solutions by connection type:**

1. **CQG/Rithmic:**
   ```
   Tools → Options → Market Data
   - Primary: CQG/Rithmic
   - Backup: Kinetick
   - Historical: Check "Download missing"
   ```

2. **Playback/Simulation:**
   ```
   Market Replay → Download data
   Playback → Ensure speed not too fast
   ```

3. **Live data gaps:**
   ```csharp
   // Add data validation
   if (Close[0] <= 0 || Volume[0] <= 0)
   {
       Log("Invalid data received", LogLevel.Warning);
       return;
   }
   ```

---

### **9. MULTI-ACCOUNT ISSUES**

#### **Problem: Trades Not Syncing Across Accounts**

**Setup verification:**

1. **Account configuration:**
   ```
   Each instance needs:
   - Unique workspace
   - Same strategy settings
   - Different account selected
   ```

2. **Position sizing:**
   ```csharp
   // Ensure consistent sizing
   if (Account.Name.Contains("Sim"))
       baseContracts = 1;  // Test with smaller size
   ```

3. **Synchronization:**
   ```
   Tools → Options → Strategies
   □ Sync account position
   □ Submit stop strategy
   ```

---

### **10. EMERGENCY PROCEDURES**

#### **STOP ALL TRADING IMMEDIATELY:**

```
Method 1: Strategy Tab
- Strategies tab → Disable all

Method 2: Account
- Control Center → Flatten Everything

Method 3: Close NinjaTrader
- File → Exit (saves state)
```

#### **RECOVERY AFTER CRASH:**

1. **Check positions:**
   ```
   Control Center → Positions tab
   - Note any open positions
   - Check for orphaned orders
   ```

2. **Restart sequence:**
   ```
   1. Start NinjaTrader
   2. Cancel all orders
   3. Load saved workspace
   4. Verify indicators loaded
   5. Re-enable strategy
   ```

3. **Verify state:**
   ```
   Dashboard should show:
   - All components: CONNECTED
   - Correct daily P&L
   - Accurate position count
   ```

---

## 🛠️ **DIAGNOSTIC TOOLS**

### **1. Component Health Check Script**

Add to any indicator for diagnostics:

```csharp
private void RunDiagnostics()
{
    Print("=== FKS DIAGNOSTICS ===");
    Print($"Core Initialized: {FKSCore.IsInitialized}");
    Print($"Market Config: {FKSCore.CurrentMarketConfig?.Symbol}");
    
    var health = FKSCore.GetComponentHealth();
    foreach (var component in health)
    {
        Print($"{component.Key}: {(component.Value.IsHealthy ? "OK" : "FAIL")}");
    }
    
    Print($"Current State: {FKSCore.CurrentTradingState?.TradingEnabled}");
    Print($"Daily P&L: {FKSCore.CurrentTradingState?.DailyPnL:C}");
    Print("===================");
}
```

### **2. Signal Quality Analyzer**

```csharp
// Add to FKS_AI OnBarUpdate for analysis
if (currentSignalType != "" && DebugMode)
{
    var breakdown = new StringBuilder();
    breakdown.AppendLine($"Signal: {currentSignalType}");
    breakdown.AppendLine($"Quality: {currentSignalQuality:P}");
    breakdown.AppendLine($"- Base: {baseQuality:P}");
    breakdown.AppendLine($"- Wave: {waveRatio:F2}x");
    breakdown.AppendLine($"- Trend: {trendAligned ? "Yes" : "No"}");
    breakdown.AppendLine($"- Volume: {volumeRatio:F2}x");
    breakdown.AppendLine($"- Time: {isOptimalSession ? "Yes" : "No"}");
    
    Print(breakdown.ToString());
}
```

---

## 📞 **WHEN ALL ELSE FAILS**

### **Reset to Clean State:**

1. **Export current settings:**
   ```
   Tools → Export → NinjaScript
   Select FKS items → Export
   ```

2. **Remove FKS completely:**
   ```
   Tools → Remove NinjaScript Assembly
   Select FKS → Remove
   Restart NinjaTrader
   ```

3. **Fresh import:**
   ```
   Import latest package
   Create new workspace
   Add components one by one
   Test each component
   ```

### **Support Resources:**

- **Logs location:** `Documents\NinjaTrader 8\logs\`
- **Trace files:** `Documents\NinjaTrader 8\trace\`
- **Screenshots:** Take before removing
- **Settings backup:** Export workspace XML

---

## ✅ **PREVENTION CHECKLIST**

**Daily startup routine:**
- [ ] Check Python server running
- [ ] Verify data connection
- [ ] Load saved workspace
- [ ] Check component health
- [ ] Review yesterday's performance
- [ ] Note any overnight gaps
- [ ] Verify risk limits reset

**Weekly maintenance:**
- [ ] Export trade history
- [ ] Backup workspace
- [ ] Clear old logs
- [ ] Update Python database
- [ ] Review error logs
- [ ] Check for updates

Remember: **Most issues are configuration-related, not code bugs!**