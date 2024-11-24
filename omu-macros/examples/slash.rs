use omu_macros::slash;

#[slash]
async fn world() {}

fn main() {
    println!("{}", __omu_world_metadata());
}
