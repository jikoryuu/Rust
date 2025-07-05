use crossterm::event::{read, Event, KeyCode};
use std::io::{stdout, Write};

fn main() {
    println!("矢印キーを押してください（Escで終了）");

    loop {
        // キーイベントを待機
        match read() {
            Ok(Event::Key(event)) => {
                match event.code {
                    KeyCode::Up => println!("↑ 上"),
                    KeyCode::Down => println!("↓ 下"),
                    KeyCode::Left => println!("← 左"),
                    KeyCode::Right => println!("→ 右"),
                    KeyCode::Esc => {
                        println!("終了します。");
                        break;
                    }
                    _ => {}
                }
            }
            Err(e) => {
                eprintln!("エラー: {:?}", e);
                break;
            }
            _ => {}
        }
    }
}
