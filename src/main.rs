use std::{io::Write, path::Path};
use std::fs::File;
use std::io::{ BufRead, BufReader };

fn get_title() -> String {
    let mut title : String = String::from( env!("CARGO_PKG_NAME") );

    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));

    return title;
}

fn print_short_banner() {
    println!("{}", get_title());
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

fn get_heading_size(heading: &str) -> usize {
    let size = heading
        .chars()
        .filter(|char| char.eq(&'#'))
        .count();

    if size > 6 {
        return 6;
    }

    return size;
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

// CODE
fn open_code_block(output_line: &mut String) {
    output_line.push_str("<pre>\n");
}

fn open_code_block_with_lang(output_line: &mut String, lang: &str) {
    output_line.push_str(format!("<pre lang=\"{}\">\n", lang).as_str());
}

fn close_code_block(output_line: &mut String) {
    output_line.push_str("</pre>\n");
}

fn add_code_to_block(output_line: &mut String, code: &str) {
    output_line.push_str(format!("{}\n", code).as_str())
}

// LIST
fn open_list(output_line: &mut String) {
    output_line.push_str("<ul>\n")
}

fn add_list_item(output_line: &mut String, text: &str) {
    output_line.push_str(format!("\t<li>{}</li>\n", parse_markdown_row(text)).as_str())
}

fn close_list(output_line: &mut String) {
    output_line.push_str("</ul>\n")
}

fn parse_wrapper(
    is_first_char: bool,
    is_last_char: bool,
    output: &mut String,
    html_tag_open: &str,
    html_tag_close: &str,
    is_open: &mut bool
) {
    if is_first_char {
        output.push_str(html_tag_open);
    } else if is_last_char {
        output.push_str(html_tag_close);
    }

    if *is_open {
        *is_open = false;
        output.push_str(html_tag_close);
    } else {
        *is_open = true;
        output.push_str(html_tag_open);
    }
}

fn parse_markdown_row(row: &str) -> String {
    let mut output = String::new();

    let mut is_bold = false;
    let mut is_italic = false;
    let mut is_code = false;
    let mut is_underline = false;

    for (idx, ch) in row.char_indices() {
        let char = String::from(ch);

        let is_last_char = idx == row.len() - 1;
        let is_first_char = idx == 0;

        let next_char : char;
        let prev_char : char;

        match &char[..] {
            "`" => {
                parse_wrapper(
                    is_first_char,
                    is_last_char,
                    &mut output,
                    "<code>",
                    "</code>",
                    &mut is_code
                );
            },
            "_" => {
                parse_wrapper(
                    is_first_char,
                    is_last_char,
                    &mut output,
                    "<u>",
                    "</u>",
                    &mut is_underline
                );
            },
            "*" => {
                if is_first_char {
                    next_char = row
                        .chars()
                        .nth(idx + 1)
                        .unwrap();

                    if next_char == '*' {
                        continue;
                    }

                    output.push_str("<i>");
                    is_italic = true;
                    continue;
                }

                if is_last_char {
                    if is_bold {
                        is_bold = false;
                        output.push_str("</b>");
                        continue;
                    }

                    if is_italic {
                        is_italic = false;
                        output.push_str("</i>");
                        continue;
                    }

                    continue;
                }

                prev_char = row
                    .chars()
                    .nth(idx - 1)
                    .unwrap();

                next_char = row
                    .chars()
                    .nth(idx + 1)
                    .unwrap();


                if next_char == '*' {
                    continue;
                }

                if prev_char == '*' && !is_bold {
                    is_bold = true;
                    output.push_str("<b>");
                    continue;
                }

                if is_bold {
                    is_bold = false;
                    output.push_str("</b>");
                    continue;
                }

                if !is_bold  && !is_italic {
                    is_italic = true;
                    output.push_str("<i>");
                    continue;
                }

                if is_italic {
                    is_italic = false;
                    output.push_str("</i>");
                    continue;
                }

                output.push_str(&char);
            },
            _ => output.push_str(&char)
        }
    }

    return output;
}

fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[INFO] Trying to parse {}", _filename);
    println!("\n");

    let input_filename = Path::new(_filename);
    let output_filename = format!("{}.html", &_filename[.._filename.len() - 3]);

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
                    close_list(&mut output_line);
                    is_list = false;
                }

                if is_code {
                    is_code = false;
                    close_code_block(&mut output_line);
                } else {
                    is_code = true;

                    let is_code_block = line_contents[..3]
                        .chars()
                        .all(|char| char.eq(&'`'));

                    if is_code_block {
                        let lang: &str = &line_contents[3..];
                        open_code_block_with_lang(&mut output_line, lang);
                    } else {
                        open_code_block(&mut output_line);
                    }
                }
            },
            Some('-') => {
                if is_code {
                    close_code_block(&mut output_line);
                    is_code = false;
                }

                if !is_list {
                    is_list = true;
                    open_list(&mut output_line);
                }

                add_list_item(&mut output_line, &line_contents[1..].trim_start())
            },
            Some('#') => {
                if is_code {
                    close_code_block(&mut output_line);
                    is_code = false;
                }

                if is_list {
                    close_list(&mut output_line);
                    is_list = false;
                }

                htag_size = get_heading_size(&line_contents);

                check_tags(&mut _htag, &mut _ptag, &mut output_line, htag_size);

                _htag = true;
                output_line.push_str(format!("<h{}>", htag_size).as_str());
                output_line.push_str(&parse_markdown_row(&line_contents[htag_size..].trim_start()));
            },
            _ => {
                if is_code {
                    add_code_to_block(&mut output_line, &line_contents)
                } else {
                    if is_list {
                        close_list(&mut output_line);
                        is_list = false;
                    }

                    if !_ptag {
                        _ptag = true;
                        output_line.push_str("<p>\n");
                    }

                    output_line.push_str(format!("\t{}", parse_markdown_row(&line_contents)).as_str());
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

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => {
            let filename = &args[1];

            if !filename.ends_with(".md") {
                println!("[ ERROR ] Not a valid ")
            }

            parse_markdown_file(filename)
        },
        _ => {
            println!("[ ERROR ] Invalid invocation!");
            usage()
        }
    }
}
