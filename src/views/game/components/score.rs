use dioxus::prelude::*;
use crate::views::game::types::*;

/// Screen showing all player scores
#[component]
pub fn GameScoreScreen(
    players: Signal<Vec<Player>>,
    round_number: Signal<i32>,
    mut game_screen: Signal<GameScreen>,
    mut cards: Signal<Vec<GameCard>>,
    mut imposter_index: Signal<usize>,
) -> Element {
    let mut sorted_players = players();
    sorted_players.sort_by(|a, b| b.score.cmp(&a.score));
    let mut show_confirmation = use_signal(|| false);

    rsx! {
        div { class: "score-screen",
            // Confirmation dialog
            if show_confirmation() {
                div { class: "confirmation-overlay",
                    div { class: "confirmation-dialog",
                        h2 { "⚠️ Start New Game?" }
                        p { "All player scores will be reset. Are you sure?" }
                        div { class: "confirmation-buttons",
                            button {
                                class: "confirm-yes-btn",
                                onclick: move |_| {
                                    // Clear all game state for a completely fresh start
                                    cards.set(Vec::new());
                                    imposter_index.set(0);
                                    show_confirmation.set(false);
                                    game_screen.set(GameScreen::Setup);
                                },
                                "Yes, Start New Game"
                            }
                            button {
                                class: "confirm-no-btn",
                                onclick: move |_| {
                                    show_confirmation.set(false);
                                },
                                "Cancel"
                            }
                        }
                    }
                }
            }
            
            h1 { "🏆 Scoreboard" }
            p { class: "round-info", "After Round {round_number()}" }
            
            div { class: "scoreboard",
                for (rank, player) in sorted_players.iter().enumerate() {
                    div { 
                        class: if rank == 0 { "score-card winner" } else { "score-card" },
                        div { class: "rank", "#{rank + 1}" }
                        div { class: "player-score-info",
                            h3 { "{player.name}" }
                            p { class: "score", "{player.score} points" }
                        }
                        if rank == 0 {
                            span { class: "winner-badge", "👑" }
                        }
                    }
                }
            }
            
            div { class: "action-buttons",
                button {
                    class: "next-round-btn",
                    onclick: move |_| {
                        // Reset all player states for new round
                        let mut updated_players = players();
                        for player in updated_players.iter_mut() {
                            player.is_eliminated = false; // Reset eliminations for new round
                        }
                        players.set(updated_players);
                        cards.set(Vec::new());
                        round_number.set(round_number() + 1);
                        game_screen.set(GameScreen::CategorySelection);
                    },
                    "Play Next Round"
                }
                
                button {
                    class: "new-game-btn",
                    onclick: move |_| {
                        show_confirmation.set(true);
                    },
                    "New Game"
                }
            }
        }
    }
}