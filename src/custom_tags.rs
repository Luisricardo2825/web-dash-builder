pub fn get_necessary() -> String {
  let script = include_bytes!("html/customTags.jsp");
  let script = String::from_utf8(script.to_vec()).unwrap();

  // result will be a String with the substituted value
  script
}

pub fn get_default() -> String {
  let custom_tags = get_necessary();
  let script = include_bytes!("html/sankhyaUtil.jsp");
  let script = String::from_utf8(script.to_vec()).unwrap();
  let script = custom_tags + &script;
  // result will be a String with the substituted value

  script
}
