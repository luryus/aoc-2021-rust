use itertools::Itertools;
use ndarray::Array2;
use num_integer::Integer;
use regex::Regex;
use std::io::{self, Read};
use std::iter::Iterator;
use std::str::FromStr;

pub fn get_input_filename() -> Option<String> {
    let args: Vec<_> = std::env::args().collect();
    match args.len() {
        2 => args.into_iter().nth(1),
        1 => None,
        _ => panic!("Invalid number of arguments ({})", args.len() - 1),
    }
}

pub fn read_input_string() -> io::Result<String> {
    match get_input_filename() {
        Some(path) => std::fs::read_to_string(path),
        None => read_stdin_to_string(),
    }
}

pub fn read_stdin_to_string() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.to_owned())
}

pub fn read_input_lines() -> io::Result<Vec<String>> {
    match get_input_filename() {
        Some(path) => read_file_lines(&path),
        None => read_stdin_lines(),
    }
}

pub fn read_file_lines(filename: &str) -> io::Result<Vec<String>> {
    let input = std::fs::read_to_string(filename)?;
    Ok(input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(l.to_owned())
            }
        })
        .collect())
}

pub fn read_stdin_lines() -> io::Result<Vec<String>> {
    let input = read_stdin_to_string()?;
    Ok(input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(l.to_owned())
            }
        })
        .collect())
}

pub fn read_input_ints<T: Integer + FromStr>(signed: bool) -> io::Result<Vec<T>> {
    match get_input_filename() {
        Some(path) => read_ints_from_file(&path, signed),
        None => read_ints_from_stdin(signed),
    }
}

pub fn read_ints_from_stdin<T: Integer + FromStr>(signed: bool) -> io::Result<Vec<T>> {
    let s = read_stdin_to_string()?;
    Ok(read_ints_from_string(&s, signed))
}

pub fn read_ints_from_file<T: Integer + FromStr>(
    filename: &str,
    signed: bool,
) -> io::Result<Vec<T>> {
    let s = std::fs::read_to_string(filename)?;
    Ok(read_ints_from_string(&s, signed))
}

pub fn read_ints_from_string<T: Integer + FromStr>(s: &str, signed: bool) -> Vec<T> {
    let re = Regex::new(if signed { r"-?\d+" } else { r"\d+" }).unwrap();
    re.find_iter(s)
        .map(|m| m.as_str())
        .filter_map(|m| m.parse::<T>().ok())
        .collect::<Vec<T>>()
}

pub fn read_input_regex_matches(regex_pattern: &str) -> io::Result<Vec<String>> {
    match get_input_filename() {
        Some(path) => read_regex_matches_from_file(&path, regex_pattern),
        None => read_regex_matches_from_stdin(regex_pattern),
    }
}

pub fn read_regex_matches_from_stdin(regex_pattern: &str) -> io::Result<Vec<String>> {
    let s = read_stdin_to_string()?;
    let matches = read_regex_matches_from_string(&s, regex_pattern);

    let res = matches.into_iter().map(|sm| sm.to_owned()).collect();

    Ok(res)
}

pub fn read_regex_matches_from_file(
    filename: &str,
    regex_pattern: &str,
) -> io::Result<Vec<String>> {
    let s = std::fs::read_to_string(filename)?;
    let matches = read_regex_matches_from_string(&s, regex_pattern);

    let res = matches.into_iter().map(|sm| sm.to_owned()).collect();

    Ok(res)
}

pub fn read_regex_matches_from_string<'a>(s: &'a str, regex_pattern: &str) -> Vec<&'a str> {
    let re = Regex::new(regex_pattern).unwrap();
    re.find_iter(s).map(|m| m.as_str()).collect()
}

pub fn split_to_tuple2<'a>(s: &'a str, pattern: &str) -> Option<(&'a str, &'a str)> {
    s.split_once(pattern)
}

pub fn split_to_tuple3<'a>(s: &'a str, pattern: &str) -> Option<(&'a str, &'a str, &'a str)> {
    let parts = s.splitn(3, pattern);
    parts.collect_tuple()
}

pub fn split_to_tuple4<'a>(
    s: &'a str,
    pattern: &str,
) -> Option<(&'a str, &'a str, &'a str, &'a str)> {
    let parts = s.splitn(4, pattern);
    parts.collect_tuple()
}

#[test]
fn test_read_ints_from_string() {
    let s = "a123b22 123x02\n123";
    let res: Vec<i32> = read_ints_from_string(s);

    assert_eq!(5, res.len());
    assert_eq!(vec![123i32, 22, 123, 02, 123], res);
}

#[test]
fn test_read_regex_matches_from_string() {
    let s = "0.12,1.23,4.2\n111.1,.,111.,.23";
    let re = r"\d+\.\d+";

    let m = read_regex_matches_from_string(s, re);
    assert_eq!(vec!["0.12", "1.23", "4.2", "111.1"], m);
}

pub trait UnwrapOptionIterator<T> {
    type Output: Iterator<Item = T>;
    fn unwrap_options(self) -> Self::Output;
}

impl<T, I: Iterator<Item = Option<T>>> UnwrapOptionIterator<T> for I {
    type Output = std::iter::Map<Self, fn(Option<T>) -> T>;

    fn unwrap_options(self) -> Self::Output {
        self.map(|x| x.unwrap())
    }
}
