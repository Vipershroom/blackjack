use rand::Rng;

pub fn input() -> String {
    let mut buf = String::new();

    std::io::stdin().read_line(&mut buf).unwrap();

    String::from(buf.trim())
}

pub struct Player {
    pub name: String,
    pub deck: Vec<i32>,
    pub sum_of_deck: i32,
}

#[allow(clippy::new_without_default)]
impl Player {
    pub fn new() -> Player {
        println!("What is your name?");
        let name = input();
        let deck: Vec<i32> = vec![];
        Player {
            name: (name),
            deck: (deck),
            sum_of_deck: (0),
        }
    }
    pub fn sum(&mut self) {
        let mut sum = 0;
        for i in &self.deck {
            sum += i
        }
        self.sum_of_deck = sum
    }
    pub fn add_deck(&mut self, num: i32) {
        self.deck.push(num);
    }
    // Method for generating AI struct
    pub fn new_ai() -> Player {
        let name = String::from("AI");
        let deck: Vec<i32> = vec![];
        Player {
            name: (name),
            deck: (deck),
            sum_of_deck: (0),
        }
    }
}

fn draw_card() -> i32 {
    rand::thread_rng().gen_range(1..13)
}

pub fn singleplayer_logic(mut player: Player) {
    player.add_deck(draw_card());
    player.add_deck(draw_card());
    println!("The game has begun!");
    loop {
        player.sum();
        match player.sum_of_deck {
            deck if deck == 21 => {
                println!("Congrats! You reached 21");
                println!("Deck: {:?}, Sum: {}", player.deck, player.sum_of_deck);
                input();
                std::process::exit(0);
            }
            deck if deck > 21 => {
                println!("You lose, you reached over 21");
                println!("Deck: {:?}, Sum: {}", player.deck, player.sum_of_deck);
                input();
                std::process::exit(0);
            }
            _ => (),
        }

        println!(
            "Your deck consists of {:?}, Sum: {}",
            player.deck, player.sum_of_deck
        );

        println!("Would you like to hit? Y/n");
        let res = input();

        if res.to_lowercase() == "y" {
            let mut card = draw_card();
            if card == 1 || card == 11 {
                println!("You got an ace! Would you like a 1 or an 11?");
                let res = input();
                if res.as_str() == "11" {
                    card = 11
                } else {
                    card = 1;
                }
            }
            player.add_deck(card)
        }
    }
}

pub fn multiplayer_logic(mut player: Player) {
    let mut ai = Player::new_ai();
    player.add_deck(draw_card());
    player.add_deck(draw_card());
    ai.add_deck(draw_card());
    ai.add_deck(draw_card());
    println!("The game has begun");
}
