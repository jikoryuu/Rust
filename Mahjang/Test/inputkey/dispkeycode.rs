//キーボードで入力されたキーコードを表示する

fn main() {
    // キーボードからの入力を受け付ける
    println!("キーを入力してください（終了するにはCtrl+Cを押してください）:");

    // 無限ループでキー入力を待つ
    loop {
        // 標準入力から1文字読み込む
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("読み込みに失敗しました");

        // 入力された文字列のバイトコードを表示
        for byte in input.bytes() {
            println!("入力されたキーのコード: {}", byte);
        }
    }
}