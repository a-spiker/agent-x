use dioxus::prelude::*;
use crate::views::game::{types::*, utils::generate_cards_for_category};

/// Screen where players view their cards one by one
#[component]
pub fn CardViewScreen(
    current_player_index: usize,
    players: Signal<Vec<Player>>,
    mut cards: Signal<Vec<GameCard>>,
    mut imposter_index: Signal<usize>,
    mut game_screen: Signal<GameScreen>,
    mut current_category: Signal<Option<(String, String)>>,
    selected_category_index: Signal<Option<usize>>,
) -> Element {
    // Initialize cards for the round using the selected category
    use_effect(move || {
        let player_count = players().len();
        if cards().is_empty() && player_count > 0 {
            if let Some(cat_index) = selected_category_index() {
                let (new_cards, new_imposter, cat_name, cat_icon) = generate_cards_for_category(player_count, cat_index);
                cards.set(new_cards);
                imposter_index.set(new_imposter);
                current_category.set(Some((cat_name.clone(), cat_icon.clone())));
            }
        }
    });

    let mut card_revealed = use_signal(|| false);
    let player_list = players();
    let cards_list = cards();
    
    if current_player_index >= player_list.len() {
        return rsx! {
            div { class: "transition-screen",
                h2 { "All players have seen their cards!" }
                button {
                    class: "proceed-btn",
                    onclick: move |_| {
                        game_screen.set(GameScreen::Voting);
                    },
                    "Proceed to Voting"
                }
            }
        };
    }
    
    // Check if cards are initialized
    if cards_list.is_empty() || current_player_index >= cards_list.len() {
        return rsx! {
            div { class: "loading-screen",
                p { "Preparing cards..." }
            }
        };
    }

    let current_player = &player_list[current_player_index];
    let current_card = &cards_list[current_player_index];

    rsx! {
        div { class: "card-view-screen",
            if !card_revealed() {
                div { class: "player-ready-screen",
                    h2 { "Pass device to:" }
                    h1 { class: "player-name", "{current_player.name}" }
                    p { class: "instruction", "⚠️ Make sure other players can't see the screen!" }
                        button {
                            class: "reveal-btn",
                            onclick: move |_| {
                                card_revealed.set(true);
                            },
                            "Reveal My Card"
                        }
                }
            } else {
                div { class: "card-revealed-screen",
                    h2 { "{current_player.name}'s Card" }
                    
                    div { 
                        class: if current_card.card_type == CardType::Imposter {
                            "game-card imposter-card"
                        } else {
                            "game-card normal-card"
                        },
                        div { class: "card-word",
                            "{current_card.word}"
                        }
                        div { class: "card-type-hint",
                            if current_card.card_type == CardType::Imposter {
                                "🎭 You are the IMPOSTER!"
                            } else {
                                "👥 You are a civilian"
                            }
                        }
                    }
                    
                    p { class: "card-instruction",
                        if current_card.card_type == CardType::Imposter {
                            "Try to blend in! Don't let others know you have the odd word."
                        } else {
                            "Find the player with the different word!"
                        }
                    }
                    
                    button {
                        class: "next-btn",
                        onclick: move |_| {
                            card_revealed.set(false);
                            game_screen.set(GameScreen::CardView {
                                current_player_index: current_player_index + 1
                            });
                        },
                        "Next Player"
                    }
                }
            }
        }
    }
}