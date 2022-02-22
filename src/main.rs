const FILE:&str = include_str!("../Meteorite_Landings.csv");

fn main() {
    for i in FILE.lines() {
        println!("{}", i);
        println!("-------");
    }
}