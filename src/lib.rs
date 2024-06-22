use pyo3::prelude::*;

pub mod generators {

    use cuid2::CuidConstructor;
    use pyo3::prelude::*;
    use rayon::prelude::*;

    pub fn cuid(len: u16) -> String {
        let constructor = CuidConstructor::new().with_length(len);
        constructor.create_id()
    }

    pub fn n_cuid(count: u64, len: u16) -> Vec<String> {
        let v: Vec<_> = (0..count).into_par_iter().map(|_| cuid(len)).collect();
        v
    }

    #[pyfunction]
    pub fn with_cuid(len: u16) -> PyResult<String> {
        Ok(cuid(len))
    }

    #[pyfunction]
    pub fn n_with_cuid(count: u64, len: u16) -> PyResult<Vec<String>> {
        Ok(n_cuid(count, len))
    }

}

#[pymodule]
fn idika(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generators::with_cuid, m)?)?;
    m.add_function(wrap_pyfunction!(generators::n_with_cuid, m)?)?;
    Ok(())
}
