// TODO: Define a base class named `Person`, with `first_name` and `last_name` attributes, set
//  by the constructor. It should be possible to access the `first_name` and `last_name` attributes
//  of a `Person`.
//  `Person` should also have a method named `full_name` that returns the full name of the person.
//  Then define a subclass named `Employee` that inherits from `Person` and adds an
//  unsigned integer `id` attribute and a constructor that sets the `id` attribute.
//  It should be possible to access the `first_name`, `last_name` and `id`
//  attributes of an `Employee`.
use pyo3::prelude::*;

#[pyclass(subclass)]
struct Person {
    #[pyo3(get)]
    first_name: String,
    #[pyo3(get)]
    last_name: String
}

#[pymethods]
impl Person {
    fn full_name(&self) -> String {
        format!("{0} {1}", self.first_name, self.last_name,)
    }

    #[new]
    fn new(first_name: String, last_name: String) -> Self {
        Person{first_name:first_name, last_name: last_name}
    }
}

#[pyclass(extends= Person)]
struct Employee {
    #[pyo3(get)]
    id: u32
}

#[pymethods]
impl Employee{
    #[new]
    fn new(first_name: String, last_name: String, id: i32) -> PyClassInitializer<Self> {
        let person = Person::new(first_name, last_name);
        let employee = Self {id: id as u32};
        PyClassInitializer::from(person).add_subclass(employee)
    }
}



#[pymodule]
fn inheritance(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Person>()?;
    m.add_class::<Employee>()?;
    Ok(())
}
