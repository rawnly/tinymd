pub fn get_heading_size(heading: &str) -> usize {
  let size = heading
      .chars()
      .filter(|char| char.eq(&'#'))
      .count();

  if size > 6 {
      return 6;
  }

  return size;
}

pub fn open(output: &mut String, size: usize) {
  output.push_str(format!("<h{}>\n", size).as_str());
}

pub fn push_content(output: &mut String, text: String) {
  output.push_str(text.as_str());
}

pub fn close(output: &mut String, size: usize) {
  output.push_str(format!("</h{}>\n", size).as_str());
}
