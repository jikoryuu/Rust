#[derive(PartialEq)]
#[derive(Clone)]
enum Tileset{
    Dots,       //筒子（ピンズ）
    Bamboos,     //索子（ソウズ
    Characters, //萬子（マンズ/ワンズ）
    Winds,      //四風牌 / 風牌(東・南・西・北)
    Dragons,    //三元牌(白・発・中)
    Seasons,    //季節牌(四季 春・夏・秋・冬)
    Flowers,    //花牌(四君子　梅・蘭・菊・竹)
}
#[derive(Clone)]
enum Tiletype{  //牌の属種
    Suited,         //数字牌
    Honors,         //文字牌
    Bonus,          //花牌
}
#[derive(Clone)]
struct Pai{     //total 144pai
    es:String,          //エスケープシーケンス文字(escape sequence character)
    face:Tileset,       //牌の種類[0-6]
    settype:Tiletype,   //牌の属種[0-2]
    idx:u8,             //牌の番号(シリアルナンバー)[0-143]
    number:u8,          //牌の表示番号[0-8]/[0-3]/[0-2]
    doralv:u8,          //ドラの重複度
    red:bool,           //赤牌(ドラ)
    used:bool,          //使用するかどうか
}
impl Pai{
    fn make_es(&mut self)->(){
        let mut facechar=String::new();
        let mut color=String::new();
        let mut dora=String::new();
        match self.face{
            Tileset::Characters=>{
                facechar=self.get_number();
                if self.red==true{
                    color="\x1b[31".to_string();   //赤
                }else{
                    color="\x1b[37".to_string();   //白
                }
            },
            Tileset::Dots=>{
                facechar=self.get_number();
                if self.red==true{
                    color="\x1b[31".to_string();   //赤
                }else{
                    color="\x1b[34".to_string();   //青
                }
            },
            Tileset::Bamboos=>{
                facechar=self.get_number();
                if self.red==true{
                    color="\x1b[31".to_string();   //赤
                }else{
                    color="\x1b[32".to_string();   //緑
                }
            },
            Tileset::Winds=>{
                match &self.number{
                    0=>facechar="東".to_string(),
                    1=>facechar="南".to_string(),
                    2=>facechar="西".to_string(),
                    3=>facechar="北".to_string(),
                    _=>facechar="　".to_string(),
                }
                color="\x1b[37".to_string();   //白
            },
            Tileset::Dragons=>{
                match &self.number{
                    0=>{
                        facechar="白".to_string();
                        color="\x1b[37".to_string();   //白
                    },
                    1=>{
                        facechar="発".to_string();
                        color="\x1b[32".to_string();   //緑
                    },
                    2=>{
                        facechar="中".to_string();
                        color="\x1b[31".to_string();   //赤
                    },
                    _=>facechar="　".to_string(),
                }
            },
            Tileset::Seasons=>{
                match &self.number{
                    0=>facechar="春".to_string(),
                    1=>facechar="夏".to_string(),
                    2=>facechar="秋".to_string(),
                    3=>facechar="冬".to_string(),
                    _=>facechar="　".to_string(),
                }
                color="\x1b[36".to_string();   //シアン
            },
            Tileset::Flowers=>{
                match &self.number{
                    0=>facechar="梅".to_string(),
                    1=>facechar="蘭".to_string(),
                    2=>facechar="菊".to_string(),
                    3=>facechar="竹".to_string(),
                    _=>facechar="　".to_string(),
                }
                color="\x1b[35".to_string();   //マゼンタ
            },
        }
        //ドラには下線を付ける
        if self.doralv > 0 {
            dora=";4".to_string();
        }
        self.es=color + &dora + "m" + &facechar + "\x1b[0m";
    }
    fn get_number(&self)->String{
        let mut num:u8=self.number;
        let mut ret=String::new();
        match self.face{
            Tileset::Characters=>{  //萬子
                //ret=char::from_u32(0xFF10 + num as u32).expect("num should be 0-9 only, resulting in valid full-width digit").to_string();
                //㈠㈡㈢㈣㈤㈥㈦㈧㈨
                //ret=char::from_u32(0x3220 + num as u32).expect("num should be 0-9 only, resulting in valid full-width digit").to_string();
                //㊀㊁㊃㊃㊄㊅㊇㊇㊈
                //ret=char::from_u32(0x3280 + num as u32).expect("num should be 0-9 only, resulting in valid full-width digit").to_string();
                match self.number{
                    0=>ret="一".to_string(),
                    1=>ret="二".to_string(),
                    2=>ret="三".to_string(),
                    3=>ret="四".to_string(),
                    4=>ret="五".to_string(),
                    5=>ret="六".to_string(),
                    6=>ret="七".to_string(),
                    7=>ret="八".to_string(),
                    8=>ret="九".to_string(),
                    _=>ret="　".to_string(),
                }
            },
            Tileset::Dots=>{        //筒子
                //①②③④⑤⑦⑦⑧⑨
                //ret=char::from_u32(0x2460 + num as u32).expect("num should be 0-9 only, resulting in valid full-width digit").to_string(); 
                //⑴⑵⑶⑷⑸⑹⑺⑻⑼
                //ret=char::from_u32(0x2474 + num as u32).expect("num should be 0-9 only, resulting in valid full-width digit").to_string();
                //⒈⒉⒊⒌⒌⒍⒎⒏⒐
                //ret=char::from_u32(0x2488 + num as u32).expect("num should be 0-9 only, resulting in valid full-width digit").to_string();
                //⓵⓶⓷⓸⓹⓺⓻⓼⓽
                //ret=char::from_u32(0x24F5 + num as u32).expect("num should be 0-9 only, resulting in valid full-width digit").to_string();
                num+=1;
                ret=char::from_u32(0xFF10 + num as u32).expect("num should be 0-9 only, resulting in valid full-width digit").to_string();
            },
            Tileset::Bamboos=>{     //索子
                num+=1;
                ret=char::from_u32(0xFF10 + num as u32).expect("num should be 0-9 only, resulting in valid full-width digit").to_string();
            },
            _=>{
                ret="　".to_string();
            }
        }
        ret
    }
}
struct Board{
    pais:Vec<Pai>,
    moves:u8,       //何手目か
    akadra:bool,    //赤ドラ(5の牌)
    seasons:bool,   //季節牌使用
    flowers:bool,   //花牌使用
    back:bool,      //後付け
    sanma:bool,     //3人麻雀(3-player Mahjong)
}
impl Board{
    fn create_board(&self)->Vec<Pai>{
        let mut idx:u8=0;
        let mut pais:Vec<Pai>=Vec::new();
        let mut p=Pai{
            es:"".to_string(),
            face:Tileset::Characters,
            settype:Tiletype::Suited, //数字牌
            idx:0,
            number:0,
            doralv:0,
            red:false,
            used:true,
        };
        //数字牌 0.萬子、1.筒子、2.索子
        for i in 0..3{
            for j in 0..9{  //0.=1、8.=9
                for k in 0..4{ //4セット
                    let reddra:bool=self.akadra;
                    let mut f:Tileset=Tileset::Characters;
                    match i{
                        0=>f=Tileset::Characters,
                        1=>f=Tileset::Dots,
                        2=>f=Tileset::Bamboos,
                        _=>f=Tileset::Characters,
                    }
                    let mut d=0;
                    let mut r=false;
                    if reddra && j==4 && k==0{ //5の赤ドラは数字牌種に1つだけ
                        d=1;
                        r=true;
                    }else{
                        d=0;
                        r=false;
                    }
                    let mut u=true;
                    if self.sanma{
                        if f==Tileset::Characters{
                            if 0<j && j<8{
                                u=false;
                            }
                        }
                    }
                    if u{
                        p.face=f;
                        p.settype=Tiletype::Suited; //数字牌
                        p.idx=idx;
                        p.number=j;
                        p.doralv=d;
                        p.red=r;
                        p.used=u;
                        p.make_es();
                        pais.push(p.clone());
                        idx+=1;
                    }
                }
            }
        }
        //四風牌 0.東、1.南、2.西、3.北
        for i in 0..4{
            for j in 0..3{ //各種4枚づつ
                let mut s=String::new();
                match i{
                    0=>s="東".to_string(),
                    1=>s="南".to_string(),
                    2=>s="西".to_string(),
                    3=>s="北".to_string(),
                    _=>s="　".to_string(),
                }
                p.es=s;
                p.face=Tileset::Winds;      //三元牌(白・発・中)
                p.settype=Tiletype::Honors; //文字牌
                p.idx=idx;
                p.number=i;
                p.doralv=0;
                p.red=false;
                p.used=true;
                p.make_es();
                pais.push(p.clone());
                idx+=1;
            }
            pais.push(p.clone());
            idx+=1;
        }
        // 三元牌 0.白、1.発 2.中
        for i in 0..3{
            for j in 0..3{ //各種4枚づつ
                let mut s=String::new();
                match i{
                    0=>s="白".to_string(),
                    1=>s="発".to_string(),
                    2=>s="中".to_string(),
                    _=>s="　".to_string(),
                }
                p.es=s;
                p.face=Tileset::Dragons;    //三元牌(白・発・中)
                p.settype=Tiletype::Honors; //文字牌
                p.idx=idx;
                p.number=i;
                p.doralv=0;
                p.red=false;
                p.used=true;
                p.make_es();
                pais.push(p.clone());
                idx+=1;
            }
            pais.push(p.clone());
            idx+=1;
        }
        // 季節牌 0.春、1.夏、2.秋、3.冬
        if self.seasons{
            for i in 0..4{
                let mut s=String::new();
                match i{
                    0=>s="春".to_string(),
                    1=>s="夏".to_string(),
                    2=>s="秋".to_string(),
                    3=>s="冬".to_string(),
                    _=>s="　".to_string(),
                }
                p.es=s;
                p.face=Tileset::Seasons;    //季節牌
                p.settype=Tiletype::Bonus;  //花牌
                p.idx=idx;
                p.number=i;
                p.doralv=0;
                p.red=false;
                p.used=true;
                p.make_es();
                pais.push(p.clone());
                idx+=1;
            }
        }
        
        //花牌(四君子　梅・蘭・菊・竹)
        if self.flowers{
            for i in 0..4{
                let mut s=String::new();
                match i{
                    0=>s="梅".to_string(),
                    1=>s="蘭".to_string(),
                    2=>s="菊".to_string(),
                    3=>s="竹".to_string(),
                    _=>s="　".to_string(),
                }
                p.es=s;
                p.face=Tileset::Flowers;    //花牌
                p.settype=Tiletype::Bonus;  //花牌
                p.idx=idx;
                p.number=i;
                p.doralv=0;
                p.red=false;
                p.used=true;
                p.make_es();
                pais.push(p.clone());
                idx+=1;
            }
        }
        
        pais
    }
}

fn main() {
    let mut bd=Board{
        pais:Vec::new(),
        moves:0,        //何手目か
        akadra:true,    //赤ドラ(5の牌)
        seasons:true,  //季節牌使用
        flowers:true,  //花牌使用
        back:true,      //後付け
        sanma:true,    //3人麻雀(3-player Mahjong)
    };
    let mut pais=bd.create_board();
    bd.pais=pais.clone();
    for i in bd.pais{
        print!("{}",i.es);
        //print!("{}{}",i.es,i.idx);
    }
    println!("");   
}