use pyo3::prelude::*;
use pyo3::types::PyAny;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::sync::Arc;
use std::fmt::Debug;

#[pyclass]
#[derive(Debug, Clone)]
pub struct PiEstimationConfig {
    pub radius: f64,
    pub precision: f64,
}

#[pymethods]
impl PiEstimationConfig {
    #[new]
    pub fn new(radius: f64, precision: f64) -> Self {
        PiEstimationConfig { radius, precision }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct OptionPricingConfig {
    pub spot_price: f64,
    pub strike_price: f64,
    pub rate: f64,
    pub volatility: f64,
    pub time_to_maturity: f64,
}

#[pymethods]
impl OptionPricingConfig {
    #[new]
    pub fn new(
        spot_price: f64,
        strike_price: f64,
        rate: f64,
        volatility: f64,
        time_to_maturity: f64,
    ) -> Self {
        OptionPricingConfig {
            spot_price,
            strike_price,
            rate,
            volatility,
            time_to_maturity,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct RiskAnalysisConfig {
    spot_price: f32,
    strike_price: f32,
    rate: f32,
    volatility: f32,
    time_to_maturity: f32,
}

#[pymethods]
impl RiskAnalysisConfig {
    #[new]
    pub fn new(
        spot_price: f32,
        strike_price: f32,
        rate: f32,
        volatility: f32,
        time_to_maturity: f32,
    ) -> Self {
        RiskAnalysisConfig {
            spot_price,
            strike_price,
            rate,
            volatility,
            time_to_maturity,
        }
    }
}

pub trait MonteCarloStrategy: Send + Sync + Debug {
    fn run_trial(&self, seed: u32, num_steps: u32) -> f64;
}

#[pyclass]
#[derive(Debug)]
pub struct PiEstimation {
    config: PiEstimationConfig,
}

#[pymethods]
impl PiEstimation {
    #[new]
    pub fn new(config: PiEstimationConfig) -> Self {
        PiEstimation { config }
    }
}

impl MonteCarloStrategy for PiEstimation {
    fn run_trial(&self, seed: u32, num_steps: u32) -> f64 {
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let mut inside_circle: u32 = 0;

        for _ in 0..num_steps {
            let x: f64 = rng.random_range(-self.config.radius..self.config.radius);
            let y: f64 = rng.random_range(-self.config.radius..self.config.radius);

            // x^2 + y^2 <= r^2
            if x * x + y * y <= (self.config.radius * self.config.radius) {
                inside_circle += 1;
            }
        }
        inside_circle as f64 / num_steps as f64
    }
}

#[pyclass]
#[derive(Debug)]
pub struct MonteCarloEngine {
    num_trials: u32,
    num_steps: u32,
    num_threads: u32,
    seed: u32,
    strategy: PyObject,
}

#[pymethods]
impl MonteCarloEngine {
    #[new]
    pub fn new(
        num_trials: u32,
        num_steps: u32,
        num_threads: u32,
        seed: u32,
        strategy: PyObject,
    ) -> Self {
        MonteCarloEngine {
            num_trials,
            num_steps,
            num_threads,
            seed,
            strategy,
        }
    }

    pub fn run(&self, py: Python) -> PyResult<f64> {
        let strategy: PyRef<PyAny> = self.strategy.extract(py)?

        let trials_per_thread = self.num_trials / self.num_threads;
        let remainder = self.num_trials % self.num_threads;

        let mut thread_trial_counts = vec![trials_per_thread; self.num_threads as usize];

        for i in 0..remainder {
            thread_trial_counts[i as usize] += 1;
        }

        let results: Vec<f64> = thread_trial_counts
            .into_par_iter()
            .enumerate()
            .map(|(i, trials)| {
                let seed = self.seed + i as u32 * trials as u32;
                let mut total = 0.0;
                for ii in 0..trials {
                    total += self.strategy.run_trial(seed + ii, self.num_steps);
                }
                total / trials as f64
            })
            .collect();

        Ok(results.iter().sum::<f64>() / results.len() as f64)
    }
}
