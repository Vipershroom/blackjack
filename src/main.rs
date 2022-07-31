mod lib;

fn main() {
    println!("Welcome to blackjack!\nPress enter to begin");
    lib::input();
    let player = lib::Player::new();
}
