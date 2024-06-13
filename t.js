window.resolveAsset = function (url) {
  url = String(url);
  const defaultUrl = String(url);
  if (url[0] == ".") {
    url = url.slice(1, url.length);
  }
  if (url[0] == "/") {
    url = url.slice(1, url.length);
  }
  const base_folder = window.localStorage.getItem("base_folder");
  const has_assets = url.toLocaleLowerCase().startsWith("assets");
  const finalUrl = window.origin + "/mge/" + base_folder +(has_assets?"":"assets/")+ url;
  return finalUrl;
};
