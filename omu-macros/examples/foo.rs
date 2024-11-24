use omu_macros::slash;

#[slash]
fn world() {
    println!("{}", __omu_world_metadata())
}

fn main() {
    world();
}