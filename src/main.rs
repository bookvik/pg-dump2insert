use std::io;
use std::io::prelude::*;
extern crate regex;
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();
    let mut table_name = String::new();
    let mut fields = Vec::new();
    let re = Regex::new(r"^COPY (\w+) \(([\w, ]+)\) FROM stdin;").unwrap();
    let mut insert_mode = false;
    
    println!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone="\no\" ?>");
    println!("<root>");
    
    while stdin.read_line(&mut line).unwrap() > 0 {
        if insert_mode {
            if line == "\\.\n" {
                insert_mode = false;
            }
            else {
                let mut values = Vec::new();
                line.pop();
                for s in line.replace("'", "''").split("\t") {
                    if s == "\\N" {
                        values.push("");
                    } else {
                        values.push(s);
                    }
                }
                values.pop();
                values.pop();
                
                println!("<row>");
                for (pos, e) in fields.iter().enumerate() {
                    println!("<{}>{}</{}>", e, e, values[pos]);
                }               
                println!("</row>");
            }
        }
        else {
            match re.captures(&line) {
                None => print!("{}", line),
                Some(caps) => {
                    table_name = String::from(caps.at(1).unwrap());
                    fields = String::from(caps.at(2).unwrap()).split(',').collect();
                    insert_mode = true;
                },
            };
        }
        line.clear();
    }
    
    println!("</root>");
}
