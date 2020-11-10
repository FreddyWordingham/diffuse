//! Simulation input structure.

use crate::{input::Settings, output::Data};
use arctk::{err::Error, geom::Grid, tools::ProgressBar};
use ndarray::Array3;
use ndarray_stats::QuantileExt;

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
    pub fn sim(&self, time: f64, values: Array3<f64>) -> Data {
        debug_assert!(time > 0.0);
        debug_assert!(values.shape() == self.grid.res());

        let data = Data::new(values);

        let voxel_size = self.grid.voxel_size();
        let dx = voxel_size.min();

        let max_coeff = self
            .coeffs
            .max()
            .expect("Failed to determine the maximum diffusion coefficient.");
        let max_dt = (dx * dx) / (4.0 * max_coeff);
        let dt = max_dt * self.sett.step_multiplier();

        let num_steps = (time / max_dt).ceil();
        let dt = time / num_steps;
        let num_steps = num_steps as u64;

        let num_cells = self.grid.total_cells();
        let mut rate: Array3<f64> = Array3::zeros(*self.grid.res());

        let mut pb = ProgressBar::new("Diffusing", num_steps as u64);
        for _ in 0..num_steps {
            pb.tick();

            let rate = self
                .multi_thread(&values)
                .expect("Failed to calculate diffusion rate.");
            values += &(rate * dt);
        }

        data
    }

    /// Run a multi-threaded diffusion simulation.
    /// # Errors
    /// if the progress bad can not be locked.
    #[allow(clippy::expect_used)]
    #[inline]
    pub fn multi_thread(&self, values: &Array3<f64>) -> Result<Array3<f64>, Error> {
        debug_assert!(values.shape() == self.coeffs.shape());

        values.

        // Ok(data)
    }
}

// /// Calculate the diffusion rates for each cell.
// #[inline]
// #[must_use]
// pub fn diff_rate(cell_size: &Vec3, values: &Array3<f64>, coeffs: &Array3<f64>) -> Array3<f64> {
//     debug_assert!(concs.shape() == coeffs.shape());

//     let num_cells = values.len();

//     let mut rate = Array3::zeros(values.raw_dim());
//     let res = values.shape();

//     for n in 0..num_cells {
//         let xi = n % res[X];
//         let yi = (n / res[X]) % res[Y];
//         let zi = n / (res[X] * res[Y]);

//         let index = [xi, yi, zi];

//         let stencil = Gradient::new(index, values);
//         let r = stencil.rate(coeffs[index], cell_size);
//         rate[index] = r;
//     }

//     rate
// }
