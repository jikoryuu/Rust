//エスケープシーケンスによるコンソール画面のクリア
//https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed?newreg=cd4ed8f1a827477f9ec7e4b3e6f85ee2
use std::io::Write;
fn main() {
    println!("コンソールをクリアします...");
    // エスケープシーケンスを使用してコンソールをクリア
    print!("\x1Bc");
    // クリア後のメッセージを表示
    println!("コンソールがクリアされました。");
    // 画面を更新するためにフラッシュ
    std::io::stdout().flush().unwrap();
}