mod lib;

fn main() {
    println!("Welcome to blackjack!\nPress enter to begin");
    let player = lib::Player::new();
    println!("Would you like to play against an AI? Y/n");
    let res = lib::input();
    match res.to_lowercase().as_str() {
        "y" => lib::multiplayer_logic(player),
        _ => lib::singleplayer_logic(player),
    }
}
