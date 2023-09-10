pub mod config_schema;
pub mod custom_tags;
pub mod header;
use napi::Either;
use napi_derive::napi;
use std::{
  error::Error,
  fs::{self, File},
  io::{self, BufReader},
  path::Path,
};

use html_minifier::HTMLMinifier;
use regex::Regex;
use zip::ZipWriter;
use zip_extensions::ZipWriterExtensions;

use crate::config_schema::{ConfigSchema, Jsp};

#[napi]
pub fn build(arg: Option<Either<ConfigSchema, String>>) -> bool {
  let result = build_internal(arg);

  match result {
    Ok(res) => res,
    Err(_) => {
      println!("Build failed");
      return false;
    }
  }
}

fn build_internal(arg: Option<Either<ConfigSchema, String>>) -> Result<bool, Box<dyn Error>> {
  let ConfigSchema {
    mut src,
    mut out_dir,
    mut jsp,
  } = if arg.is_some() {
    let config = arg.unwrap();
    match config {
      Either::A(config) => config,
      Either::B(file_path) => read_config_from_file(file_path).unwrap_or(ConfigSchema {
        src: Option::Some("./build".to_owned()),
        out_dir: Option::Some("./dist".to_owned()),
        jsp: Option::None,
      }),
    }
  } else {
    read_config_from_file("./wdb.json".to_owned()).unwrap_or(ConfigSchema {
      src: Option::Some("./build".to_owned()),
      out_dir: Option::Some("./dist".to_owned()),
      jsp: Option::None,
    })
  };
  src = Option::Some(src.unwrap_or("./build".to_owned()));
  out_dir = Option::Some(out_dir.unwrap_or("./dist".to_owned()));
  let mut out_path = Path::new(&out_dir.unwrap()).to_path_buf();
  let src_path = Path::new(&src.unwrap()).to_path_buf();
  // Copy the whole build directory to dist
  copy_dir(&src_path, &out_path).expect("Could not copy build directory");

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
        let content = fs::read_to_string(&jsp_path).unwrap();
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
        let content = fs::read_to_string(&jsp_path).unwrap();
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
        let code = fs::read_to_string(&jsp_path).unwrap();
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
  let mut file = fs::read_to_string(&path).unwrap();
  file.insert_str(0, &custom_jsp_header.join("\n"));

  // Set a new extension for HTML file to create it as JSP
  path.set_extension("jsp");

  // Uses regex to get the <head> tag from html file
  let re = Regex::new(r"<head>[.\s\S]*?</head>").unwrap();
  let caps = re.captures(&file).expect("Tag <head> not found");
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
  let regex = Regex::new(r#"(src|href)\=(\"|')(\.?\/+[\w\s\#\/\-\.]+)?(\"|')"#).unwrap();
  let substitution = "$1=\"$${BASE_FOLDER}$3\"";

  // result will be a String with the substituted value
  let result = regex.replace_all(&file, substitution);
  file = result.to_string();
  // Minify HTML
  let mut html_minifier = HTMLMinifier::new();
  html_minifier.digest(&file).unwrap();

  // Write minified HTML into the JSP file
  fs::write(&path, html_minifier.get_html()).unwrap();
  println!("{}", path.display());

  // Create zip file
  let mut zip_file_path = out_path.clone().parent().unwrap().to_path_buf();
  zip_file_path.set_extension("zip");
  let zip_file = File::create(zip_file_path).unwrap();
  let mut zip = ZipWriter::new(zip_file);
  out_path = out_path.parent().unwrap().to_path_buf();
  let result = zip.create_from_directory(&out_path);

  // Check if zip was created
  if result.is_err() {
    println!("Could not create zip file");
    return Err("Could not create zip file".into());
  }

  println!("Created zip file: dist.zip");

  return Ok(true);
}

fn copy_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
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
