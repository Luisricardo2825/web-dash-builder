#[cfg(test)]
mod tests {

  use std::path::Path;

  use napi::Either;
  use web_dash_builder::{build, config_schema::ConfigSchema};

  #[test]
  fn test_build() {
    let out_dir = "./out/SankhyaBuild";

    let build_result = build(Some(Either::A(ConfigSchema {
      out_dir: Some(out_dir.to_owned()),
      src: Some("./pkgs/dist".to_string()),
      base_folder: Some("/addon-template/html5/build".to_owned()),
      default_headers: Some(false),
      ..Default::default()
    })));

    check_out_dir(out_dir);

    assert_eq!(build_result, true);
  }

  #[test]
  fn test_wasm_build() {
    let out_dir = "./out/wasmSankhya";

    let build_result_2 = build(Some(Either::A(ConfigSchema {
      out_dir: Some(out_dir.to_owned()),
      src: Some("./pkgs/wasm".to_string()),
      ..Default::default()
    })));

    check_out_dir(out_dir);

    assert_eq!(build_result_2, true)
  }

  #[test]
  fn test_wasm_build_with_config_file() {
    let out_dir = "./out/snk";

    let build_result_2 = build(None);

    check_out_dir(out_dir);

    let matches = get_jsp_imports(&Path::new("./out/snk/index.jsp"));
    let matches = matches.join("\n");
    println!("{}", matches);

    assert_eq!(build_result_2, true)
  }

  fn get_jsp_imports(path: &Path) -> Vec<String> {
    let regex =
      regex::Regex::new(r"<%@\s*(page|taglib)\s*(import=)?(['][\w.\d*]*[']|[^<%@%>]*)?[^<%@%>]*%>");

    // Get all matchs
    let content = std::fs::read_to_string(path).expect("Could not read file");
    let mut matches = Vec::new();
    for cap in regex.unwrap().captures_iter(&content) {
      matches.push(cap.get(0).unwrap().as_str().to_string());
    }

    return matches;
  }

  fn check_out_dir(out_dir: &str) {
    // Find jsp file
    let jsp_file = std::fs::read_dir(out_dir).expect("Could not read directory");
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
  }
}
