use std::{
  env::current_dir,
  error::Error,
  fs::{self, read_dir, File},
  io::{self, BufReader, BufWriter, Read, Write},
  path::{Path, PathBuf},
};

use minify_html::{minify, Cfg};
use napi::Either;
use regex::Regex;
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipWriter};

use crate::{
  config_schema::{ConfigSchema, Jsp},
  custom_tags, header,
};

pub fn build(arg: Option<Either<ConfigSchema, String>>) -> bool {
  build_internal(arg)
}
fn build_internal(arg: Option<Either<ConfigSchema, String>>) -> bool {
  let ConfigSchema {
    mut src,
    mut out_dir,
    jsp,
    default_headers,
    base_folder,
  } = if arg.is_some() {
    let config = arg.unwrap();
    match config {
      Either::A(config) => config,
      Either::B(file_path) => read_config_from_file(file_path).unwrap_or(ConfigSchema {
        src: None,
        out_dir: None,
        jsp: None,
        default_headers: Option::Some(true),
        base_folder: None,
      }),
    }
  } else {
    read_config_from_file("wdb.json").unwrap_or(ConfigSchema {
      src: Option::Some("./dist".to_owned()),
      out_dir: Option::Some("./SankhyaBuild".to_owned()),
      jsp: None,
      default_headers: Option::Some(true),
      base_folder: None,
    })
  };

  // let entry_file = entry_file.as_ref();
  let default_headers = default_headers.unwrap_or(true);
  src = Option::Some(src.unwrap_or("./dist".to_owned()));
  out_dir = Option::Some(out_dir.unwrap_or("./SankhyaBuild".to_owned()));
  let out_path = Path::new(&out_dir.unwrap()).to_path_buf();
  let src_path = Path::new(&src.unwrap()).to_path_buf();
  // Copy the whole build directory to dist
  if let Err(error) = copy_dir(&src_path, &out_path) {
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
    eprintln!("Build failed: {error}");
    return false;
  };

  // Treat JS files
  let files = recurse(&out_path);

  for file in files {
    // File extension
    let extension = file.extension().unwrap_or_default().to_str().unwrap();
    let extension = extension.to_lowercase();
    let extension = extension.as_str();
    if ["js", "html", "jsp", "css", "json"].contains(&extension) {
      treat_asset_path(&file, base_folder.clone());

      if ["html", "jsp"].contains(&extension) {
        treat_html_esmodule(&file);

        treat_wasm_asset_path(&file);
      }

      if extension == "js" {
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
    let extension = extension.to_lowercase();
    let extension = extension.as_str();
    if ["html", "jsp"].contains(&extension) {
      let mut jsp_file = file.clone();
      jsp_file.set_extension("jsp");
      fs::rename(&file, jsp_file).unwrap();
      let file = file.to_str().unwrap();
      parse_jsp(
        &jsp_custom_code,
        file,
        &out_path,
        default_headers,
        base_folder.clone(),
      );
    }
  }

  // Create zip file
  let mut zip_file_path = out_path.clone();
  zip_file_path.set_extension("zip");

  let result = zip_dir(&out_path, &zip_file_path);

  // Check if zip was created
  if result.is_err() {
    eprintln!("Could not zip the directory:{}", &out_path.display());
    return false;
  }

  true
}

fn zip_dir(src_dir: &Path, zip_path: &Path) -> zip::result::ZipResult<()> {
  let zip_file = File::create(zip_path)?;
  let buf_writer = BufWriter::new(zip_file);
  let mut zip = ZipWriter::new(buf_writer);

  let options: zip::write::FileOptions<()> =
    FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

  for entry in WalkDir::new(src_dir) {
    let entry = entry.unwrap();
    let path = entry.path();
    let name = path.strip_prefix(src_dir).unwrap();

    if path.is_file() {
      zip.start_file(name.to_string_lossy(), options)?;
      let mut f = File::open(path)?;
      let mut buffer = Vec::new();
      f.read_to_end(&mut buffer)?;
      zip.write_all(&buffer)?;
    } else if path.is_dir() && !name.as_os_str().is_empty() {
      zip.add_directory(name.to_string_lossy(), options)?;
    }
  }

  zip.finish()?;
  Ok(())
}

fn parse_jsp(
  jsp: &Vec<Jsp>,
  file: &str,
  out_path: &Path,
  default_headers: bool,
  base_folder: Option<String>,
) {
  let headers = header::get();
  let mut custom_jsp_header: Vec<String> = vec![headers];
  let base_folder = base_folder.unwrap_or("".to_owned());
  // if !base_folder.is_empty() {

  //   let base_folder = format!(r#"<% String BASE_FOLDER = "{}"; %>"#, base_folder);
  //   custom_jsp_header.push(base_folder);
  // }
  let mut custom_jsp_content: Vec<String> = vec![];
  let mut custom_jsp_variables: Vec<String> = vec![];
  let mut out_path = out_path.to_path_buf();
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
  out_path.push(entry_file);

  let path = Path::new(&out_path).to_path_buf();

  let jsp_minified = process_jsp_file(
    &path,
    entry_file.to_string(),
    custom_jsp_content,
    custom_jsp_variables,
    custom_jsp_header,
    default_headers,
    base_folder,
  );

  // Write minified HTML into the JSP file
  match fs::write(&path, jsp_minified) {
    Ok(_) => {}
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
  default_headers: bool,
  base_folder: String,
) -> String {
  // Read html file
  let mut html_content = match fs::read_to_string(path) {
    Ok(res) => res,
    Err(_) => {
      panic!(
        "Could not find {} in the directory: {}",
        &entry_file,
        &path.display()
      );
    }
  };

  //Replace href/src attr of link tag
  html_content = replace_html_assets(html_content, &base_folder);
  html_content.insert_str(0, &custom_jsp_header.join("\n"));

  // Uses regex to get the <head> tag from html file
  let re = Regex::new(r"<head>[.\s\S]*?</head>").unwrap();
  let caps = re.captures(&html_content);

  if caps.is_some() {
    let caps = caps.unwrap();
    let header = caps.get(0).unwrap().as_str();
    let header_str = header.to_string();

    // Get all custom tags to add at header
    let custom_tags = if default_headers {
      custom_tags::get_default()
    } else {
      custom_tags::get_necessary()
    };
    let mut tags = vec![custom_tags.as_str()];
    let binding_content = custom_jsp_content.join("\n");
    let binding_variables = format!("<script>{}</script>\n", custom_jsp_variables.join("\n"));
    tags.push(&binding_content);
    tags.push(&binding_variables);

    // Insert tags inside header
    let new_header = header_str.replace("<head>", &tags.join("\n"));
    html_content = html_content.replace(
      header_str.as_str(),
      ("<head>\n".to_owned() + &new_header).as_str(),
    );
  }
  html_content = replace_base_folder_var(&html_content, &base_folder);

  // Minify HTML

  minify_code(html_content)
}

fn replace_html_assets(html_content: String, base_folder: &str) -> String {
  let regex = Regex::new(r#"([\w\S]*)\=(\"|')(\.?\/+[\w\s\#\/\-\.]+)(\"|')"#).unwrap();
  let mut substitution = "$1=\"$${BASE_FOLDER}$3\"";
  let subs = format!("$1=\"{base_folder}$3\"");

  // replace with base_folder
  let mut html_content = html_content;

  if !base_folder.is_empty() {
    substitution = subs.as_str();
  }
  // result will be a String with the substituted value
  let result = regex.replace_all(&html_content, substitution);
  html_content = result.to_string();
  html_content
}

fn replace_base_folder_var(html_content: &str, base_folder: &str) -> String {
  let regex = Regex::new(r#"(?m)<%=.?request\.getAttribute\("BASE_FOLDER"\).?%>"#).unwrap();

  if !base_folder.is_empty() {
    let result = regex.replace_all(html_content, base_folder);
    result.to_string()
  } else {
    html_content.to_owned()
  }
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
        custom_jsp_variables.push(format!("var {var_name}={code};"));
      }
      if code.is_some() {
        let code = ele.code.as_ref().unwrap().to_owned();
        let var_name = ele.var_name.as_ref().unwrap();
        custom_jsp_variables.push(format!("var {var_name}={code};"));
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

fn treat_asset_path<P: AsRef<Path>>(path: P, base_folder: Option<String>) -> bool {
  let regex = Regex::new(r#"(?i)\s*('\.?\/[^']+\.(BMP|JPG|JPEG|GIF|PNG|WEBP|SVG)'|"\.?\/[^"]+\.(BMP|JPG|JPEG|GIF|PNG|WEBP|SVG)"|`\.?\/[^`]+\.(BMP|JPG|JPEG|GIF|PNG|WEBP|SVG)`)\s*"#).unwrap();
  let path_ = path.as_ref();
  let extension = path_.extension();
  let mut content = match fs::read_to_string(path_) {
    Ok(res) => res,
    Err(_) => {
      eprintln!("Could not find the file: {:?}", &path_);
      return false;
    }
  };

  if extension.is_some() {
    let extension = extension.unwrap().to_str().unwrap().to_lowercase();
    let extension = extension.as_str();
    let mut file = File::create(path_).unwrap();
    if extension == "js" {
      let substitution = "(window.resolveAssetFullPath($1))";
      let result = regex.replace_all(&content, substitution);
      file.write_all(result.as_bytes()).unwrap();
    } else if ["html", "jsp"].contains(&extension) {
      let cont = content.clone();
      let total_matchs = regex.captures_iter(&cont).count();
      let matchs = regex.captures_iter(&cont);
      if total_matchs == 0 {
        file.write_all(content.as_bytes()).unwrap();
        return true;
      }
      for mat in matchs {
        let value = mat.get(1).unwrap().as_str();
        let mut new_value = format!(
          "${{BASE_FOLDER}}{}",
          mat.get(1).unwrap().as_str().replace("\"", "")
        );

        if base_folder.is_some() {
          let subs = format!(
            "{}{}",
            base_folder.clone().unwrap(),
            mat.get(1).unwrap().as_str().replace("\"", "")
          );
          new_value = subs;
        }
        content = content.replace(value, &new_value);
        file.write_all(content.as_bytes()).unwrap();
      }
    } else {
      file.write_all(content.as_bytes()).unwrap();
    }
  }
  true
}

fn treat_dyn_assets_path<P: AsRef<Path>>(path: P) -> bool {
  let regex = Regex::new(
      r#"('[^https][^\.\/][^']+\.(js|css)'|"[^https][^\.\/][^"]+\.(js|css)"|`[^https][^\.\/][^`]+\.(js|css)`)\s*"#,
    )
    .unwrap();
  let path_ = path.as_ref();
  let extension = path_.extension();
  let content = match fs::read_to_string(path_) {
    Ok(res) => res,
    Err(_) => {
      eprintln!("Could not find the file: {:?}", &path_);
      return false;
    }
  };

  if extension.is_some() {
    let extension = extension.unwrap().to_str().unwrap();
    let mut file = File::create(path_).unwrap();
    if extension == "js" {
      let substitution = "(window.resolveAsset($1))";
      let result = regex.replace_all(&content, substitution);
      file.write_all(result.as_bytes()).unwrap();
    }
  }
  true
}

fn treat_html_esmodule<P: AsRef<Path>>(path: P) -> bool {
  let regex =
    Regex::new(r#"(?m)(?m)(<script[^>]*>)([\s\S]*?)(import\s+[^'"]*['"])(\/[^'"]+\.js)(['"];?)"#)
      .unwrap();
  let path_ = path.as_ref();
  let extension = path_.extension();
  let content = match fs::read_to_string(path_) {
    Ok(res) => res,
    Err(_) => {
      eprintln!("Could not find the file: {:?}", &path_);
      return false;
    }
  };

  if extension.is_some() {
    let extension = extension.unwrap().to_str().unwrap();
    let extension = extension.to_lowercase();
    let extension = extension.as_str();
    let mut file = File::create(path_).unwrap();
    if ["html", "jsp"].contains(&extension) {
      let substitution = r"$1$2$3./$${BASE_FOLDER}$4$5";
      let result = regex.replace_all(&content, substitution);
      file.write_all(result.as_bytes()).unwrap();
    } else {
      file.write_all(content.as_bytes()).unwrap();
    }
  }
  true
}

fn treat_wasm_asset_path<P: AsRef<Path>>(path: P) -> bool {
  let regex =
    Regex::new(r#"(?s)(<script[^>]*>.*?)(['"`][^'"`]+\.wasm['"`])(.*?<\/script>)"#).unwrap();
  let path_ = path.as_ref();
  let extension = path_.extension();
  let content = match fs::read_to_string(path_) {
    Ok(res) => res,
    Err(_) => {
      eprintln!("Could not find the file: {:?}", &path_);
      return false;
    }
  };

  if extension.is_some() {
    let extension = extension.unwrap().to_str().unwrap();
    let mut file = File::create(path_).unwrap();
    if ["html", "jsp"].contains(&extension) {
      let result = regex.replace_all(&content, |caps: &regex::Captures| {
        format!(
          "{}{}{}",
          &caps[1],
          format_args!("window.getAsURL({})", &caps[2]),
          &caps[3]
        )
      });
      file.write_all(result.as_bytes()).unwrap();
    } else {
      file.write_all(content.as_bytes()).unwrap();
    }
  }
  true
}

fn treat_wasm_js_asset_path<P: AsRef<Path>>(path: P) -> bool {
  let regex = Regex::new(r#"(?s)[^URL(](['"`][^'"`]+\.wasm['"`])"#).unwrap();
  let path_ = path.as_ref();
  let extension = path_.extension();
  let content = match fs::read_to_string(path_) {
    Ok(res) => res,
    Err(_) => {
      eprintln!("Could not find the file: {:?}", &path_);
      return false;
    }
  };

  if extension.is_some() {
    let extension = extension.unwrap().to_str().unwrap();
    let mut file = File::create(path_).unwrap();
    if extension == "js" {
      let substitution = "window.getAsURL($0)";
      let result = regex.replace_all(&content, substitution);
      file.write_all(result.as_bytes()).unwrap();
    }
  }
  true
}

pub fn minify_code<S: AsRef<str>>(code: S) -> String {
  let code = code.as_ref();
  let code = remove_java_comments(code);
  let code = code.as_bytes();
  let mut cfg = Cfg::new();
  cfg.minify_js = true;
  cfg.minify_css = true;

  cfg.preserve_brace_template_syntax = true;
  cfg.preserve_chevron_percent_template_syntax = true;

  cfg.keep_html_and_head_opening_tags = true;
  cfg.allow_removing_spaces_between_attributes = true;
  cfg.keep_closing_tags = true;
  cfg.minify_doctype = true;

  let minified = minify(code, &cfg);

  String::from_utf8(minified).unwrap()
}

pub fn remove_java_comments(input: &str) -> String {
  let mut result = String::with_capacity(input.len());

  let mut chars = input.chars().peekable();
  let mut in_string = false;
  let mut string_delim = '\0';

  while let Some(c) = chars.next() {
    if in_string {
      // Fechamento de string
      result.push(c);
      if c == '\\' {
        // Escapa o próximo caractere
        if let Some(next) = chars.next() {
          result.push(next);
        }
      } else if c == string_delim {
        in_string = false;
      }
    } else {
      match c {
        '"' | '\'' => {
          in_string = true;
          string_delim = c;
          result.push(c);
        }
        '/' => match chars.peek() {
          Some('/') => {
            // Comentário de linha
            while let Some(nc) = chars.next() {
              if nc == '\n' {
                result.push('\n');
                break;
              }
            }
          }
          Some('*') => {
            // Comentário de bloco
            chars.next(); // Consome '*'
            while let Some(nc) = chars.next() {
              if nc == '*' {
                if let Some('/') = chars.peek() {
                  chars.next(); // Consome '/'
                  break;
                }
              }
            }
          }
          _ => {
            result.push(c);
          }
        },
        _ => result.push(c),
      }
    }
  }

  result
}
