/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/


pub fn capitalize_first(s: &str) -> String {
    s.chars()
        .take(1)
        .flat_map(|f| f.to_uppercase())
        .chain(s.chars().skip(1))
        .collect()
}