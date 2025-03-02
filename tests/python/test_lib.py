from monte_carlo_engine import (
    MonteCarloEngine,
    OptionPricingConfig,
    PiEstimationConfig,
    RandomWalkConfig,
    RiskAnalysisConfig,
    Scenario,
    ScenarioConfig,
    ScenarioSpecificConfig,
)


def test_monte_carlo_engine():
    scenario_config = ScenarioConfig(num_trials=1, num_steps=1, num_threads=1, seed=1)
    MonteCarloEngine(
        Scenario.PiEstimation,
        ScenarioSpecificConfig.PiEstimationConfig(
            PiEstimationConfig(scenario_config=scenario_config, radius=1.0, precision=1.0),
        ),
    )

    MonteCarloEngine(
        Scenario.OptionPricing,
        ScenarioSpecificConfig.OptionPricingConfig(
            OptionPricingConfig(
                scenario_config=scenario_config,
                spot_price=1.0,
                strike_price=1.0,
                rate=1.0,
                volatility=1.0,
                time_to_maturity=1.0,
            ),
        ),
    )

    MonteCarloEngine(
        Scenario.RandomWalk,
        ScenarioSpecificConfig.RandomWalkConfig(
            RandomWalkConfig(scenario_config=scenario_config, seed=1),
        ),
    )

    MonteCarloEngine(
        Scenario.RiskAnalysis,
        ScenarioSpecificConfig.RiskAnalysisConfig(
            RiskAnalysisConfig(
                scenario_config=scenario_config,
                spot_price=1.0,
                strike_price=1.0,
                rate=1.0,
                volatility=1.0,
                time_to_maturity=1.0,
            ),
        ),
    )
