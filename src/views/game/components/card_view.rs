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
    hide_imposter_identity: Signal<bool>,
    mut current_round_words: Signal<Option<(String, String)>>,
    mut starting_player_index: Signal<usize>,
) -> Element {
    // Initialize cards for the round using the selected category
    use_effect(move || {
        let player_count = players().len();
        if cards().is_empty() && player_count > 0 {
            if let Some(cat_index) = selected_category_index() {
                let (new_cards, new_imposter, cat_name, cat_icon) = generate_cards_for_category(player_count, cat_index);
                
                // Store the words for this round (civilian word, imposter word)
                if !new_cards.is_empty() {
                    let civilian_word = new_cards.iter()
                        .find(|c| c.card_type == CardType::Normal)
                        .map(|c| c.word.clone())
                        .unwrap_or_default();
                    let imposter_word = new_cards.iter()
                        .find(|c| c.card_type == CardType::Imposter)
                        .map(|c| c.word.clone())
                        .unwrap_or_default();
                    current_round_words.set(Some((civilian_word, imposter_word)));
                }
                
                cards.set(new_cards);
                imposter_index.set(new_imposter);
                current_category.set(Some((cat_name.clone(), cat_icon.clone())));
            }
        }
    });

    let player_list = players();
    let cards_list = cards();
    
    // Calculate rotated player order
    let total_players = player_list.len();
    let start_idx = starting_player_index() % total_players;
    let actual_player_index = (start_idx + current_player_index) % total_players;
    
    if current_player_index >= player_list.len() {
        return rsx! {
            div { class: "transition-screen",
                h2 { "All players have seen their cards!" }
                button {
                    class: "proceed-btn",
                    onclick: move |_| {
                        game_screen.set(GameScreen::Voting);
                    },
                    "Proceed to Discussion"
                }
            }
        };
    }
    
    // Check if cards are initialized
    if cards_list.is_empty() || actual_player_index >= cards_list.len() {
        return rsx! {
            div { class: "loading-screen",
                p { "Preparing cards..." }
            }
        };
    }

    let mut card_revealed = use_signal(|| false);
    let current_player = &player_list[actual_player_index];
    let current_card = &cards_list[actual_player_index];
    
    // Determine what to show based on hard mode
    let is_imposter = current_card.card_type == CardType::Imposter;
    let should_hide_imposter = hide_imposter_identity() && is_imposter;
    
    // If hard mode is on and this is the imposter, show civilian word instead
    let displayed_word = if should_hide_imposter {
        // Get the civilian word
        cards_list.iter()
            .find(|c| c.card_type == CardType::Normal)
            .map(|c| c.word.clone())
            .unwrap_or_else(|| current_card.word.clone())
    } else {
        current_card.word.clone()
    };

    rsx! {
        div { class: "card-view-screen",
            if !card_revealed() {
                div { class: "player-ready-screen",
                    h2 { "Pass device to:" }
                    h1 { class: "player-name", "{current_player.name}" }
                    p { class: "instruction", "⚠️ Make sure other players can't see the screen!" }
                    
                    div { class: "ready-screen-buttons",
                        // Show back button only for first player
                        if current_player_index == 0 {
                            button {
                                class: "back-btn",
                                onclick: move |_| {
                                    if let Some((cat_name, cat_icon)) = current_category() {
                                        game_screen.set(GameScreen::CategoryReveal {
                                            category_name: cat_name,
                                            category_icon: cat_icon,
                                        });
                                    }
                                },
                                "← Back"
                            }
                        }
                        
                        button {
                            class: "reveal-btn",
                            onclick: move |_| {
                                card_revealed.set(true);
                            },
                            "Reveal My Card"
                        }
                    }
                }
            } else {
                div { class: "card-revealed-screen",
                    h2 { "{current_player.name}'s Card" }
                    
                    div { 
                        class: if should_hide_imposter {
                            "game-card normal-card"
                        } else if is_imposter {
                            "game-card imposter-card"
                        } else {
                            "game-card normal-card"
                        },
                        div { class: "card-word",
                            "{displayed_word}"
                        }
                        div { class: "card-type-hint",
                            if should_hide_imposter {
                                "👥 You are a civilian"
                            } else if is_imposter {
                                "🎭 You are the IMPOSTER!"
                            } else {
                                "👥 You are a civilian"
                            }
                        }
                    }
                    
                    p { class: "card-instruction",
                        if should_hide_imposter {
                            "Find the player with the different word!"
                        } else if is_imposter {
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