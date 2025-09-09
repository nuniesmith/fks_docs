# FKS Trading Systems - Python Implementation

A Python implementation of the FKS Trading Strategy and Awesome Oscillator indicator, originally written for NinjaTrader in C#.

## Overview

This implementation consists of three main modules:

1. **`fks_strategy.py`** - The main trading strategy with risk management, position sizing, and signal generation
2. **`fks_ao.py`** - Enhanced Awesome Oscillator indicator with pattern recognition and adaptive parameters
3. **`fks_integration_example.py`** - Example integration showing how to use the modules in a web application

## Features

### Strategy Features
- Multiple trading modes (Trading Enabled, Monitoring Only, Debug)
- Configurable risk management with daily profit/loss limits
- Market regime detection (Volatile, Strong Trend, Range, Neutral)
- Time-based trading filters
- Position sizing based on signal strength and market conditions
- Session management and performance tracking

### AO Indicator Features
- Adaptive period calculation based on market conditions
- Pattern recognition (Saucer, Twin Peaks, Zero Line Cross)
- Signal quality analysis
- Divergence detection
- TradingView-style histogram coloring
- Debug mode with comprehensive metrics

## Installation

1. Clone or download the Python modules
2. Install dependencies:

```bash
pip install -r requirements.txt
```

## Basic Usage

### Simple Example

```python
from fks_strategy import FKSStrategy, StrategyMode
from fks_ao import FKSAwesomeOscillator
import pandas as pd

# Initialize strategy
strategy = FKSStrategy({
    'mode': StrategyMode.MONITORING_ONLY,
    'signal_threshold': 0.65,
    'risk_percent': 1.0
})

# Initialize AO indicator
ao = FKSAwesomeOscillator({
    'fast_period': 5,
    'slow_period': 34,
    'base_ao_threshold': 0.65
})

# Load your market data
# Data should have columns: timestamp, open, high, low, close, volume
market_data = pd.read_csv('your_data.csv')
market_data['timestamp'] = pd.to_datetime(market_data['timestamp'])

# Update indicators and strategy
ao.update_market_data(market_data)
strategy.update_market_data(market_data)

# Generate signals
ao_signal = ao.generate_signal()
strategy_signal = strategy.generate_signal()

print(f"AO Signal: {ao_signal.direction.value}, Confidence: {ao_signal.confidence:.2f}")
print(f"Strategy Signal: {strategy_signal.direction.value}, Confidence: {strategy_signal.confidence:.2f}")
```

### Web Application Integration

```python
from fks_integration_example import FKSTradingSystem

# Create trading system
system = FKSTradingSystem()

# Process new market data
new_bar = {
    'timestamp': '2024-01-15 10:30:00',
    'open': 100.50,
    'high': 101.00,
    'low': 100.25,
    'close': 100.75,
    'volume': 5000
}

system.process_new_bar(new_bar)

# Get system status
status = system.get_system_status()
print(status)
```

### FastAPI Integration

```python
from fastapi import FastAPI
from fks_integration_example import FKSWebAPI

app = FastAPI()
fks_api = FKSWebAPI()

@app.post("/api/market-update")
async def market_update(data: dict):
    result = await fks_api.handle_market_update(data)
    return result

@app.get("/api/signals")
async def get_signals():
    return await fks_api.get_signals()
```

## Configuration

### Strategy Configuration

```python
strategy_config = {
    'mode': StrategyMode.MONITORING_ONLY,  # Trading mode
    'enable_live_trading': False,          # Enable actual trading
    'debug_level': 2,                      # Debug verbosity (0-3)
    'base_contract_size': 1,               # Base position size
    'max_contract_size': 5,                # Maximum position size
    'risk_percent': 1.0,                   # Risk per trade (%)
    'atr_stop_multiplier': 2.0,            # Stop loss ATR multiplier
    'atr_target_multiplier': 3.0,          # Take profit ATR multiplier
    'signal_threshold': 0.65,              # Minimum signal confidence
    'strong_signal_threshold': 0.80,       # Strong signal threshold
    'use_time_filters': True,              # Use trading time windows
    'soft_profit_target': 2000,            # Soft daily profit target
    'hard_profit_target': 3000,            # Hard daily profit target
    'soft_loss_limit': 1000,               # Soft daily loss limit
    'hard_loss_limit': 1500,               # Hard daily loss limit
    'max_daily_trades': 10                 # Maximum trades per day
}
```

### AO Indicator Configuration

```python
ao_config = {
    'fast_period': 5,                      # Fast MA period
    'slow_period': 34,                     # Slow MA period
    'signal_period': 7,                    # Signal line period
    'base_ao_threshold': 0.65,             # Signal threshold
    'use_adaptive_periods': True,          # Enable adaptive periods
    'adaptation_rate': 0.1,                # Adaptation sensitivity
    'market_state_window': 25,             # Market state history size
    'pattern_recognition': True,           # Enable pattern detection
    'signal_quality_threshold': 0.65,      # Quality threshold
    'divergence_lookback': 25,             # Bars for divergence check
    'min_pivot_bars': 5,                   # Minimum bars for pivot
    'enable_debug_mode': False             # Enable debug logging
}
```

## Data Format

The system expects market data in pandas DataFrame format with the following columns:

- `timestamp`: datetime object or timestamp string
- `open`: Opening price
- `high`: High price
- `low`: Low price
- `close`: Closing price
- `volume`: Trading volume

## Signal Interpretation

### Signal Directions
- `LONG`: Buy signal
- `SHORT`: Sell signal
- `NEUTRAL`: No clear signal

### Signal Quality Metrics
- `confidence`: Overall signal confidence (0.0 to 1.0)
- `score`: Raw signal score
- `is_active`: Whether signal meets minimum threshold

### AO Specific Metrics
- `momentum_score`: Momentum-based signal strength
- `zero_cross_score`: Zero line cross signal strength
- `saucer_score`: Saucer pattern signal strength
- `divergence_score`: Price-indicator divergence signal
- `trend_alignment`: Trend alignment score

## Market Regimes

The strategy identifies four market regimes:

1. **VOLATILE**: High volatility, reduced position sizes
2. **STRONG_TREND**: Clear directional movement, increased position sizes
3. **RANGE**: Range-bound market, reduced position sizes
4. **NEUTRAL**: Normal market conditions

## Risk Management

The strategy implements multiple risk management layers:

1. **Position Sizing**: Dynamic sizing based on signal strength and market regime
2. **Daily Limits**: Soft and hard profit/loss limits
3. **Trade Limits**: Maximum number of trades per day
4. **Time Filters**: Optional trading time windows
5. **ATR-based Stops**: Dynamic stop loss and take profit levels

## Performance Tracking

The system tracks:
- Total signals generated
- Valid signals (meeting threshold)
- Executed trades
- Session P&L
- Win rate (when trade history available)
- Signal quality metrics

## Debug Mode

Enable debug mode for detailed logging:

```python
strategy = FKSStrategy({'debug_level': 3})
ao = FKSAwesomeOscillator({'enable_debug_mode': True})
```

## Best Practices

1. **Data Quality**: Ensure your market data is clean and properly formatted
2. **Backtesting**: Test thoroughly with historical data before live trading
3. **Risk Management**: Start with conservative settings and adjust gradually
4. **Monitoring**: Use monitoring mode first to understand signal behavior
5. **Updates**: Process data in chronological order for accurate results

## Differences from C# Implementation

1. **No NinjaTrader Dependencies**: Pure Python implementation
2. **Simplified Component System**: Direct class usage instead of component registry
3. **Web-Friendly**: Designed for web application integration
4. **Async Support**: Built-in support for async operations
5. **JSON Serialization**: Easy conversion to JSON for APIs

## License

This is a Python port of the original C# NinjaTrader implementation. Please ensure you have the right to use the original code before using this implementation.

## Support

For questions or issues:
1. Check the example integration code
2. Review the inline documentation
3. Enable debug mode for detailed logging
4. Ensure data format matches requirements
