//! Simulation input structure.

use crate::{input::Settings, output::Data, parts::Gradient};
use arctk::{
    err::Error,
    geom::Grid,
    math::Vec3,
    ord::{X, Y},
    tools::ProgressBar,
};
use ndarray::Array3;
use ndarray_stats::QuantileExt;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

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
    #[allow(clippy::expect_used)]
    #[inline]
    #[must_use]
    pub fn sim(&self, time: f64, mut values: Array3<f64>) -> Data {
        debug_assert!(time > 0.0);
        debug_assert!(values.shape() == self.grid.res());

        // let data = Data::new(values);

        let voxel_size = self.grid.voxel_size();
        let dx = voxel_size.min();

        let max_coeff = self
            .coeffs
            .max()
            .expect("Failed to determine the maximum diffusion coefficient.");
        let max_dt = (dx * dx) / (4.0 * max_coeff);
        let dt = max_dt * self.sett.step_multiplier();

        let num_steps = (time / dt).ceil();
        let dt = time / num_steps;
        let num_steps = num_steps as u64;

        let mut rate;
        let mut pb = ProgressBar::new("Diffusing", num_steps as u64);
        for _ in 0..num_steps {
            pb.tick();

            rate = self
                // .multi_thread(&values)
                // .expect("Failed to calculate diffusion rate.");
                .single_thread(&values);
            values += &(rate * dt);
        }

        Data::new(values)
    }

    /// Run a multi-threaded diffusion simulation.
    /// # Errors
    /// if the progress bad can not be locked.
    #[allow(clippy::expect_used)]
    #[inline]
    pub fn multi_thread(&self, values: &Array3<f64>) -> Result<Array3<f64>, Error> {
        debug_assert!(values.shape() == self.coeffs.shape());

        let pb = ProgressBar::new("Multi-threaded", self.grid.total_cells() as u64);
        let pb = Arc::new(Mutex::new(pb));

        let threads: Vec<_> = (0..num_cpus::get()).collect();
        let mut out: Vec<_> = threads
            .par_iter()
            .map(|_id| Self::thread(&Arc::clone(&pb), self.grid, values, self.coeffs))
            .collect();
        pb.lock()?.finish_with_message("Step complete.");

        let mut data = out.pop().expect("No data received.");
        while let Some(o) = out.pop() {
            data += &o;
        }

        Ok(data)
    }

    /// Run a cartography simulation using a single thread.
    #[allow(clippy::module_name_repetitions)]
    #[inline]
    #[must_use]
    pub fn single_thread(&self, values: &Array3<f64>) -> Array3<f64> {
        let pb = ProgressBar::new("Single-threaded", self.grid.total_cells() as u64);
        let pb = Arc::new(Mutex::new(pb));

        Self::thread(&pb, self.grid, values, self.coeffs)
    }

    /// Thread control function.
    #[allow(clippy::module_name_repetitions)]
    #[allow(clippy::expect_used)]
    #[inline]
    #[must_use]
    fn thread(
        _pb: &Arc<Mutex<ProgressBar>>,
        grid: &Grid,
        values: &Array3<f64>,
        coeffs: &Array3<f64>,
    ) -> Array3<f64> {
        let rates = Array3::zeros(*grid.res());
        diff_rate(grid.voxel_size(), values, coeffs, rates)
    }
}

/// Calculate the diffusion rates for each cell.
#[inline]
#[must_use]
pub fn diff_rate(
    cell_size: &Vec3,
    values: &Array3<f64>,
    coeffs: &Array3<f64>,
    mut rate: Array3<f64>,
) -> Array3<f64> {
    debug_assert!(values.shape() == coeffs.shape());

    let num_cells = values.len();

    let res = values.shape();

    for n in 0..num_cells {
        let xi = n % res[X];
        let yi = (n / res[X]) % res[Y];
        let zi = n / (res[X] * res[Y]);

        let index = [xi, yi, zi];

        let stencil = Gradient::new(index, values);
        let r = stencil.rate(coeffs[index], cell_size);
        rate[index] = r;
    }

    rate
}
