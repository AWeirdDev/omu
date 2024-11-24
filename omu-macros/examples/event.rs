use omu_macros::event;

#[event(ready)]
async fn on_ready() {}
fn main() {
    println!("{}", __omu_on_ready_metadata());
}
