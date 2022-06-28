use pyo3::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn read_lines_from_file(path: &str, lineno: usize, context_lines: usize) -> () {
    let reader = io::BufReader::new(File::open(path).expect("Cannot open file"));
    let start = cmp::max(0, lineno - context_lines);
    let lines_iter = reader.lines().skip(start).take(context_lines * 2 + 1).map(|l| l.unwrap());
    for line in lines_iter {
        println!("{}",line);
    }
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
