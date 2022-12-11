// https://pyo3.rs/v0.17.1/class/protocols.html#iterable-objects

use pyo3::prelude::*;
use std::sync::mpsc::{Receiver, Sender};

#[pyclass]
pub struct ResultConsumer(pub Receiver<String>);

// implement a python iterator
#[pymethods]
impl ResultConsumer {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(slf: PyRef<'_, Self>) -> Option<String> {
        slf.0.recv().ok()
    }
}

#[derive(Clone)]
pub struct ResultProducer(pub Sender<String>);

impl ResultProducer {
    pub fn add_result(&mut self, result: String) {
        self.0.send(result).unwrap();
    }
}
