mod room;
mod dungeon;
mod pos;
mod tiles;

fn main() {
    let mut dungeon = dungeon::new(50);
    dungeon.generate();

    let a = &dungeon.rooms[0].pos;
    let b = &dungeon.rooms[1].pos;
    let c = a.clone() + b.clone();
    println!("c = {:?}", c);
    let d = a.clone() - b.clone();
    println!("d = {:?}", d);
}
