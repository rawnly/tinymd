use crate::markdown_core::parser;

pub fn open(output_line: &mut String) {
  output_line.push_str("<ul>\n")
}

pub fn add_item(output_line: &mut String, text: &str) {
  output_line.push_str(format!("\t<li>{}</li>\n", parser::parse_markdown_row(text)).as_str())
}

pub fn close(output_line: &mut String) {
  output_line.push_str("</ul>\n")
}
