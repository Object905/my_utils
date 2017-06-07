use std::process::{Command, Stdio};
use std::mem;

use serde_json;
use hyper::client::Client;
use notify_rust::Notification;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use notify_rust::NotificationUrgency::Critical;

use settings;

lazy_static! {
    static ref CLIENT: Client = {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        Client::with_connector(connector)
    };
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct YandexResponse {
    code: u16,
    lang: String,
    text: [String; 1],
}

pub fn translate(text: &str) -> String {
    let url = settings::YANDEX_URL.clone() + text;
    let response = CLIENT.get(&url).send().unwrap();

    let mut response: YandexResponse = serde_json::from_reader(response).unwrap();

    mem::replace(&mut response.text[0], String::new())
}

pub fn get_current_selected_text() -> String {
    let output = Command::new("xsel")
        .arg("-o")
        .output()
        .unwrap_or_else(|_| {
                            Command::new("xclip")
                                .arg("-o")
                                .output()
                                .expect("install xsel or xclip")
                        });

    unsafe { String::from_utf8_unchecked(output.stdout) }
}

pub fn show_desktop_notification(summary: &str, body: &str) {
    let time: i32;
    if body.len() <= 15 {
        time = 2500;
    } else {
        time = 2500 + (body.len() as i32 - 15) * 100;
    }

    Notification::new()
        .summary(summary)
        .body(body)
        .timeout(time)
        .urgency(Critical)
        .show()
        .unwrap();
}

pub fn open_browser(url: &str) {
    Command::new(settings::BROWSER_COMMAND)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg(url)
        .spawn()
        .unwrap();
}

pub fn is_url(text: &str) -> bool {
    const URL_START: [&str; 3] = ["http://", "https://", "www."];

    for start in URL_START.iter() {
        if text.starts_with(start) {
            return true;
        }
    }
    false
}

pub fn warmup() {
    use lazy_static::initialize;

    initialize(&settings::YANDEX_URL);
    initialize(&settings::GOOGLE_TRANSLATOR_URL);
    initialize(&settings::GOOGLE_URL);
    initialize(&settings::WIKTIONARY_URL);
    initialize(&CLIENT);

    let _ = get_current_selected_text();
}