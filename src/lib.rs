use pyo3::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;
use regex::Regex;

#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn read_lines_from_file(path: &str, lineno: usize, context_lines: usize) -> Vec<String> {
    let reader = io::BufReader::new(File::open(path).expect("Cannot open file"));
    let start = cmp::max(0, lineno - context_lines);
    let lines_iter = reader.lines().skip(start).take(context_lines * 2 + 1).map(|l| l.unwrap());
    let mut lines = Vec::<String>::new();
    for line in lines_iter {
        lines.push(line.to_string())
    }
    lines
}



fn is_library_frame(absolute_path: String, include_paths: Vec<String>, exclude_paths: Vec<String>) -> bool {
    if (!include_paths.is_empty()) && get_path_regex(include_paths).is_match(&absolute_path) {
        return false
    } else {
        if (!exclude_paths.is_empty()) && get_path_regex(exclude_paths).is_match(&absolute_path) {
            return true
        } else {
            return false
        }
    }
}

// this will need to do something better, like construct regex only once for each set of paths and return a reference
fn get_path_regex(paths: Vec<String>) -> Regex {
    Regex::new(&paths.join("|")).unwrap()
}

#[pymodule]
fn elasticapmspeedups(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_lines_from_file, m)?)?;
    Ok(())
}
