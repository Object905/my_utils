use std::process::{Command, Stdio};
use std::io::Read;

use rustc_serialize::json::Json;

use hyper::client::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use notify_rust::Notification;
use notify_rust::NotificationUrgency::Critical;

use settings;

lazy_static! {
    static ref CLIENT: Client = {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        Client::with_connector(connector)
    };
}

pub fn translate(text: &str) -> String {
    let url = format!("https://translate.yandex.net/api/v1.5/tr.json/translate?\
        key={api_key}&lang={lang}&text={text}",
                      api_key = settings::API_KEY,
                      lang = *settings::DIRECTION_LANG,
                      text = text);
    let mut response = CLIENT.get(&url).send().unwrap();
    let mut buf = String::new();
    response.read_to_string(&mut buf).unwrap();
    let json = Json::from_str(&buf).unwrap();


    let text = json.find("text").unwrap()[0]
        .as_string()
        .unwrap()
        .trim_left_matches('"')
        .trim_right_matches('"'); // remove quotes

    text.to_owned()
}

pub fn get_current_selected_text() -> String {
    let output =
        Command::new("xsel")
            .arg("-o")
            .output()
            .unwrap_or_else(|_| {
                Command::new("xclip").arg("-o").output().expect("install xsel or xclip")
            });
    String::from_utf8_lossy(&output.stdout).into_owned()
}

pub fn show_desktop_notification(summary: &str, body: &str) {
    let time: i32;
    if body.len() <= 15 {
        time = 2500;
    } else {
        time = (2500 + (body.len() - 15) * 100) as i32;
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
    Command::new(&*settings::BROWSER_COMMAND)
        .stdout(Stdio::null())
        .arg(url)
        .spawn()
        .unwrap();
}
