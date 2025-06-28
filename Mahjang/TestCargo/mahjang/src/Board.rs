use crate::pai::{Pai, Tileset, Tiletype};

pub struct Board {
    pub pais: Vec<Pai>,
    pub moves: u8,      // 何手目か
    pub akadra: bool,   // 赤ドラ(5の牌)
    pub seasons: bool,  // 季節牌使用
    pub flowers: bool,  // 花牌使用
    pub back: bool,     // 後付け
    pub sanma: bool,    // 3人麻雀(3-player Mahjong)
}

impl Board {
    pub fn create_board(&self) -> Vec<Pai> {
        let mut idx: u8 = 0;
        let mut pais: Vec<Pai> = Vec::new();
        let mut p = Pai {
            es: "".to_string(),
            face: Tileset::Characters,
            settype: Tiletype::Suited, // 数字牌
            idx: 0,
            number: 0,
            doralv: 0,
            red: false,
            used: true,
            dora: false,
        };
        // 数字牌 0.萬子、1.筒子、2.索子
        for i in 0..3 {
            for j in 0..9 {
                // 0.=1、8.=9
                for k in 0..4 {
                    // 4セット
                    let reddra: bool = self.akadra;
                    let mut f: Tileset = Tileset::Characters;
                    match i {
                        0 => f = Tileset::Characters,
                        1 => f = Tileset::Dots,
                        2 => f = Tileset::Bamboos,
                        _ => f = Tileset::Characters,
                    }
                    let mut d = 0;
                    let mut r = false;
                    if reddra && j == 4 && k == 0 {
                        // 5の赤ドラは数字牌種に1つだけ
                        d = 1;
                        r = true;
                    } else {
                        d = 0;
                        r = false;
                    }
                    let mut u = true;
                    if self.sanma {
                        if f == Tileset::Characters {
                            if 0 < j && j < 8 {
                                u = false;
                            }
                        }
                    }
                    if u {
                        p.face = f;
                        p.settype = Tiletype::Suited; // 数字牌
                        p.idx = idx;
                        p.number = j;
                        p.doralv = d;
                        p.red = r;
                        p.used = u;
                        p.make_es();
                        pais.push(p.clone());
                        idx += 1;
                    }
                }
            }
        }
        // 四風牌 0.東、1.南、2.西、3.北
        for i in 0..4 {
            for _j in 0..3 {
                // 各種4枚づつ
                let mut s = String::new();
                match i {
                    0 => s = "東".to_string(),
                    1 => s = "南".to_string(),
                    2 => s = "西".to_string(),
                    3 => s = "北".to_string(),
                    _ => s = "　".to_string(),
                }
                p.es = s;
                p.face = Tileset::Winds;      // 三元牌(白・発・中)
                p.settype = Tiletype::Honors; // 文字牌
                p.idx = idx;
                p.number = i;
                p.doralv = 0;
                p.red = false;
                p.used = true;
                p.make_es();
                pais.push(p.clone());
                idx += 1;
            }
            pais.push(p.clone());
            idx += 1;
        }
        // 三元牌 0.白、1.発 2.中
        for i in 0..3 {
            for _j in 0..3 {
                // 各種4枚づつ
                let mut s = String::new();
                match i {
                    0 => s = "白".to_string(),
                    1 => s = "発".to_string(),
                    2 => s = "中".to_string(),
                    _ => s = "　".to_string(),
                }
                p.es = s;
                p.face = Tileset::Dragons;    // 三元牌(白・発・中)
                p.settype = Tiletype::Honors; // 文字牌
                p.idx = idx;
                p.number = i;
                p.doralv = 0;
                p.red = false;
                p.used = true;
                p.make_es();
                pais.push(p.clone());
                idx += 1;
            }
            pais.push(p.clone());
            idx += 1;
        }
        // 季節牌 0.春、1.夏、2.秋、3.冬
        if self.seasons {
            for i in 0..4 {
                let mut s = String::new();
                match i {
                    0 => s = "春".to_string(),
                    1 => s = "夏".to_string(),
                    2 => s = "秋".to_string(),
                    3 => s = "冬".to_string(),
                    _ => s = "　".to_string(),
                }
                p.es = s;
                p.face = Tileset::Seasons;    // 季節牌
                p.settype = Tiletype::Bonus;  // 花牌
                p.idx = idx;
                p.number = i;
                p.doralv = 0;
                p.red = false;
                p.used = true;
                p.make_es();
                pais.push(p.clone());
                idx += 1;
            }
        }

        // 花牌(四君子 梅・蘭・菊・竹)
        if self.flowers {
            for i in 0..4 {
                let mut s = String::new();
                match i {
                    0 => s = "梅".to_string(),
                    1 => s = "蘭".to_string(),
                    2 => s = "菊".to_string(),
                    3 => s = "竹".to_string(),
                    _ => s = "　".to_string(),
                }
                p.es = s;
                p.face = Tileset::Flowers;    // 花牌
                p.settype = Tiletype::Bonus;  // 花牌
                p.idx = idx;
                p.number = i;
                p.doralv = 0;
                p.red = false;
                p.used = true;
                p.make_es();
                pais.push(p.clone());
                idx += 1;
            }
        }

        pais
    }
}