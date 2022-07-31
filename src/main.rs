mod lib;

fn main() {
    println!("Hello, world!");
    let mut n = lib::Player::new(String::from("Lucas"), vec![2,4,6]);
    n.add_deck(2);
    println!("Name: {}, Deck {:?}, Sum: {}", n.name, n.deck, n.sum_of_deck)
}
