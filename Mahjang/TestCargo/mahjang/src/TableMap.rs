use crate::pai::Pai;
use crate::board::Board;
use crate::wall::Wall;
use rand::seq::SliceRandom;
use rand::thread_rng;

// 牌の配置マップを表示するための定数
pub const ROWS:usize=27; //牌の行数
pub const COLS:usize=37; //牌の列数

pub fn disptable() {
    let tablecolorchar="\x1b[48;2;0;60;0m　\x1b[0m".to_string(); //背景色濃緑
    let mut tablemap: [[String; COLS]; ROWS] = std::array::from_fn(|_| {
        std::array::from_fn(|_| tablecolorchar.clone())
    });
    
    let mut bd=Board{
        pais:Vec::new(),
        moves:0,        //何手目か
        akadra:true,    //赤ドラ(5の牌)
        seasons:false,   //季節牌使用
        flowers:false,   //花牌使用
        back:true,      //後付け
        sanma:false,     //3人麻雀(3-player Mahjong)
    };

    // 牌の配置マップを生成
    let mut pais=bd.create_board();
    bd.pais=pais.clone();
    bd.pais.shuffle(&mut rand::thread_rng()); // 牌をシャッフル

    // Create a Wall instance for one of the walls (e.g., East, wallno = 0)
    let mut wall = Wall::new(2,true); // wallno = 0 (East), dispback = true
    // 俯瞰指定　0=東, 1=南, 2=西, 3=北
    wall.make_position(3);
    
    // Debug: Print tile positions for the specified wall
    let mut k:usize=0; //牌のインデックス
    for j in 0..4 {
        for i in 0..34 { //34-40
        //for i in 0..40 { //34-40
            // 牌の配置マップに牌の位置を設定
            let pos = wall.tiles[i][j]; // 牌の位置を取得

            ///////////////////////////////////////
            // 王牌、花牌、ドラ表示、王牌の山分け　処理
            ///////////////////////////////////////
            //

            //tablemap[pos.row][pos.col] = wall.backes.clone(); // 牌の裏面のエスケープシーケンスを設定
            tablemap[pos.row][pos.col] = bd.pais[k].es.clone(); // 牌の表面のエスケープシーケンスを設定
            k+=1; // 牌のインデックスを更新
        }
    }

    // 牌の配置マップを表示
    let dbg = true; // デバッグフラグ(マップ行列番号表示)
    let mut s: String; // ワイド文字化された数字を格納する変数
    if dbg {
        print!("　"); // 左上の空白
        for col in 0..(COLS) {
            s = get_fullwidth_number(col % 10); // 列番号をワイド文字化
            print!("{}", s);
        }
        println!(); //改行
    }
    for row in 0..ROWS {
        if dbg {
            s= get_fullwidth_number(row % 10); // 行番号をワイド文字化
            print!("{}",s);
        }
        for col in 0..COLS {
            print!("{}", tablemap[row][col]); //牌の配置マップを表示
        }
        if dbg {
            s = get_fullwidth_number(row % 10); // 行番号をワイド文字化
            print!("{}", s);
        }
        println!(); //改行
    }
    if dbg {
        print!("　"); // 左下の空白
        for col in 0..(COLS) {
            s = get_fullwidth_number(col % 10); // 列番号をワイド文字化
            print!("{}", s);
        }
        println!(); //改行
    }
}

//■数値のワイド文字化
fn to_fullwidth_digit(n: usize) -> char {
    // 0〜9 の範囲であることを確認
    assert!(n <= 9, "0〜9の数字のみ対応しています");
    char::from_u32(0xFF10 + n as u32).unwrap()
}
fn get_fullwidth_number(number: usize) -> String {
    let fullwidth: String = number
        .to_string()
        .chars()
        .map(|c| {
            if c.is_ascii_digit() {
                to_fullwidth_digit(c.to_digit(10).unwrap() as usize)
            } else {
                c
            }
        })
        .collect();
    fullwidth
}