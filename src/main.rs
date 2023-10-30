use clap::Parser;
extern crate html;
use serde_json::Value;
use serde_yaml::{from_str as yaml_from_str, to_string as yaml_to_string};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Parser)]
#[command(
    author = "Thanachote Wattanamanikul <66011260@kmitl.ac.th>",
    version = "0.1.0",
    about = "File Converter"
)]
struct CMD {
    #[arg(long, short)]
    file_name: String,
    #[arg(long, short)]
    conversion_file: String,
    #[arg(long, short)]
    output_file: String,
}

fn json_to_yaml(json_content: &str) -> Result<String, Box<dyn Error>> {
    let json_data: Value = serde_json::from_str(json_content)?; // Parse JSON string into a Value.
    let yaml_content = yaml_to_string(&json_data)?; // Convert the JSON Value to a YAML string.
    Ok(yaml_content) // Return the YAML content as a Result.
}

// Function to convert YAML to JSON and return a pretty-printed JSON string.
fn yaml_to_json(yaml_content: &str) -> Result<String, Box<dyn Error>> {
    let json_data: Value = yaml_from_str(yaml_content)?; // Parse YAML string into a JSON Value.
    let pretty_json = serde_json::to_string_pretty(&json_data)?; // Convert the JSON Value to a pretty-printed JSON string.
    Ok(pretty_json) // Return the pretty-printed JSON content as a Result.
}

fn csv_to_html(csv_content: &str) -> Result<String, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_content.as_bytes());
    let headers = rdr.headers()?.clone();
    let mut records = Vec::new();

    for result in rdr.records() {
        let record = result?;
        records.push(record);
    }

    let mut table = String::new();
    table.push_str("<table>\n");

    table.push_str("<tr>\n");
    for header in headers.iter() {
        table.push_str(&format!("<th>{}</th>\n", header));
    }
    table.push_str("</tr>\n");

    // Table data
    for record in records.iter() {
        table.push_str("<tr>\n");
        for field in record.iter() {
            table.push_str(&format!("<td>{}</td>\n", field));
        }
        table.push_str("</tr>\n");
    }

    table.push_str("</table>");

    Ok(table)
}

fn csv_to_svg(csv_content: &str) -> String {
    let mut svg = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg width="400" height="200" xmlns="http://www.w3.org/2000/svg">
<rect width="100%" height="100%" fill="white" />
"#,
    );

    let rows: Vec<&str> = csv_content.split('\n').collect();
    let mut y = 30;

    for (row_index, row) in rows.iter().enumerate() {
        let cols: Vec<&str> = row.split(',').collect();
        let mut x = 10;

        for (col_index, col) in cols.iter().enumerate() {
            if row_index == 0 {
                // Header row
                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" font-size="12" fill="black">{}</text>"#,
                    x, y, col
                ));
            } else {
                // Data row
                svg.push_str(&format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"70\" height=\"20\" fill=\"#f0f0f0\" stroke=\"black\" stroke-width=\"1\" />
<text x=\"{}\" y=\"{}\" font-size=\"12\" fill=\"black\">{}</text>",
                    x, y, x + 5, y + 15, col
                ));
            }

            x += 80;
        }

        y += 30;
    }

    svg.push_str("</svg>");
    svg
}


fn txt_to_asc(txt_content: &str) -> Result<String, Box<dyn Error>> {
    let mut asc_content = String::new();
    for c in txt_content.chars() {
        let ascii_value = c as u32;
        asc_content.push_str(&format!("{:03} ", ascii_value));
    }
    Ok(asc_content)
}

fn asc_to_txt(asc_content: &str) -> Result<String, Box<dyn Error>> {
    let mut txt_content = String::new();
    let ascii_values: Vec<&str> = asc_content.trim().split(' ').collect();

    for ascii_str in ascii_values {
        if let Ok(ascii_value) = ascii_str.parse::<u32>() {
            if ascii_value <= 127 {
                let c = ascii_value as u8 as char;
                txt_content.push(c);
            } else {
                return Err("Invalid ASCII value".into());
            }
        } else {
            return Err("Invalid ASCII format".into());
        }
    }

    Ok(txt_content)
}

// Main function where the program execution starts.
fn main() -> Result<(), Box<dyn Error>> {
    let cmd = CMD::parse(); // Parse command-line arguments using the 'CMD' struct defined earlier.

    if cmd.conversion_file == "json_to_yaml" {
        // If the conversion type is 'json_to_yaml':
        let mut json_content = String::new();
        File::open(&cmd.file_name)?.read_to_string(&mut json_content)?; // Read the input JSON file into a string.
        let yaml_content = json_to_yaml(&json_content)?; // Convert JSON to YAML.
        let mut yaml_file = File::create(&cmd.output_file)?; // Create and open the output YAML file.
        yaml_file.write_all(yaml_content.as_bytes())?; // Write the YAML content to the output file.
    } else if cmd.conversion_file == "yaml_to_json" {
        // If the conversion type is 'yaml_to_json':
        let mut yaml_content = String::new();
        File::open(&cmd.file_name)?.read_to_string(&mut yaml_content)?; // Read the input YAML file into a string.
        let json_content = yaml_to_json(&yaml_content)?; // Convert YAML to JSON.
        let mut json_file = File::create(&cmd.output_file)?; // Create and open the output JSON file.
        json_file.write_all(json_content.as_bytes())?; // Write the JSON content to the output file.
    } else if cmd.conversion_file == "csv_to_html" {
        let mut csv_content = String::new();
        File::open(&cmd.file_name)?.read_to_string(&mut csv_content)?;
        let html_content = csv_to_html(&csv_content)?;
        let mut html_file = File::create(&cmd.output_file)?;
        html_file.write_all(html_content.as_bytes())?;
    } else if cmd.conversion_file == "csv_to_svg" {
        let mut csv_content = String::new();
        File::open(&cmd.file_name)?.read_to_string(&mut csv_content)?;

        // Call the csv_to_svg function
        let svg_content = csv_to_svg(&csv_content);

        let mut svg_file = File::create(&cmd.output_file)?;
        svg_file.write_all(svg_content.as_bytes())?;
    } else if cmd.conversion_file == "txt_to_asc" {
        let mut txt_content = String::new();
        File::open(&cmd.file_name)?.read_to_string(&mut txt_content)?;
        let asc_content = txt_to_asc(&txt_content)?;
        let mut asc_file = File::create(&cmd.output_file)?;
        asc_file.write_all(asc_content.as_bytes())?;
    } else if cmd.conversion_file == "asc_to_txt" {
        let mut asc_content = String::new();
        File::open(&cmd.file_name)?.read_to_string(&mut asc_content)?;
        let txt_content = asc_to_txt(&asc_content)?;
        let mut txt_file = File::create(&cmd.output_file)?;
        txt_file.write_all(txt_content.as_bytes())?;
    } else {
        eprintln!("Invalid conversion file format. Use 'json_to_yaml' or 'yaml_to_json'.");
        // Print an error message if an invalid conversion type is specified.
    }

    Ok(()) // Return a Result indicating success.
}
