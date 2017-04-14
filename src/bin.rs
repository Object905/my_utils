#[macro_use]
extern crate lazy_static;
extern crate hyper;
extern crate hyper_native_tls;
extern crate rustc_serialize;
extern crate notify_rust;
extern crate bindkey;

mod actions;
mod settings;
mod tools;

use bindkey::*;

fn main() {
    HotKey::new(keysym::XK_q, vec![Modifier::Window], TriggerOn::Press)
        .add(actions::translate_notify_selected);

    HotKey::new(keysym::XK_w, vec![Modifier::Window], TriggerOn::Press)
        .add(actions::google_translate_selected);

    HotKey::new(keysym::XK_f, vec![Modifier::Window], TriggerOn::Press)
        .add(actions::google_selected);

    HotKey::new(keysym::XK_g, vec![Modifier::Window], TriggerOn::Press)
        .add(actions::wiktionary_selected);

    start();
}
