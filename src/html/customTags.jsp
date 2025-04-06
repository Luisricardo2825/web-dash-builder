
<script type="module">
    var baseFolder = `<%= request.getAttribute("BASE_FOLDER") %>`;
    window.dbDialect = "<%= databaseProductName %>"
    // Check if window.baseFolder ends with "/"
    if (baseFolder.slice(-1) != "/") {
        baseFolder = baseFolder + "/";
    }
    window.baseFolder = baseFolder;
    function getAsURL(path) {
        return new URL(baseFolder + path, import.meta.url);
    };
    window.getAsURL = getAsURL;
</script>
<script type="module" id="sankhyaVariable">
    var Params = {};

    localStorage.setItem("base_folder", window.baseFolder);

    function resolveAsset(path) {
        const urlFinal = window.getAsURL(path);
        return urlFinal.pathname.slice(1, urlFinal.length);
    }
    window.resolveAsset = resolveAsset;

    function resolveAssetFullPath(path) {
        const urlFinal = window.getAsURL(path);
        return urlFinal.toString();
    };
    window.resolveAssetFullPath = resolveAssetFullPath;

</script>