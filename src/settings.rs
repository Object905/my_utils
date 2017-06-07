pub static DISPLAY_LANG: &str = "en";
pub static SOURCE_LANG: &str = "auto";
pub static TARGET_LANG: &str = "ru";
pub static TOP_LEVEL_DOMAIN: &str = "ru";

pub static BROWSER_COMMAND: &str = "/usr/bin/chromium";

static API_KEY: &'static str = include_str!("../yandex_api_key");

lazy_static! {
    static ref DIRECTION_LANG: String = {
        if SOURCE_LANG != "auto" {
            format!("{src}-{target}", src = SOURCE_LANG, target = TARGET_LANG)
        } else {
            String::from(TARGET_LANG)
        }
    };

    pub static ref GOOGLE_TRANSLATOR_URL: String = {
        format!("https://translate.google.{TLD}/?hl={display_lang}#{src}/{target}/",
        TLD = TOP_LEVEL_DOMAIN,
        display_lang = DISPLAY_LANG,
        src = SOURCE_LANG,
        target = TARGET_LANG)
    };

    pub static ref GOOGLE_URL: String = {
        format!("https://www.google.{TLD}/search?q=", TLD = TOP_LEVEL_DOMAIN)
    };

    pub static ref WIKTIONARY_URL: String = {
        format!("https://{display_lang}.wiktionary.org/wiki/", display_lang = DISPLAY_LANG)
    };

    pub static ref YANDEX_URL: String = {
        format!("https://translate.yandex.net/api/v1.5/tr.json/translate?\
            key={api_key}&lang={lang}&text=", api_key = API_KEY, lang = *DIRECTION_LANG)
    };
}
