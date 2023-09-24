use std::{
  fs::{self, read_dir, File},
  io::Write,
  path::{Path, PathBuf},
};

use regex::Regex;

fn main() {
  let path = Path::new("snk");

  let files = recurse(path);

  for file in files {
    treat_asset_path(file);
  }
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
      let substitution = "(window.baseFolder+$1)";
      let result = regex.replace_all(&content, substitution);
      file.write_all(result.as_bytes()).unwrap();
    }
    if extension == "html" {
      let cont = content.clone();
      let matchs = regex.captures_iter(&cont);
      for mat in matchs {
        let value = mat.get(1).unwrap().as_str();
        let new_value = format!("${{BASE_FOLDER}}{}", mat.get(1).unwrap().as_str().replace("\"", ""));
         content = content.replace(value, &new_value);
         file.write_all(content.as_bytes()).unwrap();
      }
    }
  }
  return true;
}
