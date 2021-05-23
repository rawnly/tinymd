pub fn open(output_line: &mut String) {
  output_line.push_str("<pre>\n");
}

pub fn open_with_lang(output_line: &mut String, lang: &str) {
  output_line.push_str(format!("<pre lang=\"{}\">\n", lang).as_str());
}

pub fn close(output_line: &mut String) {
  output_line.push_str("</pre>\n");
}

pub fn add_code(output_line: &mut String, code: &str) {
  output_line.push_str(format!("{}\n", code).as_str())
}
