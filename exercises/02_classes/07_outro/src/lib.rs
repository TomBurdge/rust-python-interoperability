use std::mem::discriminant;
use chrono::{DateTime, Utc};

// TODO: Define a base class named `Discount`, with a `percentage` attribute.
//  It should be possible to access the `percentage` attribute of a `Discount`.
//  It should also be possible to modify the `percentage` attribute of a `Discount`.
//  It must be enforced that the `percentage` attribute is a float between 0. and 1.
//  Then define two subclasses:
//  - `SeasonalDiscount` that inherits from `Discount` with two additional attributes, `to` and `from_`.
//    `from_` is a datetime object that represents the start of the discount period.
//    `to` is a datetime object that represents the end of the discount period.
//     Both `from_` and `to` should be accessible and modifiable.
//     The class should enforce that `from` is before `to`.
//  - `CappedDiscount` that inherits from `Discount` with an additional attribute `cap`.
//    `cap` is a float that represents the maximum discount (in absolute value) that can be applied.
//    It should be possible to access and modify the `cap` attribute.
//    The class should enforce that `cap` is a non-zero positive float.
//
// All classes should have a method named `apply` that takes a price (float) as input and
// returns the discounted price.
// `SeasonalDiscount` should raise an `ExpiredDiscount` exception if `apply` is called but
// the current date is outside the discount period.
use pyo3::{exceptions::PyValueError, prelude::*,create_exception, PyClass};

#[pyclass(subclass)]
struct Discount {
    #[pyo3(get)]
    percentage: f32
}

#[pymethods]
impl Discount {
    #[new]
    fn new(percentage: f32) -> PyResult<Self> {
        Discount::try_from(percentage)
    }

    #[setter]
    fn set_percentage(&mut self, percentage: f32) -> PyResult<()>  {
        if percentage> 0.0 && percentage < 1.0 {
            self.percentage = percentage;
            Ok(())
        } else {
            Err(PyValueError::new_err("Percentage must be between 0 and 1"))
        }
    }

    fn apply(& self, price: f32) -> f32 {
        price - price * self.percentage
    }
}

impl TryFrom<f32> for Discount {
    type Error = PyErr;
    fn try_from(value: f32) -> Result<Self, PyErr> {
        if value > 0.0 && value < 1.0 {
            Ok(Discount{percentage: value})
        } else {
            Err(PyValueError::new_err("Percentage must be between 0 and 1"))
        }
    }
}


create_exception!(outro2, ExpiredDiscount, pyo3::exceptions::PyException);
#[pyclass(extends=Discount)]
struct CappedDiscount {
    #[pyo3(get, set)]
    cap: f32
}

#[pymethods]
impl CappedDiscount {
    #[new]
    fn new(percentage: f32, cap: f32) -> PyResult<PyClassInitializer<Self>> {
        let discount: Discount = percentage.try_into()?;
        if cap > 0.0 { 
            let capped_discount = Self{cap: cap};
            Ok(PyClassInitializer::from(discount).add_subclass(capped_discount))
        } else {
            Err(PyValueError::new_err("Cap must be a positive number"))
        }
    }

    #[getter]
    fn get_percentage(self_: PyRef<'_, Self>) -> f32 {
        self_.as_super().percentage
    }

    fn apply(self_: PyRef<'_, Self>, price: f32) -> f32 {
        let uncapped_price = self_.as_super().apply(price);
        uncapped_price.max(price - self_.cap)
    }
}

//  - `SeasonalDiscount` that inherits from `Discount` with two additional attributes, `to` and `from_`.
//    `from_` is a datetime object that represents the start of the discount period.
//    `to` is a datetime object that represents the end of the discount period.
//     Both `from_` and `to` should be accessible and modifiable.
//     The class should enforce that `from` is before `to`.
#[pyclass(extends=Discount)]
struct SeasonalDiscount {
    #[pyo3(get, set)]
    from_: DateTime<Utc>,
    #[pyo3(get, set)]
    to: DateTime<Utc>
}

#[pymethods]
impl SeasonalDiscount {
    #[new]
    fn new(percentage: f32, from_: DateTime<Utc>, to: DateTime<Utc>) -> PyResult<PyClassInitializer<Self>> {
        if from_ >= to {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "`from_` date must be before `to` date",
            ));
        }
        let discount: Discount = percentage.try_into()?;
        let seasonal = SeasonalDiscount{from_, to};
        Ok(PyClassInitializer::from(discount).add_subclass(seasonal))
    }

    fn apply(self_: PyRef<'_, Self>, price: f32) -> PyResult<f32> {
        let now = Utc::now();
        if now < self_.from_ || now > self_.to {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Discount is expired.",
            ));
        }
        Ok(self_.as_super().apply(price))
    }
}

#[pymodule]
fn outro2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Discount>()?;
    m.add_class::<CappedDiscount>()?;
    m.add_class::<SeasonalDiscount>()?;
    m.add("ExpiredDiscount", m.py().get_type_bound::<ExpiredDiscount>())?;
    Ok(())
}
