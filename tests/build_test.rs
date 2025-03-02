#[cfg(test)]
mod tests {

  use napi::Either;
  use web_dash_builder::{build, builder::minify_code, config_schema::ConfigSchema};

  #[test]
  fn test_build() {
    let build_result = build(
      Some(Either::A(ConfigSchema {
        out_dir: None,
        src: Some("./dist".to_string()),
        jsp: None,
      })),
      Some("main.html"),
    );
    assert_eq!(build_result, true)
  }

  #[test]
  fn test_minify() {
    // Find jsp file
    let jsp_file = std::fs::read_dir("SankhyaBuild").expect("Could not read directory");
    let mut jsp_file_path = String::new();
    for file in jsp_file {
      let file = file.expect("Could not read file");
      let file_name = file.file_name();
      let file_name = file_name.to_str().expect("Could not read file name");
      if file_name.ends_with(".jsp") {
        jsp_file_path = file
          .path()
          .to_str()
          .expect("Could not read file path")
          .to_string();
      }
    }

    if jsp_file_path.is_empty() {
      panic!("Could not find jsp file")
    }
    let content = std::fs::read_to_string(jsp_file_path).expect("Could not read file");
    let minified = minify_code(content);

    if minified.contains("\n") {
      panic!("Minification failed")
    }
  }
}
