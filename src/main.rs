pub mod config_schema;
pub mod custom_tags;
pub mod header;
use napi::Either;
use std::{
  error::Error,
  fs::{self, read_dir, File},
  io::{self, BufReader, Write},
  path::{Path, PathBuf},
};

use html_minifier::HTMLMinifier;
use regex::Regex;
use zip::ZipWriter;
use zip_extensions::ZipWriterExtensions;

use crate::config_schema::{ConfigSchema, Jsp};

fn main() {
  build_internal(Some(Either::A(ConfigSchema {
    out_dir: Some("./dist".to_string()),
    src: Some("./build".to_string()),
    jsp: None,
  })));
}

fn build_internal(arg: Option<Either<ConfigSchema, String>>) -> bool {
  let ConfigSchema {
    mut src,
    mut out_dir,
    mut jsp,
  } = if arg.is_some() {
    let config = arg.unwrap();
    match config {
      Either::A(config) => config,
      Either::B(file_path) => read_config_from_file(file_path).unwrap_or(ConfigSchema {
        src: Option::Some("./dist".to_owned()),
        out_dir: Option::Some("./snk".to_owned()),
        jsp: Option::None,
      }),
    }
  } else {
    read_config_from_file("./wdb.json".to_owned()).unwrap_or(ConfigSchema {
      src: Option::Some("./dist".to_owned()),
      out_dir: Option::Some("./snk".to_owned()),
      jsp: Option::None,
    })
  };
  src = Option::Some(src.unwrap_or("./dist".to_owned()));
  out_dir = Option::Some(out_dir.unwrap_or("./snk".to_owned()));
  let mut out_path = Path::new(&out_dir.unwrap()).to_path_buf();
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
    if ["js", "JS", "html", "HTML", "CSS", "css", "json", "JSON"].contains(&extension) {
      treat_asset_path(&file);
      treat_dyn_assets_path(&file);
    }
  }

  let mut custom_jsp_header: Vec<String> = vec![header::get().to_string()];
  let mut custom_jsp_content: Vec<String> = vec![];
  let mut custom_jsp_variables: Vec<String> = vec![];

  // Process JSP custom code
  let mut default_jsp_array: Vec<Jsp> = vec![];
  let jsp_custom_code = jsp.as_mut().unwrap_or(&mut default_jsp_array);
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
            eprintln!("Could not find the file: {}", &jsp_path.display());
            return false;
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
            eprintln!("Could not find the file: {}", &jsp_path.display());
            return false;
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
            eprintln!("Could not find the file: {}", &jsp_path.display());
            return false;
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

    // let jsp_path = Path::new(&ele.path).to_path_buf();
  }
  out_path.push("index.html");

  let mut path = Path::new(&out_path).to_path_buf();

  // Read html file
  let mut file = match fs::read_to_string(&path) {
    Ok(res) => res,
    Err(_) => {
      eprintln!(
        "Could not find index.html in the directory: {}",
        &path.display()
      );
      return false;
    }
  };

  // Set a new extension for HTML file to create it as JSP
  path.set_extension("jsp");
  // Uses regex to get the <head> tag from html file
  let re = Regex::new(r"<head>[.\s\S]*?<\/head>").unwrap();
  let caps = match re.captures(&file) {
    Some(res) => res,
    None => {
      eprintln!("Could not find <head> tag in the file: {}", &path.display());
      return false;
    }
  };
  let header = caps.get(0).unwrap().as_str();
  let header_str = header.to_string();

  // Get all custom tags to add at header
  let mut tags = custom_tags::get().to_vec();
  let binding_content = custom_jsp_content.join("\n");
  let binding_variables = format!("<script>{}</script>\n", custom_jsp_variables.join("\n"));
  tags.push(&binding_content);
  tags.push(&binding_variables);

  // Insert tags inside header
  let new_header = header_str.replace("<head>", &tags.join("\n"));
  file = file.replace(
    header_str.as_str(),
    ("<head>\n".to_owned() + &new_header).as_str(),
  );
  //Replace href attr of link tag
  let regex = Regex::new(r#"([\w\S]*)\=(\"|')(\.?\/+[\w\s\#\/\-\.]+)(\"|')"#).unwrap();
  let substitution = "$1=\"$${BASE_FOLDER}$3\"";

  // result will be a String with the substituted value
  let result = regex.replace_all(&file, substitution);
  file = result.to_string();
  file.insert_str(0, &custom_jsp_header.join("\n"));

  // Minify HTML
  let mut html_minifier = HTMLMinifier::new();
  match html_minifier.digest(&file) {
    Ok(_) => {
      println!("Minified HTML");
    }
    Err(_) => {
      eprintln!("Could not minify the HTML");
      return false;
    }
  };

  // Write minified HTML into the JSP file
  match fs::write(&path, html_minifier.get_html()) {
    Ok(_) => {
      println!("Minified HTML written into the JSP file");
    }
    Err(_) => {
      eprintln!("Could not write minified HTML into JSP file");
      return false;
    }
  }
  // Create zip file
  let mut zip_file_path = match out_path.clone().parent() {
    Some(res) => res.to_path_buf(),
    None => {
      eprintln!("Could not create zip file");
      return false;
    }
  };
  zip_file_path.set_extension("zip");
  let zip_file = File::create(&zip_file_path).unwrap();
  let mut zip = ZipWriter::new(zip_file);
  out_path = out_path.parent().unwrap().to_path_buf();
  let result = zip.create_from_directory(&out_path);

  // Check if zip was created
  if result.is_err() {
    eprintln!("Could not zip the directory:{}", &out_path.display());
    return false;
  }

  println!("Created zip file: {:?}", &zip_file_path);

  return true;
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
  // Open the file in read-only mode with buffer.
  let file = File::open(path)?;
  let reader = BufReader::new(file);

  // Read the JSON contents of the file as an instance of `User`.
  let u = serde_json::from_reader(reader)?;

  // Return the `User`.
  Ok(u)
}

fn recurse(path: impl AsRef<Path>) -> Vec<PathBuf> {
  let Ok(entries) = read_dir(path) else { return vec![] };
  entries
    .flatten()
    .flat_map(|entry| {
      let Ok(meta) = entry.metadata() else { return vec![] };
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
      let substitution = "(window.resolveAsset($1))";
      let result = regex.replace_all(&content, substitution);
      file.write_all(result.as_bytes()).unwrap();
    } else if extension == "html" {
      let cont = content.clone();
      let mut matchs = regex.captures_iter(&cont);

      loop {
        let mat = &mut matchs.next();
        if mat.is_none() {
          file.write_all(content.as_bytes()).unwrap();
          break;
        }
        let mat_some = mat.as_ref().unwrap();
        let value = mat_some.get(1).unwrap().as_str();
        let new_value = format!(
          "${{BASE_FOLDER}}{}",
          mat_some.get(1).unwrap().as_str().replace("\"", "")
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
  let regex =
    Regex::new(r#"(?mi)\s*('[^']+\.(js|css)'|"[^"]+\.(js|css)"|`[^`]+\.(js|css)`)\s*"#).unwrap();

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
      let cont = content.clone();
      let mut matchs = regex.captures_iter(&cont);

      loop {
        let mat = &mut matchs.next();
        if mat.is_none() {
          file.write_all(content.as_bytes()).unwrap();
          break;
        }

        let mat_some = mat.as_ref().unwrap();
        let value = mat_some.get(1).unwrap().as_str();

        if value.starts_with("\"https") || value.starts_with("\"http") {
          continue;
        }

        let new_value = format!(
          "(window.resolveAsset({}))",
          mat_some.get(1).unwrap().as_str()
        );
        content = content.replace(value, &new_value);
        file.write_all(content.as_bytes()).unwrap();
        content = "".to_owned(); // Esvazia para a proxima interação
      }
    } else {
      file.write_all(content.as_bytes()).unwrap();
    }
  }
  return true;
}
