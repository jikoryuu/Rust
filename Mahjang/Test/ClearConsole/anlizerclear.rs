fn main() {
    // エスケープシーケンスを使用してコンソールをクリア
    print!("\x1B[2J\x1B[1;1H");
    
    // クリア後のメッセージを表示
    println!("コンソールがクリアされました。");
    
    // 画面を更新するためにフラッシュ
    std::io::stdout().flush().unwrap();
}