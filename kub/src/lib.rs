//! # kub
//!
//! `kub` is a collection of cli tools that can be useful when used as init containers in Kubernetes.
//
// /// Adds one to the number given.
// use std::error::Error;
// use std::fs;
//
// pub struct Config {
//     pub query: String,
//     pub file_path: String,
// }
//
// impl Config {
//     pub fn build(
//         mut args: impl Iterator<Item = String>,
//     ) -> Result<Config, &'static str> {
//
//         args.next();
//
//         let query = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get a query string"),
//         };
//
//         let file_path = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get a file path"),
//         };
//
//         Ok(Config { query, file_path })
//     }
// }
//
// /// Adds one to the number given.
// ///
// /// # Examples
// ///
// /// ```
// /// let config = Config{"", ""};
// /// let answer = kub::run(config);
// ///
// /// assert_eq!(6, answer);
// /// ```
// pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;
//
//     // println!("With text:\n{contents}");
//
//     let search = search(&config.query, &contents);
//     for foo in search {
//         println!("{foo}")
//     }
//
//     Ok(())
// }
//
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//
//     let collect = contents.lines().filter(|line| line.contains("query")).collect();
//
//     // collect
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn one_result() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.";
//
//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }
// }