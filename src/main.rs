use std::{io::Write, path::Path};
use std::fs::File;
use std::io::{ BufRead, BufReader };

pub mod markdown_core;
use markdown_core::{
    code_block,
    list,
    headings,
    parser
};

pub mod manifest;


fn print_short_banner() {
    println!("{}", manifest::get_title());
}

fn print_long_banner() {
    print_short_banner();

    println!("Written by: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));

    println!("Usage: tinymd <somefile.md>")
}

fn usage() {
    print_long_banner();
}


fn check_tags(_htag: &mut bool, _ptag: &mut bool, output_line: &mut String, size: usize) {
    if *_ptag {
        *_ptag = false;
        output_line.push_str("\n</p>\n")
    }

    if *_htag {
        *_htag = false;
        output_line.push_str(format!("</h{}>\n", size).as_str())
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("[ ERROR ] Invalid invocation!");
        usage();
        return;
    }

    let filename = &args[1];

    if !filename.ends_with(".md") {
        println!("[ ERROR ] Not a valid ");
        return;
    }

    print_short_banner();
    println!("[INFO] Trying to parse {}", filename);
    println!("\n");

    let input_filename = Path::new(filename);
    let output_filename = format!("{}.html", &filename[..filename.len() - 3]);

    let file = match File::open(&input_filename) {
        Err(err) => panic!("Failed to open file. {}", err.to_string()),
        Ok(value) => value,
    };

    let mut _htag : bool = false;
    let mut _ptag : bool = false;
    let mut is_list : bool = false;
    let mut is_code : bool = false;

    let mut htag_size : usize = 1;

    let mut tokens : Vec<String> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap();

        if line_contents == "" {
            continue;
        }

        let mut output_line : String = String::new();
        let mut first_char : Vec<char> = line_contents
            .chars()
            .take(1)
            .collect();

        match first_char.pop() {
            Some('`') => {
                if is_list {
                    list::close(&mut output_line);
                    is_list = false;
                }

                if is_code {
                    is_code = false;
                    code_block::close(&mut output_line);
                } else {
                    is_code = true;

                    let is_code_block = line_contents[..3]
                        .chars()
                        .all(|char| char.eq(&'`'));

                    if is_code_block {
                        let lang: &str = &line_contents[3..];
                        code_block::open_with_lang(&mut output_line, lang);
                    } else {
                        code_block::open(&mut output_line);
                    }
                }
            },
            Some('-') => {
                if is_code {
                    code_block::close(&mut output_line);
                    is_code = false;
                }

                if !is_list {
                    is_list = true;
                    list::open(&mut output_line);
                }

                list::add_item(&mut output_line, &line_contents[1..].trim_start())
            },
            Some('#') => {
                if is_code {
                    code_block::close(&mut output_line);
                    is_code = false;
                }

                if is_list {
                    list::close(&mut output_line);
                    is_list = false;
                }

                htag_size = headings::get_heading_size(&line_contents);

                check_tags(&mut _htag, &mut _ptag, &mut output_line, htag_size);

                _htag = true;
                output_line.push_str(format!("<h{}>", htag_size).as_str());
                output_line.push_str(&parser::parse_markdown_row(&line_contents[htag_size..].trim_start()));
            },
            _ => {
                if is_code {
                    code_block::add_code(&mut output_line, &line_contents)
                } else {
                    if is_list {
                        list::close(&mut output_line);
                        is_list = false;
                    }

                    if !_ptag {
                        _ptag = true;
                        output_line.push_str("<p>\n");
                    }

                    output_line.push_str(format!("\t{}", parser::parse_markdown_row(&line_contents)).as_str());
                }
            }
        }

        check_tags(&mut _htag, &mut _ptag, &mut output_line, htag_size);

        if output_line.trim() != "" {
            tokens.push(output_line);
        }
    }

    if is_list {
        tokens.push(String::from("</ul>\n"))
    }

    if is_code {
        tokens.push(String::from("</pre>\n"))
    }

    let mut outfile = File::create(output_filename)
        .expect("[ERROR] Failed to create the output file");

    for token in tokens {
        outfile
            .write_all(token.as_bytes())
            .expect("[ERROR] Could not write to the output file.");
    }
}
