use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};

fn main() {
    println!("キーを押してください（Zキーで終了）");

    loop {
        if let Ok(Event::Key(KeyEvent { code, kind, .. })) = read() {
            if kind != KeyEventKind::Press {
                continue;
            }

            match code {
                KeyCode::Char(c) => {
                    println!("文字: '{}', ASCIIコード: {}, KeyCode: {:?}", c, c as u8, code);
                    if c == 'Z' {
                        println!("Zキーが押されました。終了します。");
                        break;
                    }
                }
                _ => {
                    println!("特殊キー: KeyCode::{:?}", code);
                }
            }
        }
    }
}
