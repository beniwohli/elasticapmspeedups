use pyo3::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;
use regex::Regex;
use std::collections::HashMap;

#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn read_lines_from_file(path: &str, lineno: usize, context_lines: usize) -> (Vec<String>, String, Vec<String>) {
    let lineno_zero = lineno - 1;
    let lower_bound = match lineno_zero.checked_sub(context_lines) {
        None => 0,
        Some(i) => i
    };
    let upper_bound = lineno_zero + context_lines;
    let offset = lineno_zero - lower_bound;
    let lines = get_lines(path, lower_bound, upper_bound + 1);
    let pre_context = &lines[0..offset];
    let main_line = &lines[offset];
    let mut post_context = &Vec::<String>::new();
    if lines.len() > offset {
        let post_context = &lines[offset + 1..];
    } 
    (pre_context.to_vec(), main_line.to_string(), post_context.to_vec())
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

#[pyfunction]
fn dictate(dictish: &PyAny) -> HashMap<String, String> {
    let mut result = HashMap::<String, String>::new();
    let mut keys = Vec::<String>::new();
    if dictish.hasattr("iterkeys").unwrap() {
        keys = dictish.getattr("iterkeys").unwrap().extract().unwrap();
    } else {
        keys = dictish.getattr("keys").unwrap().extract().unwrap();
    }
    for key in keys {
        result.insert(key.to_string(), dictish.getattr(key.to_string()).unwrap().to_string());
    }
    result
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
    m.add_function(wrap_pyfunction!(dictate, m)?)?;
    Ok(())
}
