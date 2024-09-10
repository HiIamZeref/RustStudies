use rand::{thread_rng, seq::SliceRandom};


#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

impl Deck {
    fn new() -> Self { // This is a associated function: it is called on the type itself, not on an instance of the type
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let values = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];

        let mut cards: Vec<String> = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards: cards } // Deleting the semicolon here will return the Deck instance (Implicit return)

    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> { //Num types: i: signed integer, u: unsigned integer, f: floating point number
        self.cards.split_off(self.cards.len() - num_cards)
    }

    
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    let hand = deck.deal(5);
    
    println!("Here is the hand: {:#?}", hand);


}
