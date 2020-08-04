mod ecs;
fn main() {
    println!("Aloha World!");
    let mut x = ecs::EntityAllocator::new();

    for _ in 0..5 {
        x.new_entity();
    }

    println!("{:#?}", x.entities);
}
