mod pai;
mod board;

use board::Board;

fn main() {
    let mut bd = Board {
        pais: Vec::new(),
        moves: 0,       // 何手目か
        akadra: true,   // 赤ドラ(5の牌)
        seasons: true,  // 季節牌使用
        flowers: true,  // 花牌使用
        back: true,     // 後付け
        sanma: false,   // 3人麻雀(3-player Mahjong)
    };
    let mut pais = bd.create_board();
    bd.pais = pais.clone();
    for i in bd.pais {
        print!("{}", i.es);
    }
    println!("");
}