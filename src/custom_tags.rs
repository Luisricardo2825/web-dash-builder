use crate::builder::html;

pub fn get() -> String {
  let script = html! {
  <snk:load></snk:load>
  <script type="text/javascript" id="sankhyaVariable">
      var Params = {};
      var base_path = r"${BASE_FOLDER}\/".replace("\\", "");
      localStorage.setItem("base_folder", base_path);
      window.baseFolder = base_path;
      window.resolveAsset = function (url) {
        url = String(url);
        if (url[0] == ".") {
            url = url.slice(1, url.length);
        }
        if (url[0] == "/") {
            url = url.slice(1, url.length);
        }
        const base_folder = window.localStorage.getItem("base_folder");
        const isJsFile = url.endsWith(".js");
        if (!isJsFile && base_folder){
          const finalUrl = "./"+base_folder + url;
          return finalUrl;
        }
        if (base_folder) {
            const finalUrl = "./mge/" + base_folder + url;
            return finalUrl;
        } else {
            return url;
        }
      };
      window.dbDialect = "<%=EntityFacadeFactory.getDWFFacade().getJdbcWrapper().getDataSource().getConnection().getMetaData().getDatabaseProductName()%>"
  </script>
  };

  // result will be a String with the substituted value
  script
}
