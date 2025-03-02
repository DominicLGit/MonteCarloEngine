from enum import Enum


class SimulationType(str, Enum):
    PiEstimatation = "PiEstimatation"
    OptionPricing = "OptionPricing"
    RandomWalk = "RandomWalk"


class MonteCarloEngine:
    def __init__(self, simulation_type: SimulationType, **params):
        """
        Initialize the engine with a chosen SimulationType (e.g., 'PiEstimatation', 'OptionPricing')
        and any simulation-specific parameters.
        """
        self.simulation_type = simulation_type
        self.params = params

    def set_seed(self, seed: int) -> None:
        """
        Set the random seed for reproducibility.
        """
        self.params["seed"] = seed

    def run(self, iterations: int) -> "SimulationResult":
        """
        Run the simulation for a specified number of iterations.
        """
        outcomes, mean, std = _rust_run_simulation(self.simulation_type, iterations, self.params)
        return SimulationResult(outcomes, mean, std)

    def update_params(self, **params) -> None:
        """
        Update simulation parameters.
        """
        self.params.update(params)


class SimulationResult:
    def __init__(self, outcomes: list, mean: float, std: float):
        self.outcomes = outcomes
        self.mean = mean
        self.std = std

    def summary(self) -> dict:
        """
        Return a summary of the simulation results.
        """
        return {"mean": self.mean, "std": self.std, "n": len(self.outcomes)}


def _rust_run_simulation(simulation_type: SimulationType, iterations: int, params: dict) -> tuple[list, float, float]:
    pass
