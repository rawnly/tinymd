pub fn parse_wrapper(
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

pub fn parse_markdown_row(row: &str) -> String {
    let mut output = String::new();

    let mut is_bold = false;
    let mut is_italic = false;
    let mut is_code = false;
    let mut is_underline = false;

    // Anchor tag stuff

    let mut is_image = false;

    let mut is_link = false;
    let mut is_link_text = false;
    let mut is_link_url = false;

    let mut link_text = String::new();
    let mut link_url = String::new();

    for (idx, ch) in row.char_indices() {
        let char = String::from(ch);

        let is_last_char = idx == row.len() - 1;
        let is_first_char = idx == 0;

        let next_char : char;
        let prev_char : char;


        match &char[..] {
            "!" => {
                if is_last_char || is_code {
                    output.push_str(&char);
                    continue;
                }

                next_char = row
                        .chars()
                        .nth(idx + 1)
                        .unwrap();

                if next_char == '[' {
                    is_image = true;
                }
            },
            "[" => {
                if is_last_char || is_code {
                    output.push_str(&char);
                    continue;
                }

                if !is_link {
                    is_link = true;
                    is_link_text = true;
                    continue;
                }

                output.push_str(&char);
            },
            "]" => {
                if is_last_char || is_code {
                    output.push_str(&char);
                    continue;
                }

                if is_link_text && (is_link || is_image) {
                    is_link_text = false;
                    continue;
                }

                output.push_str(&char);
            },
            "(" => {
                if is_last_char || is_code {
                    output.push_str(&char);
                    continue;
                }

                if !is_link_url && (is_link || is_image) {
                    is_link_url = true;
                    continue;
                }

                output.push_str(&char);
            },
            ")" => {
                if is_image {
                    if is_link_url {
                        is_link_url = false;
                    }


                    is_image = false;
                    output.push_str(&format!("<img src=\"{}\" alt=\"{}\" />", link_url, link_text));
                    continue;
                }

                if is_link {
                    if is_link_url {
                        is_link_url = false;
                    }

                    is_link = false;
                    output.push_str(&format!("<a href=\"{}\">{}</a>", link_url, link_text));
                    continue;
                }

                output.push_str(&char);
            },
            "`" =>
                parse_wrapper(
                    is_first_char,
                    is_last_char,
                    &mut output,
                    "<code>",
                    "</code>",
                    &mut is_code
                )
            ,
            "_" =>
                parse_wrapper(
                    is_first_char,
                    is_last_char,
                    &mut output,
                    "<u>",
                    "</u>",
                    &mut is_underline
                )
            ,
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
            _ => {
                if is_link {
                    if is_link_text {
                        link_text.push_str(&char);
                    }

                    if is_link_url {
                        link_url.push_str(&char);
                    }

                    continue;
                }

                output.push_str(&char)
            }
        }
    }

    return output;
}
