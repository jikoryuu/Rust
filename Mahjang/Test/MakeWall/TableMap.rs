mod pai;
mod board;
mod wall;

use board::Board;
use wall::Wall;

// 牌の配置マップを表示するための定数
const ROWS:usize=27; //牌の行数
const COLS:usize=37; //牌の列数

fn main() {
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

    // Create a Wall instance for one of the walls (e.g., East, wallno = 0)
    let mut wall = Wall::new(2,true); // wallno = 0 (East), dispback = true
    // 俯瞰指定　0=東, 1=南, 2=西, 3=北
    wall.make_position(3);
    
    // Debug: Print tile positions for the specified wall
    let mut k:usize=0; //牌のインデックス
    for j in 0..4 {
        for i in 0..34 {
            // 牌の配置マップに牌の位置を設定
            let pos = wall.tiles[i][j]; // 牌の位置を取得
            //tablemap[pos.row][pos.col] = wall.backes.clone(); // 牌の裏面のエスケープシーケンスを設定
            tablemap[pos.row][pos.col] = pais[k].es.clone(); // 牌の表面のエスケープシーケンスを設定
            k+=1; // 牌のインデックスを更新
        }
    }   
    // 牌の配置マップを表示
    for row in 0..ROWS {    
        for col in 0..COLS {
            print!("{}", tablemap[row][col]); //牌の配置マップを表示
        }
        println!(); //改行
    }
}