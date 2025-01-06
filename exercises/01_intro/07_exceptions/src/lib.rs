use pyo3::prelude::*;
use pyo3::types::PyLong;
use pyo3::exceptions::PyTypeError;

#[pyfunction]
// TODO: Implement a function that returns a list containing the first `n` numbers in Fibonacci's sequence.
//  It must raise a `TypeError` if `n` is not an integer or if it is less than 0.
fn fibonacci(n: Bound<'_, PyLong>)-> PyResult<Vec<u32>> {
    let n: usize = n.extract().map_err(|_| PyTypeError::new_err("n must be an integer"))?;
    let mut numbers: Vec<u32> = vec![];
    if n == 0 {
        return Ok(numbers)
    }
    for i in 0..=n.saturating_sub(1) {
        if i < 2 {
            numbers.push(i as u32);
        }
        else {
            numbers.push(numbers[i -2] + numbers[i -1])
        }
    }
    Ok(numbers)
}

#[pymodule]
fn exceptions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
