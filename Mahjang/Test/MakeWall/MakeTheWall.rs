#[derive(Clone)]
#[derive(Copy)]
pub struct Position {
    pub row: usize,          //x座標
    pub col: usize,          //y座標
}
impl Position {
    pub fn new_with_index(i: usize) -> Self {
        // 行・列を例えば COLS 列ごとに変換
        const COLS: usize = 34; // 例
        Position {
            col: i % COLS,
            row: i / COLS,
        }
    }
}

#[derive(Clone)]
pub struct Wall{     //牌山
    pub wallno:u8,                  //牌山の番号[0-4(+1)] サンマは[0-3(+1)]
    pub tiles:[[Position;4];34],    //牌の位置(牌の位置はrow,colで表現)
    pub used:bool,                  //使用済みかどうか(使用済みの場合usedesを表示する)
    pub dispback:bool,              //裏表示するかどうか
    pub usedes:String,              //使用済みの表示文字列(全角スペースで背景深緑のエスケープシーケンス)
    pub backes:String,              //裏表示の文字列(文字🟨)[背景は深緑]のエスケープシーケンス
    pub facees:String,              //エスケープシーケンス文字(escape sequence character)
}

impl Wall {
    pub fn new(wallno: u8, dispback: bool) -> Wall {
        Wall {
            wallno,
            tiles: [[Position { row: 0, col: 0 }; 4]; 34],
            used: false,
            dispback,
            usedes: String::from("\x1b[48;2;0;60;0m　\x1b[0m"), // 全角スペース
            backes: String::from("\x1b[48;2;0;60;0m🟨\x1b[0m"), // 背景深緑のエスケープシーケンス
            facees: String::from(""), // 牌の表面のエスケープシーケンス
        }
    }
    pub fn make_position(&mut self){
        //東(起家)
        self.tiles[0][0].row = 23;
        self.tiles[0][0].col = 10;
        self.tiles[1][0].row = 24;
        self.tiles[1][0].col = 10;
        self.tiles[2][0].row = 23;
        self.tiles[2][0].col = 11;
        self.tiles[3][0].row = 24;
        self.tiles[3][0].col = 11;
        self.tiles[4][0].row = 24;
        self.tiles[4][0].col = 12;
        self.tiles[5][0].row = 24;
        self.tiles[5][0].col = 12;
        self.tiles[6][0].row = 24;
        self.tiles[6][0].col = 13;
        self.tiles[7][0].row = 24;
        self.tiles[7][0].col = 13;
        self.tiles[8][0].row = 24;
        self.tiles[8][0].col = 14;
        self.tiles[9][0].row = 24;
        self.tiles[9][0].col = 14;
        self.tiles[10][0].row = 24;
        self.tiles[10][0].col = 15;
        self.tiles[11][0].row = 24;
        self.tiles[11][0].col = 15;
        self.tiles[12][0].row = 24;
        self.tiles[12][0].col = 16;
        self.tiles[13][0].row = 24;
        self.tiles[13][0].col = 16;
        self.tiles[14][0].row = 24;
        self.tiles[14][0].col = 17;
        self.tiles[15][0].row = 24;
        self.tiles[15][0].col = 17;
        self.tiles[16][0].row = 24;
        self.tiles[16][0].col = 18;
        self.tiles[17][0].row = 24;
        self.tiles[17][0].col = 18;
        self.tiles[18][0].row = 24;
        self.tiles[18][0].col = 19;
        self.tiles[19][0].row = 24;
        self.tiles[19][0].col = 19;
        self.tiles[20][0].row = 24;
        self.tiles[20][0].col = 20;
        self.tiles[21][0].row = 24;
        self.tiles[21][0].col = 20;
        self.tiles[22][0].row = 24;
        self.tiles[22][0].col = 21;
        self.tiles[23][0].row = 24;
        self.tiles[23][0].col = 21;
        self.tiles[24][0].row = 24;
        self.tiles[24][0].col = 22;
        self.tiles[25][0].row = 24;
        self.tiles[25][0].col = 22;
        self.tiles[26][0].row = 24;
        self.tiles[26][0].col = 23;
        self.tiles[27][0].row = 24;
        self.tiles[27][0].col = 23;
        self.tiles[28][0].row = 24;
        self.tiles[28][0].col = 24;
        self.tiles[29][0].row = 24;
        self.tiles[29][0].col = 24;
        self.tiles[30][0].row = 24;
        self.tiles[30][0].col = 25;
        self.tiles[31][0].row = 24;
        self.tiles[31][0].col = 25;
        self.tiles[32][0].row = 24;
        self.tiles[32][0].col = 26;
        self.tiles[33][0].row = 24;
        self.tiles[33][0].col = 26;
        //南
        self.tiles[0][1].row = 21;
        self.tiles[0][1].col = 33;
        self.tiles[1][1].row = 21;
        self.tiles[1][1].col = 34;
        self.tiles[2][1].row = 20;
        self.tiles[2][1].col = 33;
        self.tiles[3][1].row = 20;
        self.tiles[3][1].col = 34;
        self.tiles[4][1].row = 19;
        self.tiles[4][1].col = 33;
        self.tiles[5][1].row = 19;
        self.tiles[5][1].col = 34;
        self.tiles[6][1].row = 18;
        self.tiles[6][1].col = 33;
        self.tiles[7][1].row = 18;
        self.tiles[7][1].col = 34;
        self.tiles[8][1].row = 17;
        self.tiles[8][1].col = 33;
        self.tiles[9][1].row = 17;
        self.tiles[9][1].col = 34;
        self.tiles[10][1].row = 16;
        self.tiles[10][1].col = 33;
        self.tiles[11][1].row = 16;
        self.tiles[11][1].col = 34;
        self.tiles[12][1].row = 15;
        self.tiles[12][1].col = 33;
        self.tiles[13][1].row = 15;
        self.tiles[13][1].col = 34;
        self.tiles[14][1].row = 14;
        self.tiles[14][1].col = 33;
        self.tiles[15][1].row = 14;
        self.tiles[15][1].col = 34;
        self.tiles[16][1].row = 13;
        self.tiles[16][1].col = 33;
        self.tiles[17][1].row = 13;
        self.tiles[17][1].col = 34;
        self.tiles[18][1].row = 12;
        self.tiles[18][1].col = 33;
        self.tiles[19][1].row = 12;
        self.tiles[19][1].col = 34;
        self.tiles[20][1].row = 11;
        self.tiles[20][1].col = 33;
        self.tiles[21][1].row = 11;
        self.tiles[21][1].col = 34;
        self.tiles[22][1].row = 10;
        self.tiles[22][1].col = 33;
        self.tiles[23][1].row = 10;
        self.tiles[23][1].col = 34;
        self.tiles[24][1].row = 9;
        self.tiles[24][1].col = 33;
        self.tiles[25][1].row = 9;
        self.tiles[25][1].col = 34;
        self.tiles[26][1].row = 8;
        self.tiles[26][1].col = 33;
        self.tiles[27][1].row = 8;
        self.tiles[27][1].col = 34;
        self.tiles[28][1].row = 7;
        self.tiles[28][1].col = 33;
        self.tiles[29][1].row = 7;
        self.tiles[29][1].col = 24;
        self.tiles[30][1].row = 6;
        self.tiles[30][1].col = 33;
        self.tiles[31][1].row = 6;
        self.tiles[31][1].col = 25;
        self.tiles[32][1].row = 5;
        self.tiles[32][1].col = 33;
        self.tiles[33][1].row = 5;
        self.tiles[33][1].col = 34;
        //西
        self.tiles[0][2].row = 3;
        self.tiles[0][2].col = 26;
        self.tiles[1][2].row = 2;
        self.tiles[1][2].col = 26;
        self.tiles[2][2].row = 3;
        self.tiles[2][2].col = 25;
        self.tiles[3][2].row = 2;
        self.tiles[3][2].col = 25;
        self.tiles[4][2].row = 3;
        self.tiles[4][2].col = 24;
        self.tiles[5][2].row = 2;
        self.tiles[5][2].col = 24;
        self.tiles[6][2].row = 3;
        self.tiles[6][2].col = 23;
        self.tiles[7][2].row = 2;
        self.tiles[7][2].col = 23;
        self.tiles[8][2].row = 3;
        self.tiles[8][2].col = 22;
        self.tiles[9][2].row = 2;
        self.tiles[9][2].col = 22;
        self.tiles[10][2].row = 3;
        self.tiles[10][2].col = 21;
        self.tiles[11][2].row = 2;
        self.tiles[11][2].col = 21;
        self.tiles[12][2].row = 3;
        self.tiles[12][2].col = 20;
        self.tiles[13][2].row = 2;
        self.tiles[13][2].col = 20;
        self.tiles[14][2].row = 3;
        self.tiles[14][2].col = 19;
        self.tiles[15][2].row = 2;
        self.tiles[15][2].col = 19;
        self.tiles[16][2].row = 3;
        self.tiles[16][2].col = 18;
        self.tiles[17][2].row = 2;
        self.tiles[17][2].col = 18;
        self.tiles[18][2].row = 3;
        self.tiles[18][2].col = 17;
        self.tiles[19][2].row = 2;
        self.tiles[19][2].col = 17;
        self.tiles[20][2].row = 3;
        self.tiles[20][2].col = 16;
        self.tiles[21][2].row = 2;
        self.tiles[21][2].col = 16;
        self.tiles[22][2].row = 3;
        self.tiles[22][2].col = 15;
        self.tiles[23][2].row = 2;
        self.tiles[23][2].col = 15;
        self.tiles[24][2].row = 3;
        self.tiles[24][2].col = 14;
        self.tiles[25][2].row = 2;
        self.tiles[25][2].col = 14;
        self.tiles[26][2].row = 3;
        self.tiles[26][2].col = 13;
        self.tiles[27][2].row = 2;
        self.tiles[27][2].col = 13;
        self.tiles[28][2].row = 3;
        self.tiles[28][2].col = 12;
        self.tiles[29][2].row = 2;
        self.tiles[29][2].col = 12;
        self.tiles[30][2].row = 3;
        self.tiles[30][2].col = 11;
        self.tiles[31][2].row = 2;
        self.tiles[31][2].col = 11;
        self.tiles[32][2].row = 3;
        self.tiles[32][2].col = 10;
        self.tiles[33][2].row = 2;
        self.tiles[33][2].col = 10;
        //北
        self.tiles[0][3].row = 5;
        self.tiles[0][3].col = 3;
        self.tiles[1][3].row = 5;
        self.tiles[1][3].col = 2;
        self.tiles[2][3].row = 6;
        self.tiles[2][3].col = 3;
        self.tiles[3][3].row = 6;
        self.tiles[3][3].col = 2;
        self.tiles[4][3].row = 7;
        self.tiles[4][3].col = 3;
        self.tiles[5][3].row = 7;
        self.tiles[5][3].col = 2;
        self.tiles[6][3].row = 8;
        self.tiles[6][3].col = 3;
        self.tiles[7][3].row = 8;
        self.tiles[7][3].col = 2;
        self.tiles[8][3].row = 9;
        self.tiles[8][3].col = 3;
        self.tiles[9][3].row = 9;
        self.tiles[9][3].col = 2;
        self.tiles[10][3].row = 10;
        self.tiles[10][3].col = 3;
        self.tiles[11][3].row = 10;
        self.tiles[11][3].col = 2;
        self.tiles[12][3].row = 11;
        self.tiles[12][3].col = 3;
        self.tiles[13][3].row = 11;
        self.tiles[13][3].col = 2;
        self.tiles[14][3].row = 12;
        self.tiles[14][3].col = 3;
        self.tiles[15][3].row = 12;
        self.tiles[15][3].col = 2;
        self.tiles[16][3].row = 13;
        self.tiles[16][3].col = 3;
        self.tiles[17][3].row = 13;
        self.tiles[17][3].col = 2;
        self.tiles[18][3].row = 14;
        self.tiles[18][3].col = 3;
        self.tiles[19][3].row = 14;
        self.tiles[19][3].col = 2;
        self.tiles[20][3].row = 15;
        self.tiles[20][3].col = 3;
        self.tiles[21][3].row = 15;
        self.tiles[21][3].col = 2;
        self.tiles[22][3].row = 16;
        self.tiles[22][3].col = 3;
        self.tiles[23][3].row = 16;
        self.tiles[23][3].col = 2;
        self.tiles[24][3].row = 17;
        self.tiles[24][3].col = 3;
        self.tiles[25][3].row = 17;
        self.tiles[25][3].col = 2;
        self.tiles[26][3].row = 18;
        self.tiles[26][3].col = 3;
        self.tiles[27][3].row = 18;
        self.tiles[27][3].col = 2;
        self.tiles[28][3].row = 19;
        self.tiles[28][3].col = 3;
        self.tiles[29][3].row = 19;
        self.tiles[29][3].col = 2;
        self.tiles[30][3].row = 20;
        self.tiles[30][3].col = 3;
        self.tiles[31][3].row = 20;
        self.tiles[31][3].col = 2;
        self.tiles[32][3].row = 21;
        self.tiles[32][3].col = 3;
        self.tiles[33][3].row = 21;
        self.tiles[33][3].col = 2;
    }
}

fn main() {
    const N: usize = 136; // 牌の総数
    let mut wall: [Position; N] = std::array::from_fn(|i| {
        Position::new_with_index(i)
    });
    wall.make_position();
    
    // デバッグ用: 牌の位置を表示
    for i in 0..4 {
        for j in 0..34 {
            println!("Tile [{}][{}]: ({}, {})", i, j, wall.tiles[i][j].row, wall.tiles[i][j].col);
        }
    }
}