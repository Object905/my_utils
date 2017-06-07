use tools;
use settings;


pub fn google_selected() {
    let text = tools::get_current_selected_text();

    if tools::is_url(&text) {
        tools::open_browser(&text);
    } else {
        let search_url = settings::GOOGLE_URL.clone() + &text;
        tools::open_browser(&search_url);
    }
}

pub fn google_translate_selected() {
    let text = tools::get_current_selected_text();
    let g_translator_url = settings::GOOGLE_TRANSLATOR_URL.clone() + &text;
    tools::open_browser(&g_translator_url);
}

pub fn translate_notify_selected() {
    let text = tools::get_current_selected_text().to_lowercase();
    let translated = tools::translate(&text);
    tools::show_desktop_notification(&translated, &text);
}

pub fn wiktionary_selected() {
    let text = tools::get_current_selected_text().replace(" ", "_");
    let wiktionary_word_url = settings::WIKTIONARY_URL.clone() + &text;
    tools::open_browser(&wiktionary_word_url);
}
