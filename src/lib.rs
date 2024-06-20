use pyo3::prelude::*;

pub mod generators {

    use cuid2::CuidConstructor;
    use pyo3::prelude::*;
    use rayon::prelude::*;
    use sonyflake::Sonyflake;

    pub fn snowflake() -> u64 {
        let sf = Sonyflake::new().unwrap();
        sf.next_id().unwrap()
    }

    pub fn cuid(len: u16) -> String {
        let constructor = CuidConstructor::new().with_length(len);
        constructor.create_id()
    }

    pub fn n_snowflake(count: u64) -> Vec<u64> {
        let v: Vec<_> = (0..count).into_par_iter().map(|_| snowflake()).collect();
        v
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
    pub fn with_snowflake() -> PyResult<u64> {
        Ok(snowflake())
    }

    #[pyfunction]
    pub fn n_with_cuid(count: u64, len: u16) -> PyResult<Vec<String>> {
        Ok(n_cuid(count, len))
    }

    #[pyfunction]
    pub fn n_with_snowflake(count: u64) -> PyResult<Vec<u64>> {
        Ok(n_snowflake(count))
    }
}

#[pymodule]
fn idika(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generators::with_cuid, m)?)?;
    m.add_function(wrap_pyfunction!(generators::with_snowflake, m)?)?;
    m.add_function(wrap_pyfunction!(generators::n_with_snowflake, m)?)?;
    m.add_function(wrap_pyfunction!(generators::n_with_snowflake, m)?)?;
    Ok(())
}
