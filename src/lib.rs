use pyo3::prelude::*;
mod engine;

/// A Python module implemented in Rust.
#[pymodule]
fn monte_carlo_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<engine::MonteCarloEngine>()?;
    m.add_class::<engine::PiEstimationConfig>()?;
    m.add_class::<engine::PiEstimation>()?;
    m.add_class::<engine::OptionPricingConfig>()?;
    m.add_class::<engine::RiskAnalysisConfig>()?;

    Ok(())
}
