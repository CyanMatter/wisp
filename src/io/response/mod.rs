mod card;
mod clipboard;

use leptos::*;

#[component]
pub fn ResponseList(output: ReadSignal<String>) -> impl IntoView {
    let (cards, set_cards) = create_signal(card::CardList::new());

    view! {
        <div>
            <ul>
                <li>
                    <Target output=output set_cards=set_cards/>
                </li>
                <For
                    each=move || cards.get().iter()
                    key=|card| card.id()
                    children=move |card: card::Card| {
                        view! { <CardElement card=card set_cards=set_cards/> }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn Target(output: ReadSignal<String>, set_cards: WriteSignal<card::CardList>) -> impl IntoView {
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
fn CardElement(card: card::Card, set_cards: WriteSignal<card::CardList>) -> impl IntoView {
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
