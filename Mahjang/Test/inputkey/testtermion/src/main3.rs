use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};

fn main() {
    println!("矢印キーを押してください（Escで終了）");

    loop {
        if let Ok(Event::Key(KeyEvent { code, kind, .. })) = read() {
            if kind != KeyEventKind::Press {
                continue; // 押下以外（Releaseなど）は無視
            }

            match code {
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
    }
}
