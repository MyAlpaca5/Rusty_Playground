pub mod query_handler;

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use std::{sync::mpsc, thread};

use pyo3::prelude::*;
use query_handler::{
    core::{ResultConsumer, ResultProducer},
    utils::run_query,
};

#[pyfunction]
pub fn start_query(py: Python<'_>) -> PyResult<ResultConsumer> {
    let (producer, consumer) = mpsc::channel::<String>();
    let result_producer = ResultProducer(producer);
    let result_consumer = ResultConsumer(consumer);

    // start the query in another thread, so downstream query result consumer can work asynchronously
    py.allow_threads(move || {
        thread::spawn(|| {
            run_query(result_producer);
        })
    });

    // return the receiving end of the channel, so downstream query result consumer can read the result
    Ok(result_consumer)
}

#[pymodule]
fn pyo3_dataframe(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(start_query, m)?)?;
    Ok(())
}
