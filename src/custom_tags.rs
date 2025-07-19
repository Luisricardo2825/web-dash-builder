pub fn get_necessary() -> String {
  let script = include_bytes!("html/customTags.jsp");
  String::from_utf8(script.to_vec()).unwrap()
}

pub fn get_default() -> String {
  let custom_tags = get_necessary();
  let script = include_bytes!("html/sankhyaUtil.jsp");
  let script = String::from_utf8(script.to_vec()).unwrap();
  custom_tags + &script
}
