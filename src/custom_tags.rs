pub fn get() -> [&'static str; 4] {
  let first_script = r#"<script type="text/javascript" id="sankhyaVariable">var Params = {};var base_path = `${BASE_FOLDER}\/`.replace("\\","");localStorage.setItem("base_folder", base_path);window.baseFolder=base_path;</script>"#;
  let second_script = r#"<script type="text/javascript">window.resolveAsset = function (url) { url = String(url); if (url[0] == ".") { url = url.slice(1, url.length); } if (url[0] == "/") { url = url.slice(1, url.length); } const base_folder = window.localStorage.getItem("base_folder"); if (base_folder) { const finalUrl = "./mge/"+base_folder + url; return finalUrl; } else { return url; } };</script>"#;
  let third_script = r#"<script>window.dbDialect=eval("<%=EntityFacadeFactory.getDWFFacade().getJdbcWrapper().getDataSource().getConnection().getMetaData().getDatabaseProductName()%>")</script>"#;
  return ["<snk:load></snk:load>", first_script, second_script,third_script];
}
