# ■rustでコンソール上の矢印キーなどの入力を検知する方法

Rustでコンソール上の矢印キー入力を検知するには、**標準ライブラリでは対応が難しい**ため、以下のような外部クレートを使うのが一般的です：

---

## 🧰 よく使われるクレートと方法

### 1. **crossterm**
- クロスプラットフォーム対応（Windows, Linux, macOS）
- 矢印キーなどの特殊キーも検知可能
- 非常に人気があり、ドキュメントも充実

```toml
# Cargo.toml
[dependencies]
crossterm = "0.27"
```

```rust
use crossterm::event::{read, Event, KeyCode};

fn main() {
    println!("矢印キーを押してください（Escで終了）");

    loop {
        if let Ok(Event::Key(event)) = read() {
            match event.code {
                KeyCode::Up => println!("↑ 上"),
                KeyCode::Down => println!("↓ 下"),
                KeyCode::Left => println!("← 左"),
                KeyCode::Right => println!("→ 右"),
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }
}
```

---

### 2. **termion**
- UNIX系（Linux/macOS）向けの軽量クレート
- Rawモードでキー入力を即時取得可能

```toml
[dependencies]
termion = "1.5"
```

```rust
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "矢印キーを押してください（qで終了）\r\n").unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Up => println!("↑ 上"),
            Key::Down => println!("↓ 下"),
            Key::Left => println!("← 左"),
            Key::Right => println!("→ 右"),
            Key::Char('q') => break,
            _ => {}
        }
    }
}
```

---

### 3. **rustyline / dialoguer / tui-rs**
- 対話的CLIやTUIアプリを作る場合に便利
- 矢印キーによる選択肢移動などが簡単に実装可能
- 例: `dialoguer::Select` でリスト選択UIを作成

---

## 🧭 どれを選ぶべき？
| 用途 | おすすめクレート |
|------|------------------|
| シンプルなキー入力検知 | `crossterm` ✅ |
| UNIX専用・軽量志向 | `termion` |
| 対話的CLIやTUI構築 | `dialoguer`, `tui-rs`, `rustyline` |

---

もしカキツバタさんのMahjongプロジェクトで、コンソール上で牌の選択や操作を矢印キーで行いたい場合は、`crossterm` が最も柔軟で扱いやすいと思います。  
必要であれば、MahjongのUIに組み込む具体例も一緒に考えましょうか？

# ■crosstermを使ったものをお願いします

了解です！以下に、`crossterm` を使って**矢印キー入力を検知するシンプルなRustプログラム**を紹介します。これはクロスプラットフォーム対応で、WindowsでもLinux/macOSでも動作します。

---

## 🧪 `crossterm` を使った矢印キー検知プログラム

### 📦 `Cargo.toml`

```toml
[dependencies]
crossterm = "0.27"
```

> ✅ 最新バージョンは [crates.io/crossterm](https://crates.io/crates/crossterm) で確認できます。

---

### 🧾 `main.rs`

```rust
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
```

---

## 💡 補足ポイント

- `crossterm::event::read()` はブロッキングで、キー入力を待ちます。
- `KeyCode::Esc` でループを抜けるようにしていますが、他のキーでも条件を追加できます。
- 非ブロッキングで使いたい場合は `poll()` を併用する方法もあります。

---

## 🧭 次のステップ

- このコードをベースに、Mahjongの牌選択やカーソル移動に応用できます。
- もし「牌を矢印キーで選んでEnterで決定」みたいなUIを作りたい場合は、次のステップとして「カーソル位置の管理」や「画面の再描画」も組み込めます。

必要であれば、そういったインタラクティブなUIの雛形も一緒に作っていきましょうか？

# ■しかしこのコードでは矢印キーを1度押しても2度の表示がなされてしまいます

ご指摘の「矢印キーを1度押すと2度表示される」問題は、**Windows環境での `crossterm` の仕様**によるものです。

---

## ⚠️ 原因：`KeyEventKind::Press` と `KeyEventKind::Release` の両方が発生する

`crossterm` はバージョン 0.26 以降、**Windows環境でキーの「押下（Press）」と「離す（Release）」の両方のイベントを検知**するようになりました。  
そのため、次のようなコードでは **1回のキー入力で2回のイベントが発生**します：

```rust
if let Ok(Event::Key(event)) = read() {
    match event.code {
        KeyCode::Up => println!("↑ 上"), // ← PressとReleaseの両方で呼ばれる
        _ => {}
    }
}
```

---

## ✅ 解決策：`KeyEventKind::Press` のみを処理する

以下のように `event.kind == KeyEventKind::Press` を条件に加えることで、**キーを押したときだけ処理**するようにできます：

```rust
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
```

---

## 🧪 補足：他の環境では？

| OS | イベントの種類 | 対応の必要性 |
|----|----------------|--------------|
| Windows | `Press` + `Release` 両方 | ✅ 必要 |
| Linux/macOS | `Press` のみ | ❌ 不要（ただし入れても問題なし） |

---

これで1回の矢印キー入力に対して1回だけ表示されるようになるはずです。  
もしこのコードをMahjongの牌選択などに応用したい場合も、同じ方法で安定した入力処理ができますよ。

次に進めたい方向があれば、たとえば「選択中の牌をハイライト表示する」など、UIの強化も一緒に考えられます。どうしますか？

# ■すばらしいです　次は入力したアスキーコードを表示するようにしたいです　それから終了はESCキーではなくZキーでお願いします

ありがとうございます、カキツバタさん！  
では、以下のように改良してみましょう：

---

## 🎯 改良ポイント

1. **押されたキーの ASCIIコード（数値）を表示**
2. **終了キーを `Esc` → `Z` に変更（大文字）**

---

## 🧾 改良後の `main.rs`

```rust
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
```

---

## 🧪 実行例

```
キーを押してください（Zキーで終了）
文字: 'a', ASCIIコード: 97
↑ 上
文字: 'Z', ASCIIコード: 90
Zキーが押されました。終了します。
```

---

## 💡 補足

- `KeyCode::Char(c)` を使うことで、任意の文字キーを取得できます。
- `c as u8` で ASCIIコード（バイト値）を表示しています。
- `Z` は大文字なので、**Shiftを押しながらZ** を入力してください。

---

この仕組みを使えば、たとえば「牌に対応するキーを押したときに選択する」などの入力マッピングも簡単にできます。  
次は「キー入力に応じて牌を選択・表示する」ようなUIに進めてみますか？

# ■矢印キーのキーコードは表示することはできませんか

はい、`crossterm` を使って**矢印キーを含むすべてのキー入力の「キーコード（KeyCode）」を表示する**ことが可能です。  
以下のように `KeyCode` を `{:?}` で表示すれば、矢印キーも `"Up"`, `"Down"` などとして表示されます。

---

## 🧾 すべてのキーコードを表示するサンプル

```rust
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
```

---

## 🧪 実行例

```
キーを押してください（Zキーで終了）
特殊キー: KeyCode::Up
特殊キー: KeyCode::Left
文字: 'a', ASCIIコード: 97, KeyCode: Char('a')
文字: 'Z', ASCIIコード: 90, KeyCode: Char('Z')
Zキーが押されました。終了します。
```

---

## 💡 補足

- `KeyCode::{}` の列挙型は [`crossterm::event::KeyCode`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html) に定義されています。
- 矢印キーは `KeyCode::Up`, `KeyCode::Down`, `KeyCode::Left`, `KeyCode::Right` として識別されます。
- `F1`〜`F12` などのファンクションキーも `KeyCode::F(n)` で取得可能です。

---

このコードをベースに、たとえば「どのキーが押されたかをログに残す」「特定のキーに機能を割り当てる」など、柔軟な入力処理が可能になります。

次は、押されたキーに応じてカーソルを移動したり、牌を選択したりするようなUIに進めてみますか？

