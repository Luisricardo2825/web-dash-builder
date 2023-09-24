pub fn get() -> [&'static str;3] {
  return [
    "<snk:load></snk:load>",
    "  <script type=\"text/javascript\" id=\"sankhyaVariable\">var Params = {};
        localStorage.setItem(\"base_folder\", \"${BASE_FOLDER}\");window.baseFolder=\"${BASE_FOLDER}\";</script>",
        "<script type=\"text/javascript\">window.resolveAsset = function (url) { url = String(url); if (url[0] == \".\") { url = url.slice(1, url.length); } if (url[0] == \"/\") { url = url.slice(1, url.length); } const base_folder = window.localStorage.getItem(\"base_folder\"); if (base_folder) { const finalUrl = base_folder + url; return finalUrl; } else { return url; } };</script>"
  ];
}
