use pyo3::prelude::*;

#[pyfunction]
// TODO: Define a function that takes as input a vector of unsigned integers
//  and prints each number in the list.
fn print_number_list(list: Vec<i32>) {
    for val in list {
        println!("{val}")
    }
}

#[pymodule]
fn arguments(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(print_number_list, m)?)?;
    Ok(())
}
