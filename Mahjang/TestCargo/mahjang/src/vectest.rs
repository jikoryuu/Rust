use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone)]
pub struct Pai {
    pub name: String, //牌の名前
}

pub fn vectest() {
    let mut vec: Vec<Pai> = Vec::new();
    for i in 0..10 {
        vec.push(Pai { name: format!("牌{}", i) });
    }
    vec.shuffle(&mut thread_rng());
    for i in &vec {
         print!("{},", i.name);
    }
    println!();
    let mut vec2: Vec<Pai> = Vec::new();
    for i in 6..10 {
        vec2.push(vec[i].clone());
    }    
    for i in 0..6 {
        vec2.push(vec[i].clone());
    }
    for i in &vec2 {
         print!("{},", i.name);
    }
    println!();

}