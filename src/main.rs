mod nintai_game;
mod nintai_play;

fn main() {
    let g = nintai_game::GameArea::new(nintai_game::Colors::Yellow);
    println!("The GameArea is: {:?}", g);
}
