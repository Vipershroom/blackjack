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
}

fn draw_card() -> i32 {
    rand::thread_rng().gen_range(1..13)
}

pub fn game_logic(mut player: Player) {
    player.add_deck(draw_card());
    player.add_deck(draw_card());
    println!("The game has begun!");
    loop {
        player.sum();
        match player.sum_of_deck {
            deck if deck == 21 => {
                println!("Congrats! You reached 21");
                println!("Deck: {:?}, Sum: {}", player.deck, player.sum_of_deck);
                std::process::exit(0);
            }
            deck if deck > 21 => {
                println!("You lose, you reached over 21");
                println!("Deck: {:?}, Sum: {}", player.deck, player.sum_of_deck);
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
            player.add_deck(draw_card())
        }
    }
}
