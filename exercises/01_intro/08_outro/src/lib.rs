// TODO: Expose a function named `max_k` that takes a list of unsigned integers and return as output
//   a list containing the `k` largest numbers in the list, in descending order.
//
// Hint: you can use the `num_bigint` crate if you think it'd be useful.
use pyo3::prelude::*;
use pyo3::types::PyLong;
use pyo3::exceptions::{PyTypeError, PyValueError};
use num_bigint::BigInt;


#[pyfunction]
fn max_k(mut vec: Vec<BigInt>, k: Bound<'_, PyLong>) -> PyResult<Vec<BigInt>> {
    if vec.iter().any(|x| x < &BigInt::from(0)) {
        return Err(PyTypeError::new_err("All numbers must be non negative."))
    }
    let k: usize = k.extract().map_err(|_| PyTypeError::new_err("n must be an integer"))?;
    if k > vec.len() {
        return Err(PyValueError::new_err("k must be smaller than the length of the list"))
    }
    vec.sort();
    vec.reverse();
    vec.truncate(k);
    Ok(vec)
}

#[pymodule]
#[pyo3(name="outro1")]
fn functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(max_k, m)?)?;
    Ok(())
}