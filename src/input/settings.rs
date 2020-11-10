//! Diffusion simulation settings structure.

use arctk::clone;
use arctk_attr::input;

/// Loadable input diffusion settings structure.
#[input]
pub struct Settings {
    /// Time step multiplier (compared to max possible).
    step_multiplier: f64,
}

impl Settings {
    clone!(step_multiplier, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(step_multiplier: f64) -> Self {
        debug_assert!(step_multiplier > 0.0);
        debug_assert!(step_multiplier >= 1.0);

        Self { step_multiplier }
    }
}
