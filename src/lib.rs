use pyo3::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;
use regex::Regex;

#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn read_lines_from_file(path: &str, lineno: usize, context_lines: usize) -> (Vec<String>, String, Vec<String>) {
    let start = cmp::max(0, lineno - context_lines);
    let pre_context = get_lines(path, start, context_lines);
    let main_line = get_lines(path, (start + context_lines), 1).first().unwrap().to_string();
    let post_context = get_lines(path, (start + context_lines + 1), context_lines);
    (pre_context, main_line, post_context)
}

fn get_lines(path: &str, skip:usize, count: usize) -> Vec<String> {
    let reader = io::BufReader::new(File::open(path).expect("Cannot open file"));
    let lines = reader.lines().skip(skip).take(count).map(|l| l.unwrap());
    let mut values = Vec::<String>::new();
    for line in lines {
        values.push(line.to_string())
    }
    values
}

#[pyfunction]
fn walk_stack(frame: &PyAny) -> Vec<&PyAny> {
    let mut frames = Vec::<&PyAny>::new();
    let mut current_frame: &PyAny = frame;
    frames.push(current_frame);
    while current_frame.hasattr("f_back").unwrap() {
        frames.push(current_frame);
        current_frame = current_frame.getattr("f_back").unwrap()
    }
    frames
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
    m.add_function(wrap_pyfunction!(walk_stack, m)?)?;
    Ok(())
}
