// TODO: Every time either `name` or `price` is accessed, increment the `n_visits` field in `Item` by one.
use pyo3::prelude::*;

#[pyclass]
struct Item {
    name: String,
    price: u64,
    #[pyo3(get)]
    n_visits: u64,
}

#[pymethods]
impl Item {
    #[new]
    fn new(name: String, price: u64) -> Self {
        Item {
            name,
            price,
            n_visits: 0,
        }
    }

    #[getter]
    fn get_price(&mut self) -> u64 {
        self.increment_n_visits();
        self.price
    }

    #[getter]
    fn get_name(&mut self) -> String {
        self.increment_n_visits();
        self.name.clone()
    }
}

impl Item {
    fn increment_n_visits(&mut self) {
        self.n_visits += 1;
    }
}

#[pymodule]
fn setters(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Item>()?;
    Ok(())
}
