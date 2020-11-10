//! Simulation input structure.

use crate::{input::Settings, output::Data};
use arctk::geom::Grid;
use ndarray::Array3;

/// Simulation input structure.
pub struct System<'a> {
    /// Coefficents.
    pub coeffs: &'a Array3<f64>,
    /// Integration settings.
    pub sett: &'a Settings,
    /// Simulation grid.
    pub grid: &'a Grid,
}

impl<'a> System<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(coeffs: &'a Array3<f64>, sett: &'a Settings, grid: &'a Grid) -> Self {
        debug_assert!(coeffs.shape() == grid.res());

        Self { coeffs, sett, grid }
    }

    /// Simulate the change of given initial condition within the system.
    #[inline]
    #[must_use]
    pub fn sim(&self, time: f64, values: Array3<f64>) -> Data {
        debug_assert!(time > 0.0);
        debug_assert!(values.shape() == self.grid.res());

        let data = Data::new(values);

        data
    }
}
