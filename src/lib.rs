pub mod check;

use crate::check::{all_item_keys, parse_csv, parse_xlsx, transform, write_csv};
use serde::Deserialize;
use std::fs;

use clap::Parser;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Mapping {
    pub fixed_header: Vec<String>,
    pub flat_item_key: String,
    pub flat_item_value: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "mapping.toml")]
    pub mapping_file: String,
    pub check_file: String,
}

pub fn parse_mapping_file(mapping_file: &str) -> Mapping {
    let text: String = fs::read_to_string(mapping_file).unwrap_or_else(|e| {
        panic!(
            "Something went wrong reading file {}.\n{:?}",
            mapping_file, e
        )
    });
    let mapping: Mapping =
        toml::from_str(&text).unwrap_or_else(|e| panic!("format error {}.\n{:?}", mapping_file, e));

    mapping
}

pub fn run(args: Args) {
    let mapping = parse_mapping_file(&args.mapping_file);
    println!("Using {:?}", mapping);
    assert!(
        !mapping.fixed_header.is_empty(),
        "fixed_header must have at least 1 column."
    );

    assert!(
        args.check_file.ends_with(".csv") || args.check_file.ends_with(".xlsx"),
        "only 'csv' or 'xlsx' file can be parsed."
    );
    println!("Reading '{}' ...", args.check_file);
    let checks = if args.check_file.ends_with(".xlsx") {
        parse_xlsx(&args.check_file, mapping.clone()).unwrap()
    } else {
        parse_csv(&args.check_file, mapping.clone()).unwrap()
    };

    println!("Transforming ...");
    let item_keys = all_item_keys(checks.clone(), mapping.clone());
    let flatted_map = transform(checks, mapping.clone());

    let write_to = format!("{}.csv", args.check_file);

    let res = write_csv(&write_to, flatted_map, item_keys, mapping);
    assert!(res.is_ok());
    println!("Transformed to '{}'", write_to);
}
