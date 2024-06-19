mod lists;

use pyo3::prelude::*;

use crate::lists::List;

#[pyclass]
struct IntStack(List<i32>);

#[pymethods]
impl IntStack {
    pub fn push(&mut self, elem: i32) {
        self.0 = self.0.prepend(elem);
    }

    pub fn pop(&mut self) {
        self.0 = self.0.tail();
    }

    pub fn head(&self) -> i32 {
        self.0.head().map(|v| v.clone()).unwrap()
    }
}

#[pyfunction]
fn int_stack() -> IntStack {
    IntStack(List::new())
}

#[pymodule]
fn rbind(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(int_stack, m)?)?;
    m.add_class::<IntStack>()?;
    Ok(())
}
