use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};

fn main() {
    println!("キーを押してください（Zキーで終了）");

    loop {
        if let Ok(Event::Key(KeyEvent { code, kind, .. })) = read() {
            if kind != KeyEventKind::Press {
                continue;
            }

            match code {
                KeyCode::Up => println!("↑ 上"),
                KeyCode::Down => println!("↓ 下"),
                KeyCode::Left => println!("← 左"),
                KeyCode::Right => println!("→ 右"),
                KeyCode::Char(c) => {
                    println!("文字: '{}', ASCIIコード: {}", c, c as u8);
                    if c == 'Z' {
                        println!("Zキーが押されました。終了します。");
                        break;
                    }
                }
                other => {
                    println!("その他のキー: {:?}", other);
                }
            }
        }
    }
}
