#[derive(Clone, Copy)]
pub struct Position {
    pub row: usize, // x座標
    pub col: usize, // y座標
    pub pai: pai, // 牌のインデックス
}

impl Position {
    pub fn new_with_index(i: usize) -> Self {
        const COLS: usize = 136; // 例
        Position {
            col: i % COLS,
            row: i / COLS,
        }
    }
}

#[derive(Clone)]
pub struct Wall {
    pub wallno: u8,                 // 牌山の番号[0-4(+1)] サンマは[0-3(+1)]
    pub tiles: [[Position; 4]; 34], // 牌の位置(牌の位置はrow,colで表現)
    pub used: bool,                 // 使用済みかどうか(使用済みの場合usedesを表示する)
    pub dispback: bool,             // 裏表示するかどうか
    pub usedes: String,             // 使用済みの表示文字列(全角スペースで背景深緑のエスケープシーケンス)
    pub backes: String,             // 裏表示の文字列(文字🟨)[背景は深緑]のエスケープシーケンス
    pub facees: String,             // エスケープシーケンス文字(escape sequence character)
}

impl Wall {
    pub fn new(wallno:u8,dispback: bool) -> Wall {
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

    pub fn make_position(&mut self) {
        // 東 (East)
        self.tiles[0][0] = Position { row: 23, col: 10 };
        self.tiles[1][0] = Position { row: 24, col: 10 };
        self.tiles[2][0] = Position { row: 23, col: 11 };
        self.tiles[3][0] = Position { row: 24, col: 11 };
        self.tiles[4][0] = Position { row: 23, col: 12 };
        self.tiles[5][0] = Position { row: 24, col: 12 };
        self.tiles[6][0] = Position { row: 23, col: 13 };
        self.tiles[7][0] = Position { row: 24, col: 13 };
        self.tiles[8][0] = Position { row: 23, col: 14 };
        self.tiles[9][0] = Position { row: 24, col: 14 };
        self.tiles[10][0] = Position { row: 23, col: 15 };
        self.tiles[11][0] = Position { row: 24, col: 15 };
        self.tiles[12][0] = Position { row: 23, col: 16 };
        self.tiles[13][0] = Position { row: 24, col: 16 };
        self.tiles[14][0] = Position { row: 23, col: 17 };
        self.tiles[15][0] = Position { row: 24, col: 17 };
        self.tiles[16][0] = Position { row: 23, col: 18 };
        self.tiles[17][0] = Position { row: 24, col: 18 };
        self.tiles[18][0] = Position { row: 23, col: 19 };
        self.tiles[19][0] = Position { row: 24, col: 19 };
        self.tiles[20][0] = Position { row: 23, col: 20 };
        self.tiles[21][0] = Position { row: 24, col: 20 };
        self.tiles[22][0] = Position { row: 23, col: 21 };
        self.tiles[23][0] = Position { row: 24, col: 21 };
        self.tiles[24][0] = Position { row: 23, col: 22 };
        self.tiles[25][0] = Position { row: 24, col: 22 };
        self.tiles[26][0] = Position { row: 23, col: 23 };
        self.tiles[27][0] = Position { row: 24, col: 23 };
        self.tiles[28][0] = Position { row: 23, col: 24 };
        self.tiles[29][0] = Position { row: 24, col: 24 };
        self.tiles[30][0] = Position { row: 23, col: 25 };
        self.tiles[31][0] = Position { row: 24, col: 25 };
        self.tiles[32][0] = Position { row: 23, col: 26 };
        self.tiles[33][0] = Position { row: 24, col: 26 };
        // 南 (South)
        self.tiles[0][1] = Position { row: 21, col: 33 };
        self.tiles[1][1] = Position { row: 21, col: 34 };
        self.tiles[2][1] = Position { row: 20, col: 33 };
        self.tiles[3][1] = Position { row: 20, col: 34 };
        self.tiles[4][1] = Position { row: 19, col: 33 };
        self.tiles[5][1] = Position { row: 19, col: 34 };
        self.tiles[6][1] = Position { row: 18, col: 33 };
        self.tiles[7][1] = Position { row: 18, col: 34 };
        self.tiles[8][1] = Position { row: 17, col: 33 };
        self.tiles[9][1] = Position { row: 17, col: 34 };
        self.tiles[10][1] = Position { row: 16, col: 33 };
        self.tiles[11][1] = Position { row: 16, col: 34 };
        self.tiles[12][1] = Position { row: 15, col: 33 };
        self.tiles[13][1] = Position { row: 15, col: 34 };
        self.tiles[14][1] = Position { row: 14, col: 33 };
        self.tiles[15][1] = Position { row: 14, col: 34 };
        self.tiles[16][1] = Position { row: 13, col: 33 };
        self.tiles[17][1] = Position { row: 13, col: 34 };
        self.tiles[18][1] = Position { row: 12, col: 33 };
        self.tiles[19][1] = Position { row: 12, col: 34 };
        self.tiles[20][1] = Position { row: 11, col: 33 };
        self.tiles[21][1] = Position { row: 11, col: 34 };
        self.tiles[22][1] = Position { row: 10, col: 33 };
        self.tiles[23][1] = Position { row: 10, col: 34 };
        self.tiles[24][1] = Position { row: 9, col: 33 };
        self.tiles[25][1] = Position { row: 9, col: 34 };
        self.tiles[26][1] = Position { row: 8, col: 33 };
        self.tiles[27][1] = Position { row: 8, col: 34 };
        self.tiles[28][1] = Position { row: 7, col: 33 };
        self.tiles[29][1] = Position { row: 7, col: 34 }; // Fixed: col was 24, likely a typo
        self.tiles[30][1] = Position { row: 6, col: 33 };
        self.tiles[31][1] = Position { row: 6, col: 34 }; // Fixed: col was 25, likely a typo
        self.tiles[32][1] = Position { row: 5, col: 33 };
        self.tiles[33][1] = Position { row: 5, col: 34 };
        // 西 (West)
        self.tiles[0][2] = Position { row: 3, col: 26 };
        self.tiles[1][2] = Position { row: 2, col: 26 };
        self.tiles[2][2] = Position { row: 3, col: 25 };
        self.tiles[3][2] = Position { row: 2, col: 25 };
        self.tiles[4][2] = Position { row: 3, col: 24 };
        self.tiles[5][2] = Position { row: 2, col: 24 };
        self.tiles[6][2] = Position { row: 3, col: 23 };
        self.tiles[7][2] = Position { row: 2, col: 23 };
        self.tiles[8][2] = Position { row: 3, col: 22 };
        self.tiles[9][2] = Position { row: 2, col: 22 };
        self.tiles[10][2] = Position { row: 3, col: 21 };
        self.tiles[11][2] = Position { row: 2, col: 21 };
        self.tiles[12][2] = Position { row: 3, col: 20 };
        self.tiles[13][2] = Position { row: 2, col: 20 };
        self.tiles[14][2] = Position { row: 3, col: 19 };
        self.tiles[15][2] = Position { row: 2, col: 19 };
        self.tiles[16][2] = Position { row: 3, col: 18 };
        self.tiles[17][2] = Position { row: 2, col: 18 };
        self.tiles[18][2] = Position { row: 3, col: 17 };
        self.tiles[19][2] = Position { row: 2, col: 17 };
        self.tiles[20][2] = Position { row: 3, col: 16 };
        self.tiles[21][2] = Position { row: 2, col: 16 };
        self.tiles[22][2] = Position { row: 3, col: 15 };
        self.tiles[23][2] = Position { row: 2, col: 15 };
        self.tiles[24][2] = Position { row: 3, col: 14 };
        self.tiles[25][2] = Position { row: 2, col: 14 };
        self.tiles[26][2] = Position { row: 3, col: 13 };
        self.tiles[27][2] = Position { row: 2, col: 13 };
        self.tiles[28][2] = Position { row: 3, col: 12 };
        self.tiles[29][2] = Position { row: 2, col: 12 };
        self.tiles[30][2] = Position { row: 3, col: 11 };
        self.tiles[31][2] = Position { row: 2, col: 11 };
        self.tiles[32][2] = Position { row: 3, col: 10 };
        self.tiles[33][2] = Position { row: 2, col: 10 };
        // 北 (North)
        self.tiles[0][3] = Position { row: 5, col: 3 };
        self.tiles[1][3] = Position { row: 5, col: 2 };
        self.tiles[2][3] = Position { row: 6, col: 3 };
        self.tiles[3][3] = Position { row: 6, col: 2 };
        self.tiles[4][3] = Position { row: 7, col: 3 };
        self.tiles[5][3] = Position { row: 7, col: 2 };
        self.tiles[6][3] = Position { row: 8, col: 3 };
        self.tiles[7][3] = Position { row: 8, col: 2 };
        self.tiles[8][3] = Position { row: 9, col: 3 };
        self.tiles[9][3] = Position { row: 9, col: 2 };
        self.tiles[10][3] = Position { row: 10, col: 3 };
        self.tiles[11][3] = Position { row: 10, col: 2 };
        self.tiles[12][3] = Position { row: 11, col: 3 };
        self.tiles[13][3] = Position { row: 11, col: 2 };
        self.tiles[14][3] = Position { row: 12, col: 3 };
        self.tiles[15][3] = Position { row: 12, col: 2 };
        self.tiles[16][3] = Position { row: 13, col: 3 };
        self.tiles[17][3] = Position { row: 13, col: 2 };
        self.tiles[18][3] = Position { row: 14, col: 3 };
        self.tiles[19][3] = Position { row: 14, col: 2 };
        self.tiles[20][3] = Position { row: 15, col: 3 };
        self.tiles[21][3] = Position { row: 15, col: 2 };
        self.tiles[22][3] = Position { row: 16, col: 3 };
        self.tiles[23][3] = Position { row: 16, col: 2 };
        self.tiles[24][3] = Position { row: 17, col: 3 };
        self.tiles[25][3] = Position { row: 17, col: 2 };
        self.tiles[26][3] = Position { row: 18, col: 3 };
        self.tiles[27][3] = Position { row: 18, col: 2 };
        self.tiles[28][3] = Position { row: 19, col: 3 };
        self.tiles[29][3] = Position { row: 19, col: 2 };
        self.tiles[30][3] = Position { row: 20, col: 3 };
        self.tiles[31][3] = Position { row: 20, col: 2 };
        self.tiles[32][3] = Position { row: 21, col: 3 };
        self.tiles[33][3] = Position { row: 21, col: 2 };
    }
}


fn main() {
    // Create a Wall instance for one of the walls (e.g., East, wallno = 0)
    let mut wall = Wall::new(0,true); // wallno = 0 (East), dispback = true
    wall.make_position();

    // Debug: Print tile positions for the specified wall
    for _j in 0..4 {
        for _i in 0..34 {
            print!(
                "{}", wall.backes
            );
        }
        println!();
        println!();
    }
}