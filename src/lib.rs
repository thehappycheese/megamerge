
extern crate ndarray;
use pyo3::{prelude::*, PyIterProtocol};
use numpy::ndarray::{ArrayD, ArrayViewD, ArrayViewMutD};
use numpy::{PyArray, PyArray2};


#[pyclass] 
struct Iterator { 
    iter: Box<dyn std::iter::Iterator<Item = i32> + Send>, 
    token: Python<'a>, 
} 
 
#[pyproto] 
impl PyIterProtocol for Iterator { 
    fn __iter__(&mut self) -> PyResult<Py<Iterator>> { 
        Ok(self.into()) 
    } 
}



/// A Python module implemented in Rust.
#[pymodule]
fn megamerge(_py: Python, m: &PyModule) -> PyResult<()> {

    fn ddo(segmentation:ArrayViewD<'_, f64>, data: ArrayViewD<'_, f64>) -> i32 {
        5
    }

    #[pyfn(m)]
    fn do_chonk<'py>(
        py           : Python<'py>,
        segmentation : &'py PyArray2<f64>,
        data         : &'py PyArray2<f64>,
    )->PyResult<&'py PyArray2<f64>>{
        Ok(segmentation)
    }
 #[pyclass] 
 struct Iterator { 
     iter: Box<iter::Iterator<Item = i32> + Send>, 
     token: PyToken, 
 } 
  
 #[pyproto] 
 impl PyIterProtocol for Iterator { 
     fn __iter__(&mut self) -> PyResult<Py<Iterator>> { 
         Ok(self.into()) 
     } 
    //m.add_function(wrap_pyfunction!(do_chonk, m)?)?;
    Ok(())
}