use std::{
  env::current_dir,
  error::Error,
  fs::{self, read_dir, File},
  io::{self, BufReader, Write},
  path::{Path, PathBuf},
};

use minify_html::{minify, Cfg};
use napi::Either;
use regex::Regex;
use zip::ZipWriter;
use zip_extensions::ZipWriterExtensions;

use crate::{
  config_schema::{ConfigSchema, Jsp},
  custom_tags, header,
};

pub fn build(arg: Option<Either<ConfigSchema, String>>) -> bool {
  return build_internal(arg);
}
fn build_internal(arg: Option<Either<ConfigSchema, String>>) -> bool {
  let ConfigSchema {
    mut src,
    mut out_dir,
    jsp,
  } = if arg.is_some() {
    let config = arg.unwrap();
    match config {
      Either::A(config) => config,
      Either::B(file_path) => read_config_from_file(file_path).unwrap_or(ConfigSchema {
        src: None,
        out_dir: None,
        jsp: Option::None,
      }),
    }
  } else {
    read_config_from_file("wdb.json".to_owned()).unwrap_or(ConfigSchema {
      src: Option::Some("./dist".to_owned()),
      out_dir: Option::Some("./SankhyaBuild".to_owned()),
      jsp: Option::None,
    })
  };

  // let entry_file = entry_file.as_ref();
  src = Option::Some(src.unwrap_or("./dist".to_owned()));
  out_dir = Option::Some(out_dir.unwrap_or("./SankhyaBuild".to_owned()));
  let out_path = Path::new(&out_dir.unwrap()).to_path_buf();
  let src_path = Path::new(&src.unwrap()).to_path_buf();
  // Copy the whole build directory to dist
  match copy_dir(&src_path, &out_path) {
    Err(error) => {
      if !src_path.exists() {
        eprintln!(
          "Could not find the build directory: {}",
          &src_path.display()
        );
        return false;
      }
      if !out_path.exists() {
        eprintln!(
          "Could not find the output directory: {}",
          &out_path.display()
        );
        return false;
      }
      eprintln!("Build failed: {}", error);
      return false;
    }
    Ok(_) => {}
  };

  // Treat JS files
  let files = recurse(&out_path);

  for file in files {
    // File extension
    let extension = file.extension().unwrap_or_default().to_str().unwrap();
    let ext = extension.to_lowercase();
    if ["js", "html", "jsp", "css", "json"].contains(&ext.as_str()) {
      treat_asset_path(&file);

      if ["html", "jsp"].contains(&ext.as_str()) {
        treat_html_esmodule(&file);

        treat_wasm_asset_path(&file);
      }

      if ext == "js" {
        treat_dyn_assets_path(&file);

        treat_wasm_js_asset_path(&file);
      }
    }
  }

  // Transform all files to .jsp
  let files = recurse(&out_path);
  let default_jsp_array: Vec<Jsp> = vec![];
  let jsp_custom_code = jsp.unwrap_or(default_jsp_array);
  for file in files {
    let extension = file.extension().unwrap_or_default().to_str().unwrap();
    let ext = extension.to_lowercase();
    if ext == "html" {
      let mut jsp_file = file.clone();
      jsp_file.set_extension("jsp");
      fs::rename(&file, jsp_file).unwrap();
      let file = file.to_str().unwrap();
      parse_jsp(&jsp_custom_code, file, &out_path);
    }
  }

  // Create zip file
  let mut zip_file_path = out_path.clone();
  zip_file_path.set_extension("zip");

  let zip_file = File::create(&zip_file_path).unwrap();
  let zip = ZipWriter::new(zip_file);
  let result = zip.create_from_directory(&out_path);

  // Check if zip was created
  if result.is_err() {
    eprintln!("Could not zip the directory:{}", &out_path.display());
    return false;
  }

  // println!("Created zip file: {:?}", &zip_file_path);

  return true;
}

fn parse_jsp(jsp: &Vec<Jsp>, file: &str, out_path: &PathBuf) {
  let default_headers = minify_code(header::get());
  let mut custom_jsp_header: Vec<String> = vec![default_headers];
  let mut custom_jsp_content: Vec<String> = vec![];
  let mut custom_jsp_variables: Vec<String> = vec![];
  let mut out_path = out_path.clone();
  // Process JSP custom code
  process_custom_jsp(
    jsp,
    &mut custom_jsp_header,
    &mut custom_jsp_content,
    &mut custom_jsp_variables,
  );

  // Get entryFile as file
  let mut entry_file_path = Path::new(file).to_path_buf();
  entry_file_path.set_extension("jsp");
  let entry_file = entry_file_path.file_name().unwrap().to_str().unwrap();
  out_path.push(&entry_file);

  let path = Path::new(&out_path).to_path_buf();

  let jsp_minified = process_jsp_file(
    &path,
    entry_file.to_string(),
    custom_jsp_content,
    custom_jsp_variables,
    custom_jsp_header,
  );

  // Write minified HTML into the JSP file
  match fs::write(&path, jsp_minified) {
    Ok(_) => {
      // println!("Minified HTML written into the JSP file");
    }
    Err(_) => {
      panic!(
        "{}",
        format!(
          "Could not write minified HTML into JSP file: {}",
          &path.to_str().unwrap()
        )
      );
    }
  }
}
fn process_jsp_file(
  path: &PathBuf,
  entry_file: String,
  custom_jsp_content: Vec<String>,
  custom_jsp_variables: Vec<String>,
  custom_jsp_header: Vec<String>,
) -> String {
  // Read html file
  let mut html_content = match fs::read_to_string(&path) {
    Ok(res) => res,
    Err(_) => {
      panic!(
        "Could not find {} in the directory: {}",
        &entry_file,
        &path.display()
      );
    }
  };

  // Uses regex to get the <head> tag from html file
  let re = Regex::new(r"<head>[.\s\S]*?</head>").unwrap();
  let caps = match re.captures(&html_content) {
    Some(res) => res,
    None => {
      panic!(
        "Could not find <head> tag in the file: {} {html_content}",
        &path.display()
      );
    }
  };
  let header = caps.get(0).unwrap().as_str();
  let header_str = header.to_string();

  // Get all custom tags to add at header
  let binding = custom_tags::get();
  let mut tags = vec![binding.as_str()];
  let binding_content = custom_jsp_content.join("\n");
  let binding_content = minify_code(binding_content);
  let binding_variables = format!("<script>{}</script>\n", custom_jsp_variables.join("\n"));
  tags.push(&binding_content);
  tags.push(&binding_variables);

  // Insert tags inside header
  let new_header = header_str.replace("<head>", &tags.join("\n"));
  html_content = html_content.replace(
    header_str.as_str(),
    ("<head>\n".to_owned() + &new_header).as_str(),
  );
  //Replace href attr of link tag
  html_content = replace_href(custom_jsp_header, html_content);

  // Minify HTML
  let html_minified = minify_code(html_content);
  return html_minified;
}

fn replace_href(custom_jsp_header: Vec<String>, html_content: String) -> String {
  let regex = Regex::new(r#"([\w\S]*)\=(\"|')(\.?\/+[\w\s\#\/\-\.]+)(\"|')"#).unwrap();
  let substitution = "$1=\"$${BASE_FOLDER}$3\"";
  let mut html_content = html_content;
  // result will be a String with the substituted value
  let result = regex.replace_all(&html_content, substitution);
  html_content = result.to_string();
  html_content.insert_str(0, &custom_jsp_header.join("\n"));
  html_content
}

fn process_custom_jsp(
  jsp: &Vec<Jsp>,
  custom_jsp_header: &mut Vec<String>,
  custom_jsp_content: &mut Vec<String>,
  custom_jsp_variables: &mut Vec<String>,
) {
  let jsp_custom_code = jsp;
  for ele in jsp_custom_code {
    let path = &ele.path;
    let code = &ele.code;
    let type_field = &ele.type_field;

    if type_field.to_uppercase() == "BODY" {
      if path.is_some() {
        let path = ele.path.as_ref().unwrap();
        let jsp_path = Path::new(path).to_path_buf();
        let content = match fs::read_to_string(&jsp_path) {
          Ok(res) => res,
          Err(_) => {
            panic!("Could not find the file: {}", &jsp_path.display());
          }
        };
        custom_jsp_content.push(content);
      }
      if code.is_some() {
        let code = ele.code.as_ref().unwrap().to_owned();
        custom_jsp_content.push(code);
      }
    }
    if type_field.to_uppercase() == "HEADER" {
      if path.is_some() {
        let path = ele.path.as_ref().unwrap();
        let jsp_path = Path::new(path).to_path_buf();
        let content = match fs::read_to_string(&jsp_path) {
          Ok(res) => res,
          Err(_) => {
            panic!("Could not find the file: {}", &jsp_path.display());
          }
        };
        custom_jsp_header.push(content);
      }
      if code.is_some() {
        let code = ele.code.as_ref().unwrap().to_owned();
        custom_jsp_header.push(code);
      }
    }
    if type_field.to_uppercase() == "VARIABLE" {
      if path.is_some() {
        let path = ele.path.as_ref().unwrap();
        let jsp_path = Path::new(path).to_path_buf();
        let code = match fs::read_to_string(&jsp_path) {
          Ok(res) => res,
          Err(_) => {
            panic!("Could not find the file: {}", &jsp_path.display());
          }
        };
        let var_name = ele.var_name.as_ref().unwrap();
        custom_jsp_variables.push(format!("var {}={};", var_name, code));
      }
      if code.is_some() {
        let code = ele.code.as_ref().unwrap().to_owned();
        let var_name = ele.var_name.as_ref().unwrap();
        custom_jsp_variables.push(format!("var {}={};", var_name, code));
      }
    }
  }
}

fn copy_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
  let dir_exist = dst.as_ref().exists();
  if dir_exist {
    fs::remove_dir_all(&dst).unwrap();
  }
  fs::create_dir_all(&dst)?;
  for entry in fs::read_dir(src)? {
    let entry = entry?;
    let ty = entry.file_type()?;
    if ty.is_dir() {
      copy_dir(entry.path(), dst.as_ref().join(entry.file_name()))?;
    } else {
      fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
    }
  }
  Ok(())
}

fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<ConfigSchema, Box<dyn Error>> {
  let current_dir = current_dir();
  let mut current_dir = current_dir.expect("Could not get current dir");

  current_dir.push(path.as_ref());
  // Open the file in read-only mode with buffer.
  let file = File::open(current_dir)?;
  let reader = BufReader::new(file);

  // Read the JSON contents of the file as an instance of `User`.
  let u = serde_json::from_reader(reader)?;

  // Return the `User`.
  Ok(u)
}

fn recurse(path: impl AsRef<Path>) -> Vec<PathBuf> {
  let Ok(entries) = read_dir(path) else {
    return vec![];
  };
  entries
    .flatten()
    .flat_map(|entry| {
      let Ok(meta) = entry.metadata() else {
        return vec![];
      };
      if meta.is_dir() {
        return recurse(entry.path());
      }
      if meta.is_file() {
        return vec![entry.path()];
      }
      vec![]
    })
    .collect()
}

fn treat_asset_path<P: AsRef<Path>>(path: P) -> bool {
  let regex = Regex::new(r#"(?i)\s*('\.?\/[^']+\.(BMP|JPG|JPEG|GIF|PNG|WEBP|SVG)'|"\.?\/[^"]+\.(BMP|JPG|JPEG|GIF|PNG|WEBP|SVG)"|`\.?\/[^`]+\.(BMP|JPG|JPEG|GIF|PNG|WEBP|SVG)`)\s*"#).unwrap();
  let path_ = path.as_ref();
  let extension = path_.extension();
  let mut content = match fs::read_to_string(&path_) {
    Ok(res) => res,
    Err(_) => {
      eprintln!("Could not find the file: {:?}", &path_);
      return false;
    }
  };

  if extension.is_some() {
    let extension = extension.unwrap().to_str().unwrap();
    let mut file = File::create(&path_).unwrap();
    if extension == "js" {
      let substitution = "(window.resolveAssetFullPath($1))";
      let result = regex.replace_all(&content, substitution);
      file.write_all(result.as_bytes()).unwrap();
    } else if ["html", "jsp"].contains(&extension) {
      let cont = content.clone();
      let total_matchs = regex.captures_iter(&cont).count();
      let matchs = regex.captures_iter(&cont);
      if total_matchs <= 0 {
        file.write_all(content.as_bytes()).unwrap();
        return true;
      }
      for mat in matchs {
        let value = mat.get(1).unwrap().as_str();
        let new_value = format!(
          "${{BASE_FOLDER}}{}",
          mat.get(1).unwrap().as_str().replace("\"", "")
        );
        content = content.replace(value, &new_value);
        file.write_all(content.as_bytes()).unwrap();
      }
    } else {
      file.write_all(content.as_bytes()).unwrap();
    }
  }
  return true;
}

fn treat_dyn_assets_path<P: AsRef<Path>>(path: P) -> bool {
  let regex = Regex::new(
      r#"('[^https][^\.\/][^']+\.(js|css)'|"[^https][^\.\/][^"]+\.(js|css)"|`[^https][^\.\/][^`]+\.(js|css)`)\s*"#,
    )
    .unwrap();
  let path_ = path.as_ref();
  let extension = path_.extension();
  let content = match fs::read_to_string(&path_) {
    Ok(res) => res,
    Err(_) => {
      eprintln!("Could not find the file: {:?}", &path_);
      return false;
    }
  };

  if extension.is_some() {
    let extension = extension.unwrap().to_str().unwrap();
    let mut file = File::create(&path_).unwrap();
    if extension == "js" {
      let substitution = "(window.resolveAsset($1))";
      let result = regex.replace_all(&content, substitution);
      file.write_all(result.as_bytes()).unwrap();
    }
  }
  return true;
}

fn treat_html_esmodule<P: AsRef<Path>>(path: P) -> bool {
  let regex =
    Regex::new(r#"(?m)(?m)(<script[^>]*>)([\s\S]*?)(import\s+[^'"]*['"])(\/[^'"]+\.js)(['"];?)"#)
      .unwrap();
  let path_ = path.as_ref();
  let extension = path_.extension();
  let content = match fs::read_to_string(&path_) {
    Ok(res) => res,
    Err(_) => {
      eprintln!("Could not find the file: {:?}", &path_);
      return false;
    }
  };

  if extension.is_some() {
    let extension = extension.unwrap().to_str().unwrap();
    let mut file = File::create(&path_).unwrap();
    if ["html", "jsp"].contains(&extension) {
      let substitution = r"$1$2$3./$${BASE_FOLDER}$4$5";
      let result = regex.replace_all(&content, substitution);
      file.write_all(result.as_bytes()).unwrap();
    } else {
      file.write_all(content.as_bytes()).unwrap();
    }
  }
  return true;
}

fn treat_wasm_asset_path<P: AsRef<Path>>(path: P) -> bool {
  let regex =
    Regex::new(r#"(?s)(<script[^>]*>.*?)(['"`][^'"`]+\.wasm['"`])(.*?<\/script>)"#).unwrap();
  let path_ = path.as_ref();
  let extension = path_.extension();
  let content = match fs::read_to_string(&path_) {
    Ok(res) => res,
    Err(_) => {
      eprintln!("Could not find the file: {:?}", &path_);
      return false;
    }
  };

  if extension.is_some() {
    let extension = extension.unwrap().to_str().unwrap();
    let mut file = File::create(&path_).unwrap();
    if ["html", "jsp"].contains(&extension) {
      let result = regex.replace_all(&content, |caps: &regex::Captures| {
        format!(
          "{}{}{}",
          &caps[1],
          format!("window.getAsURL({})", &caps[2]),
          &caps[3]
        )
      });
      file.write_all(result.as_bytes()).unwrap();
    } else {
      file.write_all(content.as_bytes()).unwrap();
    }
  }
  return true;
}

fn treat_wasm_js_asset_path<P: AsRef<Path>>(path: P) -> bool {
  let regex = Regex::new(r#"(?s)[^URL(](['"`][^'"`]+\.wasm['"`])"#).unwrap();
  let path_ = path.as_ref();
  let extension = path_.extension();
  let content = match fs::read_to_string(&path_) {
    Ok(res) => res,
    Err(_) => {
      eprintln!("Could not find the file: {:?}", &path_);
      return false;
    }
  };

  if extension.is_some() {
    let extension = extension.unwrap().to_str().unwrap();
    let mut file = File::create(&path_).unwrap();
    if extension == "js" {
      let substitution = "window.getAsURL($0)";
      let result = regex.replace_all(&content, substitution);
      file.write_all(result.as_bytes()).unwrap();
    }
  }
  return true;
}

pub fn minify_code<S: AsRef<str>>(code: S) -> String {
  let code = code.as_ref();
  let code = remove_java_comments(code);
  let code = code.as_bytes();
  let mut cfg = Cfg::new();
  cfg.minify_js = true;
  cfg.minify_css = true;

  cfg.preserve_brace_template_syntax = false;
  cfg.preserve_chevron_percent_template_syntax = false;

  cfg.keep_html_and_head_opening_tags = true;
  cfg.keep_spaces_between_attributes = true;
  cfg.keep_closing_tags = true;
  cfg.do_not_minify_doctype = true;

  let minified = minify(code, &cfg);
  let minified = String::from_utf8(minified).unwrap();
  println!("{}", minified);
  return minified;
}

pub fn remove_java_comments(code: &str) -> String {
  let mut result = String::new();
  let mut in_string = false;
  let mut in_char = false;
  let mut in_single_line_comment = false;
  let mut in_multi_line_comment = false;
  let mut prev_char = '\0';
  let mut chars = code.chars().peekable();

  while let Some(c) = chars.next() {
    if in_single_line_comment {
      if c == '\n' {
        in_single_line_comment = false;
        result.push(c);
      }
      continue;
    }

    if in_multi_line_comment {
      if prev_char == '*' && c == '/' {
        in_multi_line_comment = false;
      }
      prev_char = c;
      continue;
    }

    if !in_string && !in_char {
      if prev_char == '/' && c == '/' {
        in_single_line_comment = true;
        result.pop(); // Remove o '/'
        continue;
      } else if prev_char == '/' && c == '*' {
        in_multi_line_comment = true;
        result.pop(); // Remove o '/'
        continue;
      }
    }

    if c == '"' && prev_char != '\\' {
      in_string = !in_string;
    } else if c == '\'' && prev_char != '\\' {
      in_char = !in_char;
    }

    result.push(c);
    prev_char = c;
  }

  result
}
