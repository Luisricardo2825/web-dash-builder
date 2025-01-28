pub fn get() -> String {
  let script = include_bytes!("html/customTags.html");
  let script = String::from_utf8(script.to_vec()).unwrap();

  // result will be a String with the substituted value
  script
}

