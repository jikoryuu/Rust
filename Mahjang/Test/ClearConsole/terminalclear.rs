//この操作は失敗しました
fn main(){
    println!("コンソールをクリアします...");    
    //std::process::Command::new("clear")   // Unix系のコマンド 
    std::process::Command::new("cls")   // Windowsのコマンド
        .status()
        //.expect("Failed to clear the console");
        .unwrap();
    println!("コンソールがクリアされました。");
}