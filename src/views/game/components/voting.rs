use dioxus::prelude::*;
use crate::views::game::types::*;

/// Voting screen where all players collectively decide who to evict
#[component]
pub fn VotingScreen(
    mut players: Signal<Vec<Player>>,
    mut game_screen: Signal<GameScreen>,
    imposter_index: Signal<usize>,
    mut cards: Signal<Vec<GameCard>>,
    current_category: Signal<Option<(String, String)>>,
    starting_player_index: Signal<usize>,
) -> Element {
    let player_list = players();
    let mut show_restart_confirmation = use_signal(|| false);
    
    // Only show non-eliminated players
    let active_indices: Vec<usize> = player_list.iter()
        .enumerate()
        .filter(|(_, p)| !p.is_eliminated)
        .map(|(i, _)| i)
        .collect();
    
    // Rotate voting order based on starting_player_index
    let total_players = player_list.len();
    let start_idx = starting_player_index() % total_players;
    
    // Create rotated player list for voting display
    let mut rotated_player_data: Vec<(usize, String)> = Vec::new();
    for i in 0..active_indices.len() {
        let idx = active_indices[(start_idx + i) % active_indices.len()];
        rotated_player_data.push((idx, player_list[idx].name.clone()));
    }
    
    rsx! {
        div { class: "voting-screen",
            // Restart confirmation dialog
            if show_restart_confirmation() {
                div { class: "confirmation-overlay",
                    div { class: "confirmation-dialog",
                        h2 { "⚠️ Restart Round?" }
                        p { "This will reset the current round and go back to category selection. Progress will be lost." }
                        div { class: "confirmation-buttons",
                            button {
                                class: "confirm-yes-btn",
                                onclick: move |_| {
                                    // Reset round state
                                    cards.set(Vec::new());
                                    show_restart_confirmation.set(false);
                                    game_screen.set(GameScreen::CategorySelection);
                                },
                                "Yes, Restart Round"
                            }
                            button {
                                class: "confirm-no-btn",
                                onclick: move |_| {
                                    show_restart_confirmation.set(false);
                                },
                                "Cancel"
                            }
                        }
                    }
                }
            }
            
            h1 { "🗳️ Discussion & Voting" }
            
            div { class: "voting-instructions",
                p { "💬 Discussion order (everyone explains their word):" }
                div { class: "discussion-order",
                    for (order_num, (_idx, player_name)) in rotated_player_data.iter().enumerate() {
                        span { 
                            key: "{order_num}",
                            class: "discussion-order-item",
                            "{order_num + 1}. {player_name}"
                        }
                    }
                }
                p { class: "hint", "After discussion, tap on the player card you all agreed to evict." }
            }
            
            div { class: "players-voting-list",
                for &(player_idx, ref player_name) in rotated_player_data.iter() {
                    div { class: "player-voting-card",
                        div { class: "player-info",
                            h3 { "{player_name}" }
                        }
                        button {
                            class: "evict-btn",
                            onclick: move |_| {
                                let was_imposter = player_idx == imposter_index();
                                game_screen.set(GameScreen::Elimination { 
                                    eliminated_index: player_idx,
                                    was_imposter 
                                });
                            },
                            "Evict"
                        }
                    }
                }
            }
            
            div { class: "voting-footer",
                button {
                    class: "restart-round-btn",
                    onclick: move |_| {
                        show_restart_confirmation.set(true);
                    },
                    "🔄 Restart Round"
                }
            }
        }
    }
}