use pyo3::{exceptions::PyValueError, prelude::*};

// TODO: Add a `__new__` constructor to the `ShoppingOrder` class that takes the following arguments:
//  - `name` (non-empty string)
//  - `price` (non-zero integer)
//  - `quantity` (non-zero integer)
//  The constructor should raise a `ValueError` if any of the arguments are invalid.

#[pyclass]
struct ShoppingOrder {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    price: u64,
    #[pyo3(get, set)]
    quantity: u64,
}

#[pymethods]
impl ShoppingOrder {
    #[new]
    fn new(name: String, price: i32, quantity: i32) -> PyResult<ShoppingOrder> {
        // let n: usize = n.extract().map_err(|_| PyTypeError::new_err("n must be an integer"))?;
        if price < 0 {
            return Err(PyValueError::new_err("Negative prices are not supported"))
        }
        if price == 0 {
            return Err(PyValueError::new_err("Zero prices are not supported"))
        }
        if quantity < 0 {
            return Err(PyValueError::new_err("Negative quantities are not supported"))
        }
        if quantity == 0 {
            return Err(PyValueError::new_err("Zero quantities are not supported"))
        }
        if name == "" {
            return Err(PyValueError::new_err("Name can't be empty"))
        }
        if name == "  " {
            return Err(PyValueError::new_err("Name can't be just be whitespace"))
        }
        Ok(ShoppingOrder{
            name: name,
            price: price as u64,
            quantity: quantity as u64,
        })
    }
}

#[pymodule]
fn constructors(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ShoppingOrder>()?;
    Ok(())
}
