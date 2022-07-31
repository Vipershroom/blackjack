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

fn game_logic(player: Player) {
    loop {}
}
