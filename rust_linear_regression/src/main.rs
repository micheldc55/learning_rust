use ndarray::prelude::*;
use ndarray_linalg::Solve;
use pyo3::prelude::*;
use statrs::distribution::StudentsT;
use statrs::statistics::Distribution;

#[pyfunction]
fn linear_regression(x: Vec<Vec<f64>>, y: Vec<f64>) -> PyResult<(Vec<f64>, Vec<f64>)> {
    let x = Array2::from_shape_vec((x.len(), x[0].len()), x.into_iter().flatten().collect())
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format! ("{}", e)))?;
    
    let y = Array1::from_shape_vec(y.len(), y)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format! ("{}", e)))?;

    let x_t = x.t().to_owned();
    let xtx = x_t.dot(&x);
    let xty = x_t.dot(&y);

    let beta = xtx.solve(&xty)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format! ("{}", e)))?;

    // Residuals computation:
    let y_hat = x.dot(&beta);
    let residuals = &y - &y_hat;

    let n: f64 = y.len() as f64;
    let p: f64 = beta.len() as f64;
    let sigma2 = residuals.dot(&residuals) / (n - p);

    // Calculate the standard err coeficinets
    let var_beta = sigma2 * xtx.inv().unwrap();
    let se_beta = var_beta.diag().mapv(|x| x.sqrt());

    // Calculate statistics using the student T distribution
    let t_values: Array1<f64> = beta.iter()
                                    .rip(se_beta.iter())
                                    .map(|(b, se)| b / se)
                                    .collect();

    // p-values calculations
    let t_dist = StudentsT::new(0.0, 1.0, (n-p) as f64).unwrap();
    let p_values: Vec<f64> = t_values.iter()
                                    .map(|&t| 2.0 * (1.0 - t_dist_cdf(t.abs())))
                                    .collect();

        Ok((beta.to_vec(), p_values))
}

#[pymodule]
fn linear_regression_module(
    py: Python, m: & Pymodule
) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(linear_regression, m)?)?;
    Ok(())
}