use dioxus::prelude::*;
use crate::views::game::types::*;

/// Voting screen where all players collectively decide who to evict
#[component]
pub fn VotingScreen(
    mut players: Signal<Vec<Player>>,
    mut game_screen: Signal<GameScreen>,
    imposter_index: Signal<usize>,
) -> Element {
    let player_list = players();
    
    // Only show non-eliminated players
    let active_indices: Vec<usize> = player_list.iter()
        .enumerate()
        .filter(|(_, p)| !p.is_eliminated)
        .map(|(i, _)| i)
        .collect();
    
    // Pre-compute player data for the loop
    let player_data: Vec<(usize, String)> = active_indices.iter()
        .map(|&idx| (idx, player_list[idx].name.clone()))
        .collect();
    
    rsx! {
        div { class: "voting-screen",
            h1 { "🗳️ Group Decision Time!" }
            
            div { class: "voting-instructions",
                p { "Discuss among all players and decide who to evict." }
                p { class: "hint", "Tap on the player card you all agreed to evict." }
            }
            
            div { class: "players-voting-list",
                for &(player_idx, ref player_name) in player_data.iter() {
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
        }
    }
}