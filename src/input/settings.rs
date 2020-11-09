//! Diffusion simulation settings structure.

use arctk::clone;
use arctk_attr::input;

/// Loadable input diffusion settings structure.
#[input]
pub struct Settings {
    /// Minimum time step.
    min_step: f64,
}

impl Settings {
    clone!(min_step, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(min_step: f64) -> Self {
        debug_assert!(min_step > 0.0);

        Self { min_step }
    }
}
