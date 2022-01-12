mod nintai_game;

fn main() {
    let g = nintai_game::GameArea::new(nintai_game::Colors::Yellow);
    format!("The GameArea is: {:?}", g);
}
