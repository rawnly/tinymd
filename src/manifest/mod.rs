
pub fn get_title() -> String {

  let mut title : String = String::from( env!("CARGO_PKG_NAME") );

  title.push_str(" (v");
  title.push_str(env!("CARGO_PKG_VERSION"));
  title.push_str("), ");
  title.push_str(env!("CARGO_PKG_DESCRIPTION"));

  return title;
}
