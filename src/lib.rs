
//extern crate ndarray;

//use ndarray::{};




//use ndarray::{ArrayView1, parallel::prelude::IntoParallelIterator};
use pyo3::{
    prelude::*,
    PyIterProtocol,
    iter::IterNextOutput, types::PyTuple,
};

use numpy::{
    PyArray1,
    PyArray2,
    ndarray::{
        parallel::prelude::*,
        Array2
    }
};


#[pyclass] 
struct Iterator { 
    offset:usize,
    proximity_threshold:f64,
    segmentation: Array2<f64>,
    data: Array2<f64>
}
// unsafe impl Send for Iterator{

// }

impl Iterator {
    fn doof(&self){
          
    }
}
 
#[pyproto] 
impl PyIterProtocol<'_> for Iterator { 
    fn __iter__(slf: PyRef<Self>) -> PyResult<Py<Iterator>> { 
        Ok(slf.into()) 
    }
    fn __next__(mut slf: PyRefMut<Self>) -> IterNextOutput<Py<PyTuple>, &'static str>{
        

        if slf.offset < slf.segmentation.shape()[0] {
            
            let seg_from = slf.segmentation[(slf.offset, 0)];
            let seg_to   = slf.segmentation[(slf.offset, 1)];
            
            slf.offset += 1;
            
            
            let proximity_threshold = slf.proximity_threshold;

            let overlaps:Vec<(usize, f64)> = 
                slf
                .data
                .outer_iter()
                .into_par_iter()
                .enumerate()
                .filter_map(|(index, row)| {
                    let data_from = row[0];
                    let data_to   = row[1];
                    let overlap = f64::min(data_to, seg_to) - f64::max(data_from, seg_from);
                    if overlap > proximity_threshold {
                        Some((index, overlap))
                    } else {
                        None
                    }
                })
                .collect();
            
            let result_index:&PyArray1<usize>   = PyArray1::from_iter(slf.py(), overlaps.iter().map(|(index, _)| *index));
            let result_overlap:&PyArray1<f64>   = PyArray1::from_iter(slf.py(), overlaps.iter().map(|(_, overlap)| *overlap));
            let ff = (result_index, result_overlap).to_object(slf.py());
            let kk = <PyTuple as PyTryFrom>::try_from(ff.as_ref(slf.py())).unwrap();
            
            IterNextOutput::Yield(
                kk.into()
            )  
        }else{
            IterNextOutput::Return("Ended")
        }
    }
}



/// A Python module implemented in Rust.
#[pymodule]
fn megamerge(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m)]
    fn merge_interval_index<'py>(
        segmentation : &'py PyArray2<f64>,
        data         : &'py PyArray2<f64>,
        proximity_threshold : f64,
    )->PyResult<Iterator>{

        Ok(
            Iterator{
                offset:       0,
                proximity_threshold: proximity_threshold,
                segmentation: segmentation.to_owned_array().into(),
                data:         data.to_owned_array().into(),            
            }
        )

    }
    
    //m.add_class<Iterator>()?;
    //m.add_function(wrap_pyfunction!(do_chonk, m)?)?;
    Ok(())
}