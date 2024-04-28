mod card;
mod clipboard;

use leptos::*;

stylance::import_crate_style!(style, "src/io/response/response.module.scss");

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
                        view! { <PinnedCard card=card set_cards=set_cards/> }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn Target(output: ReadSignal<String>, set_cards: WriteSignal<card::CardList>) -> impl IntoView {
    view! {
        <div id="target" class=style::response>
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

            // A button that copies the response to the clipboard
            <clipboard::CopyResponseFromSignal text=output/>

            // A button that copies a shareable url to the clipboard
            <clipboard::CopyShareableURLFromSignal text=output/>
        </div>
    }
}

#[component]
fn PinnedCard(card: card::Card, set_cards: WriteSignal<card::CardList>) -> impl IntoView {
    let id = card.id();

    view! {
        <li id=format!("pinned-card-{id}") class=style::response>
            <div class="functions">
                // A button that, when clicked, removes this card from the list
                <button on:click=move |_| {
                    set_cards
                        .update(move |cards| {
                            cards.remove_card_with_id(id);
                        });
                }>
                    "Unpin"
                </button>

                // A button that copies the response to the clipboard
                <clipboard::CopyResponse text=card.response()/>

                // A button that copies a shareable url to the clipboard
                <clipboard::CopyShareableURL text=card.response()/>              
            </div>
            <div>
                // A division for the message with the invisible payload
                <span class="message">
                    { card.response() }
                </span>

                <hr/>

                // A division for the message's visible projection
                <span class="projection">
                    "TODO: message's visible projection"
                </span>
            </div>
        </li>
    }
}
