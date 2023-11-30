use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    pub input: std::path::PathBuf,
}

impl Cli {
    pub fn line_reader(&self) -> impl Iterator<Item = String> + '_ {
        let f = File::open(&self.input).unwrap();

        BufReader::new(f).lines().map(|l| l.unwrap())
    }

    pub fn input_string(&self) -> String {
        read_to_string(&self.input).unwrap()
    }
}

pub fn parse() -> Cli {
    Cli::parse()
}
