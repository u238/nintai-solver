mod nintai_game;
mod nintai_play;

fn main() {
    let mut g = nintai_game::GameArea::new(nintai_game::Colors::Yellow);
    let c = g.get_piece_candidates();
    // println!("The GameArea is: {:?}", g);
    println!("The candidates count is: {:?}", c.iter().count());
    println!("The candidates is: {:?}", c);

    // g.pieces[0].position = 1;
    // g.next_piece_index = 2;

    // println!("The 3%2 is: {:?}", (3 % 2));
    // println!("The 0+2%3 is: {:?}", ((0 + 2) % 3));
    // println!("The 1+2%3 is: {:?}", ((1 + 2) % 3));
    // println!("The 2+2%3 is: {:?}", ((2 + 2) % 3));
}
