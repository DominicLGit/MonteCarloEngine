from typing import Protocol

class PiEstimationConfig:
    radius: float
    precision: float

    def __init__(self, radius: float, precision: float) -> None: ...

class OptionPricingConfig:
    spot_price: float
    strike_price: float
    rate: float
    volatility: float
    time_to_maturity: float

    def __init__(
        self, spot_price: float, strike_price: float, rate: float, volatility: float, time_to_maturity: float,
    ) -> None: ...

class RiskAnalysisConfig:
    spot_price: float
    strike_price: float
    rate: float
    volatility: float
    time_to_maturity: float

    def __init__(
        self, spot_price: float, strike_price: float, rate: float, volatility: float, time_to_maturity: float,
    ) -> None: ...

class MonteCarloStrategy(Protocol):
    ...

class PiEstimation(MonteCarloStrategy):
    def __init__(self, config: PiEstimationConfig) -> None: ...

class MonteCarloEngine:
    def __init__(
        self, num_trials: int, num_steps: int, num_threads: int, seed: int, strategy: MonteCarloStrategy,
    ) -> None: ...
    def run(self) -> float: ...
