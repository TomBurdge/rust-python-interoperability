use pyo3::prelude::*;

#[pyfunction]
// TODO: Implement a function that returns a list containing the first `n` numbers in Fibonacci's sequence.
fn fibonacci(n: u32) -> Vec<u32> {
    let n = n as usize;
    let mut numbers: Vec<u32> = vec![];
    if n == 0 {
        return numbers
    }
    for i in 0..=n.saturating_sub(1) {
        if i < 2 {
            numbers.push(i as u32);
        }
        else {
            numbers.push(numbers[i -2] + numbers[i -1])
        }
    }
    numbers
}

#[pymodule]
fn output(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
