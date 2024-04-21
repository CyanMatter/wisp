#[derive(Clone, PartialEq, Eq)]
struct Generator {
    value: Option<usize>,
}

impl Generator {
    fn new() -> Generator {
        Generator { value: Some(0) }
    }
}

impl Iterator for Generator {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.value;
        self.value = self.value?.checked_add(1);
        current
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct CardList {
    id_gen: Generator,
    cards: Vec<Card>,
}

impl CardList {
    pub fn new() -> CardList {
        CardList {
            id_gen: Generator::new(),
            cards: Vec::<Card>::new(),
        }
    }

    pub fn add_card(&mut self, response: String) {
        self.cards.push(Card {
            id: self.id_gen.next().expect("Ran out of unique identifiers for a new card"),
            response,
        });
    }

    pub fn remove_card_with_id(&mut self, id: usize) {
        if let Some(index) = self.cards.iter().position(|card| card.id == id) {
            self.cards.remove(index);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Card> {
        <Vec<Card> as Clone>::clone(&self.cards).into_iter().rev()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Card {
    id: usize,
    response: String,
}

impl Card {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn response(&self) -> String {
        self.response.clone()
    }
}