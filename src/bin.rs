#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate hyper;
extern crate hyper_native_tls;
extern crate notify_rust;
extern crate bindkey;

mod actions;
mod settings;
mod tools;

use bindkey::{CallbackStorage, HotKey, Modifier, TriggerOn, keysym};

fn main() {
    let mut storage = CallbackStorage::new();

    storage.add(&HotKey::new(keysym::XK_q, vec![Modifier::Window], TriggerOn::Press),
                actions::translate_notify_selected);

    storage.add(&HotKey::new(keysym::XK_w, vec![Modifier::Window], TriggerOn::Press),
                actions::google_translate_selected);

    storage.add(&HotKey::new(keysym::XK_f, vec![Modifier::Window], TriggerOn::Press),
                actions::google_selected);

    storage.add(&HotKey::new(keysym::XK_g, vec![Modifier::Window], TriggerOn::Press),
                actions::wiktionary_selected);

    tools::warmup();

    bindkey::start(storage);
}
