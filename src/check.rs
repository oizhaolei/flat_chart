use csv::{Reader as CsvReader, Writer};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use calamine::{open_workbook, Reader, Xlsx};
use itertools::Itertools;

use crate::Mapping;

fn valid_headers(mapping: Mapping, headers: Vec<String>) {
    assert!(
        mapping
            .fixed_header
            .iter()
            .all(|f| headers.iter().find(|&h| h == f).is_some())
            && headers
                .iter()
                .find(|&h| h == &mapping.flat_item_key)
                .is_some()
            && headers
                .iter()
                .find(|&h| h == &mapping.flat_item_value)
                .is_some(),
        "'mapping.toml' definition is invalid for data file."
    );
}

pub fn parse_csv(
    check_file: &str,
    mapping: Mapping,
) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let mut checks: Vec<HashMap<String, String>> = Vec::new();
    let mut rdr = CsvReader::from_path(check_file)?;
    let headers: Vec<String> = rdr.headers()?.into_iter().map(|f| f.to_string()).collect();
    println!("Header: {:?}", headers);
    //header valid check
    valid_headers(mapping, headers.clone());

    let mut rdr = CsvReader::from_path(check_file)?;
    for result in rdr.records() {
        let record = result?;
        let mut line: HashMap<String, String> = HashMap::new();
        for col in 0..record.len() {
            line.insert(
                headers.get(col).unwrap().to_string(),
                record.get(col).unwrap().trim().to_string(),
            );
        }
        checks.push(line.clone());
    }
    println!("(row, col): ({}, {})", checks.len(), headers.len());
    Ok(checks)
}
pub fn parse_xlsx(
    check_file: &str,
    mapping: Mapping,
) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    // opens a new workbook
    let mut workbook: Xlsx<_> = open_workbook(check_file)
        .unwrap_or_else(|e| panic!("Cannot open file {}.\n{:?}", check_file, e));

    let mut checks: Vec<HashMap<String, String>> = Vec::new();

    // Read whole worksheet data and provide some statistics
    if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
        //header
        let mut headers: Vec<String> = Vec::new();
        for col in 0..=range.get_size().1 {
            let cell = range.get_value((0, col as u32));
            if let Some(value) = cell {
                if let Some(t) = value.get_string() {
                    headers.push(t.to_string());
                }
            }
        }
        //  header valid check
        valid_headers(mapping, headers.clone());
        println!("Header: {:?}", headers);
        println!("(row, col): {:?}", range.get_size());

        //data
        for row in 1..range.get_size().0 {
            let mut line: HashMap<String, String> = HashMap::new();
            for col in 0..=range.get_size().1 {
                let cell = range.get_value((row as u32, col as u32));
                if let Some(value) = cell {
                    if let Some(t) = value.get_string() {
                        line.insert(headers.get(col).unwrap().to_string(), t.trim().to_string());
                    }
                }
            }
            checks.push(line.clone());
        }
    }
    Ok(checks)
}

pub fn write_csv(
    write_to: &str,
    map: HashMap<Vec<String>, HashMap<String, String>>,
    all_item_keys: Vec<String>,
    mapping: Mapping,
) -> Result<(), Box<dyn Error>> {
    let fixed_header = mapping.fixed_header;

    let mut wtr = Writer::from_path(write_to)?;
    //fixed header
    for col in fixed_header.into_iter() {
        wtr.write_field(col)?;
    }
    //dynamic header
    for col in all_item_keys.clone().into_iter() {
        wtr.write_field(col)?;
    }
    wtr.write_record(None::<&[u8]>)?;

    for (key, items) in map.into_iter() {
        //fixed data
        for v in key.into_iter() {
            wtr.write_field(v)?;
        }
        //dynamic data
        for col in all_item_keys.clone().into_iter() {
            let f = items.get(&col.to_string());
            if let Some(v) = f {
                wtr.write_field(v)?;
            } else {
                wtr.write_field("")?;
            }
        }
        wtr.write_record(None::<&[u8]>)?;
    }
    wtr.flush()?;
    Ok(())
}

//all item keys
pub fn all_item_keys(checks: Vec<HashMap<String, String>>, mapping: Mapping) -> Vec<String> {
    let mut vec: Vec<String> = checks
        .into_iter()
        .map(|line| {
            let item_key = line.get(&mapping.flat_item_key);
            if let Some(key) = item_key {
                key.to_string()
            } else {
                String::from("")
            }
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    vec.sort();
    vec
}

pub fn transform(
    checks: Vec<HashMap<String, String>>,
    mapping: Mapping,
) -> HashMap<Vec<String>, HashMap<String, String>> {
    //grouping
    let map: HashMap<Vec<String>, Vec<HashMap<String, String>>> =
        checks.into_iter().into_group_map_by(|check| {
            let key: Vec<String> = mapping
                .fixed_header
                .clone()
                .into_iter()
                .map(|h| {
                    let value = check.get(&h);
                    if let Some(text) = value {
                        text.to_string()
                    } else {
                        String::from("")
                    }
                })
                .collect();
            key
        });

    //flat
    let flatted_map: HashMap<Vec<String>, HashMap<String, String>> = map
        .into_iter()
        .map(|(key, vec)| {
            (
                key,
                vec.into_iter()
                    .map(|hm: HashMap<String, String>| {
                        let key = hm.get(&mapping.flat_item_key).unwrap();
                        let value = hm.get(&mapping.flat_item_value).unwrap();
                        (key.to_string(), value.to_string())
                    })
                    .collect::<HashMap<String, String>>(),
            )
        })
        .collect();
    flatted_map
}
