mod lib;

fn main() {
    println!("Welcome to blackjack!\nPress enter to begin");
    let player = lib::Player::new();
    lib::game_logic(player);
}
