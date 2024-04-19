mod clipboard;

use leptos::*;

#[component]
pub fn ResponseList(output: ReadSignal<String>) -> impl IntoView {
    let (cards, set_cards) = create_signal(cards::CardList::new());

    view! {
        <div>
            <ul>
                <li>
                    <Target output=output set_cards=set_cards/>
                </li>
                <For
                    each=move || cards.get().iter()
                    key=|card| card.id()
                    children=move |card: cards::Card| {
                        view! { <CardElement card=card set_cards=set_cards/> }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn Target(output: ReadSignal<String>, set_cards: WriteSignal<cards::CardList>) -> impl IntoView {
    view! {
        <div id="target" class="response">
            <span>{output}</span>

            // A button that, when pressed, adds a new card to the top of the responses with the current response string
            <button on:click=move |_| {
                set_cards
                    .update(move |cards| {
                        cards.add_card(output.get_untracked());
                    });
            }>
                "Pin"
            </button>

            <clipboard::CopyButton text=output.get_untracked()/>
        </div>
    }
}

#[component]
fn CardElement(card: cards::Card, set_cards: WriteSignal<cards::CardList>) -> impl IntoView {
    let id = card.id();

    view! {
        <li>
            <div class="response">
                <span>{id} {card.response()}</span>

                // A button that, when clicked, removes this card from the list
                <button on:click=move |_| {
                    set_cards
                        .update(move |cards| {
                            cards.remove_card_with_id(id);
                        });
                }>
                    "Unpin"
                </button>

                <clipboard::CopyButton text=card.response()/>
            </div>
        </li>
    }
}

mod cards {
    #[derive(Clone, PartialEq, Eq)]    pub struct CardList {
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
}
