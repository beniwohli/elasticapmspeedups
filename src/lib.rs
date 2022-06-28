use pyo3::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn read_lines_from_file(path: &str, lineno: usize, context_lines: usize) -> () {
    let reader = io::BufReader::new(File::open("input").expect("Cannot open file"));
    let start = cmp::max(0, lineno - context_lines);
    println!("something spicy")
    
}

#[pyfunction]
fn do_a_hello_world() -> PyResult<String> {
    Ok("hello world".to_string())
}

#[pymodule]
fn elasticapmspeedups(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_lines_from_file, m)?)?;
    m.add_function(wrap_pyfunction!(do_a_hello_world, m)?)?;
    Ok(())
}
