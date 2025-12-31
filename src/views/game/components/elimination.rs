use dioxus::prelude::*;
use crate::views::game::types::*;

/// Screen showing elimination results
#[component]
pub fn EliminationScreen(
    mut players: Signal<Vec<Player>>,
    eliminated_index: usize,
    was_imposter: bool,
    mut game_screen: Signal<GameScreen>,
    mut round_number: Signal<i32>,
    mut cards: Signal<Vec<GameCard>>,
    imposter_index: Signal<usize>,
) -> Element {
    let player_list = players();
    let eliminated_player = &player_list[eliminated_index];
    let active_count = player_list.iter().filter(|p| !p.is_eliminated).count();
    
    rsx! {
        div { class: "elimination-screen",
            h1 { "🗳️ Player Eliminated" }
            
            div { class: "elimination-result",
                p { class: "eliminated-player",
                    "{eliminated_player.name} has been evicted!"
                }
                p { class: "players-remaining",
                    "{active_count - 1} players remaining"
                }
            }
            
            div { class: "action-buttons",
                button {
                    class: "continue-btn",
                    onclick: move |_| {
                        let mut updated_players = players();
                        // Eliminate the player
                        updated_players[eliminated_index].is_eliminated = true;
                        
                        // Check if imposter was eliminated
                        if was_imposter {
                            // Imposter found - civilians win!
                            // ALL civilians get points, even if they were eliminated before
                            for (i, player) in updated_players.iter_mut().enumerate() {
                                if i != imposter_index() {
                                    player.score += 10;
                                }
                            }
                            players.set(updated_players);
                            game_screen.set(GameScreen::RoundEnd { 
                                imposter_found: true,
                                game_over: true 
                            });
                        } else {
                            // Check if only 2 players remain
                            let remaining_count = updated_players.iter()
                                .filter(|p| !p.is_eliminated)
                                .count();
                            
                            players.set(updated_players);
                            
                            if remaining_count <= 2 {
                                // Imposter wins!
                                let mut final_players = players();
                                final_players[imposter_index()].score += 20;
                                players.set(final_players);
                                game_screen.set(GameScreen::RoundEnd { 
                                    imposter_found: false,
                                    game_over: true 
                                });
                            } else {
                                // Continue to next voting round
                                round_number.set(round_number() + 1);
                                game_screen.set(GameScreen::Voting);
                            }
                        }
                    },
                    "Continue"
                }
            }
        }
    }
}