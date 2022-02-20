use std::collections::HashSet;
use std::fs::File;
use anyhow::{Context, Result};

use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
//
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug, Default)]
pub struct Flags {
    print_line_number: bool,
    print_file_name_only: bool,
    match_case_insensitive: bool,
    invert_match: bool,
    match_full_line: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut instance = Flags::default();
        for x in flags {
            match *x {
                "-x"=> instance.match_full_line = true,
                "-n"=> instance.print_line_number = true,
                "-l"=> instance.print_file_name_only = true,
                "-v"=> instance.invert_match = true,
                "-i"=> instance.match_case_insensitive=true,
                _ => {
                    panic!("Invalid command line parameter.")
                },
            }
        }
        instance
    }
}

pub fn grep(pattern: &str, flag: &Flags, files: &[&str]) -> Result<Vec<String>> {
    let reg = make_regex(pattern, flag)?;

    let (errs, succs): (Vec<Result<Vec<String>>>, Vec<Result<Vec<String>>>) = files.into_iter().map(
        |x| grep_file(&reg, flag, *x, files.len()>1)
    ).partition(|x| x.is_err());

    if errs.len() > 0 {
        Err(errs.into_iter().next().unwrap()
            .expect_err("Error should have been partitioned."))
    }else {
        let res : Vec<String> = succs.into_iter()
            .flat_map(|x|x.expect("Successful result should have been partitioned"))
            .collect();
        Ok(res)
    }
}

fn grep_file(reg: &Regex, flag: &Flags, file: &str, multiple_file: bool) -> Result<Vec<String>> {
    let lines = read_lines(file)
        .context(format!("Error reading file: {}", file))?;

    let strs: Vec<String> = lines.into_iter()
        .filter_map(|x|x.ok())
        .enumerate()
        .filter_map(|(x, line)|{
            match_and_print_line(&reg, flag, file, x, line, multiple_file)
        }).collect();

    if flag.print_file_name_only { // remove duplicate
        let sets = strs.into_iter().collect::<HashSet<String>>();
        Ok(sets.into_iter().collect::<Vec<String>>())
    } else {
        Ok(strs)
    }
}

fn make_regex(pattern: &str, flag: &Flags) -> std::result::Result<Regex, regex::Error> {
    let pat = if flag.match_full_line {
        format!("^{}$", pattern)
    } else {
        pattern.to_string()
    };

    let pat = if flag.match_case_insensitive {
        format!("(?i){}",pat)
    } else {
        pat
    };
    return Regex::new(pat.as_str());
}

fn match_and_print_line(reg: &Regex, flag: &Flags, file: &str, line_number:usize,
                        line: String, multiple_file: bool)-> Option<String> {
    let matched = match_line(&reg, &line, flag.invert_match);
    if matched {
        let str = print_matched_line(&line, line_number, file,
                                     flag.print_line_number, flag.print_file_name_only,
                                     multiple_file);
        Option::Some(str)
    }else {
        Option::None
    }

}

fn print_matched_line(line: &String, line_number: usize, filename: &str, print_line_number: bool,
                      print_file_name_only: bool, multiple_file: bool) -> String {
    if print_file_name_only {
        filename.to_string()
    } else {
      if print_line_number {
          if multiple_file {
              format!("{}:{}:{}", filename, line_number+1, line)
          }else {
              format!("{}:{}", line_number+1, line)
          }
      }else {
          if multiple_file {
              format!("{}:{}", filename, line)
          } else {
              format!("{}",  line)
          }
      }
    }
}

fn match_line(pattern: &Regex, line: &String, match_invert: bool) -> bool {
    let matched = pattern.is_match(line.as_str());
    if match_invert {
        !matched
    } else {
        matched
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
