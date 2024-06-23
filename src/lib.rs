use pyo3::prelude::*;

pub mod generators {

    use std::fmt::Error;

    use cuid2::CuidConstructor;
    use pyo3::prelude::*;
    use rayon::prelude::*;
    
    #[derive(Clone, Debug)]
    #[pyclass]
    pub struct SingleResult {
        
        #[pyo3(get)]
        pub result: String,
    }
    
    #[derive(Clone, Debug)]
    #[pyclass]
    pub struct MultipleResult {

        #[pyo3(get)]
        pub result: Vec<SingleResult>,
    }
    
    #[pymethods]
    impl SingleResult {
        pub fn pipe(&self, f: &Bound<'_, PyAny>) -> PyResult<Self> {
            let result = self.clone();
            _ = f.call1((result.result,));
            Ok(self.clone())
        }
    }
    
    #[pymethods]
    impl MultipleResult {
        pub fn pipe(&self, f: &Bound<'_, PyAny>) -> PyResult<Self> {
            let result = self.clone();
            
            for r in result.result.iter() {
                _ = f.call1((r.result.clone(),));
            }

            Ok(self.clone())
        }
    }

    pub fn cuid(len: u16) -> Result<SingleResult, Error> {
        let constructor = CuidConstructor::new().with_length(len);
        Ok(SingleResult { result: constructor.create_id()})
    }

    pub fn n_cuid(count: u64, len: u16) -> Result<MultipleResult, Error> {
        let v: Vec<SingleResult> = (0..count).into_par_iter().map(|_| cuid(len).unwrap()).collect();
        Ok(MultipleResult { result: v })
    }

    #[pyfunction]
    pub fn with_cuid(len: u16) -> PyResult<SingleResult> {
        Ok(cuid(len).unwrap())
    }

    #[pyfunction]
    pub fn n_with_cuid(count: u64, len: u16) -> PyResult<MultipleResult> {
        Ok(n_cuid(count, len).unwrap())
    }

}

#[pymodule]
fn idika(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generators::with_cuid, m)?)?;
    m.add_function(wrap_pyfunction!(generators::n_with_cuid, m)?)?;
    Ok(())
}
