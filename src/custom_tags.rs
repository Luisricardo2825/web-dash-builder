pub fn get() -> [&'static str; 2] {
    return [
        "<snk:load></snk:load>",
        "  <script type=\"text/javascript\" id=\"sankhyaVariable\">var Params = {};
        localStorage.setItem(\"base_folder\", \"${BASE_FOLDER}\");</script>",
    ];
}
