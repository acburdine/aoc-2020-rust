use std::fs;

pub fn read_input<T, F>(file: &str, f: F) -> Vec<T>
where
    F: FnMut(&str) -> T,
{
    fs::read_to_string(file)
        .expect("unable to read file")
        .lines()
        .map(f)
        .collect()
}
