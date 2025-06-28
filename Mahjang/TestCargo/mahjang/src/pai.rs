#[derive(PartialEq, Clone)]
pub enum Tileset {
    Dots,       // 筒子（ピンズ）
    Bamboos,    // 索子（ソウズ）
    Characters, // 萬子（マンズ/ワンズ）
    Winds,      // 四風牌 / 風牌(東・南・西・北)
    Dragons,    // 三元牌(白・発・中)
    Seasons,    // 季節牌(四季 春・夏・秋・冬)
    Flowers,    // 花牌(四君子 梅・蘭・菊・竹)
}

#[derive(Clone)]
pub enum Tiletype {
    Suited,  // 数字牌
    Honors,  // 文字牌
    Bonus,   // 花牌
}

#[derive(Clone)]
pub struct Pai {
    pub es: String,         // エスケープシーケンス文字(escape sequence character)
    pub face: Tileset,      // 牌の種類[0-6]
    pub settype: Tiletype,  // 牌の属種[0-2]
    pub idx: u8,            // 牌の番号(シリアルナンバー)[0-143]
    pub number: u8,         // 牌の表示番号[0-8]/[0-3]/[0-2]
    pub doralv: u8,         // ドラの重複度
    pub dora: bool,         // ドラ表示牌
    pub red: bool,          // 赤牌(ドラ)
    pub used: bool,         // 使用するかどうか
}
//牌の初期化はBoard::create_boardでやる
impl Pai {
    pub fn make_es(&mut self) -> () {
        let mut facechar = String::new();
        let mut color = String::new();
        let mut dora = String::new();
        match self.face {
            Tileset::Characters => {
                facechar = self.get_number();
                if self.red == true {
                    // 赤牌
                    color = "91;107".to_string(); // 太字・文字色明赤・背景色明白
                } else {
                    color = "30;107".to_string(); // 文字色黒・背景色明白
                }
                
            }
            Tileset::Dots => {
                facechar = self.get_number();
                if self.red == true {
                    // 赤牌
                    color = "91;107".to_string(); // 文字色明赤・背景色明白
                } else {
                    color = "94;107".to_string(); // 文字色明青・背景色明白
                }
                
            }
            Tileset::Bamboos => {
                facechar = self.get_number();
                if self.red == true {
                    // 赤牌
                    color = "1;91;107".to_string(); // 太字・文字色明赤・背景色白
                } else {
                    color = "1;32;107".to_string(); // 太字・文字色緑・背景色白
                }
                
            }
            Tileset::Winds => {
                match &self.number {
                    0 => facechar = "東".to_string(),
                    1 => facechar = "南".to_string(),
                    2 => facechar = "西".to_string(),
                    3 => facechar = "北".to_string(),
                    _ => facechar = "　".to_string(),
                }
                color = "30;107".to_string(); // 太字・文字色黒・背景色明白
            }
            Tileset::Dragons => {
                match &self.number {
                    0 => {
                        facechar = "白".to_string();
                        color = "30;107".to_string(); // 太字・文字色黒・背景色明白
                    }
                    1 => {
                        facechar = "発".to_string();
                        color = "32;107".to_string(); // 太字・文字色緑・背景色明白
                    }
                    2 => {
                        facechar = "中".to_string();
                        color = "91;107".to_string(); // 太字・文字色明赤・背景色明白
                    }
                    _ => facechar = "　".to_string(),
                }
            }
            Tileset::Seasons => {
                match &self.number {
                    0 => facechar = "春".to_string(),
                    1 => facechar = "夏".to_string(),
                    2 => facechar = "秋".to_string(),
                    3 => facechar = "冬".to_string(),
                    _ => facechar = "　".to_string(),
                }
                color = "96;107".to_string(); // シアン・背景色明白
            }
            Tileset::Flowers => {
                match &self.number {
                    0 => facechar = "梅".to_string(),
                    1 => facechar = "蘭".to_string(),
                    2 => facechar = "菊".to_string(),
                    3 => facechar = "竹".to_string(),
                    _ => facechar = "　".to_string(),
                }
                color = "95;107".to_string(); // マゼンタ・背景色明白
            }
        }
        // ドラには下線を付ける
        if self.doralv > 0 {
            dora = ";4".to_string();
        }
        self.es = "\x1b[".to_string() + &color + &dora + "m" + &facechar + "\x1b[0m";
    }

    pub fn get_number(&self) -> String {
        let mut num: u8 = self.number;
        let mut ret = String::new();
        match self.face {
            Tileset::Characters => {
                // 萬子
                match self.number {
                    0 => ret = "一".to_string(),
                    1 => ret = "二".to_string(),
                    2 => ret = "三".to_string(),
                    3 => ret = "四".to_string(),
                    4 => ret = "五".to_string(),
                    5 => ret = "六".to_string(),
                    6 => ret = "七".to_string(),
                    7 => ret = "八".to_string(),
                    8 => ret = "九".to_string(),
                    _ => ret = "　".to_string(),
                }
            }
            Tileset::Dots => {
                // 筒子
                num += 1;
                ret = char::from_u32(0xFF10 + num as u32)
                    .expect("num should be 0-9 only, resulting in valid full-width digit")
                    .to_string();
            }
            Tileset::Bamboos => {
                // 索子
                num += 1;
                ret = char::from_u32(0xFF10 + num as u32)
                    .expect("num should be 0-9 only, resulting in valid full-width digit")
                    .to_string();
            }
            _ => {
                ret = "　".to_string();
            }
        }
        ret
    }
}