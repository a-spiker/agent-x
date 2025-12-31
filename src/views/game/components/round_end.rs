use dioxus::prelude::*;
use crate::views::game::types::*;

/// Screen showing round results
#[component]
pub fn RoundEndScreen(
    mut players: Signal<Vec<Player>>,
    imposter_found: bool,
    game_over: bool,
    mut game_screen: Signal<GameScreen>,
    mut round_number: Signal<i32>,
    mut cards: Signal<Vec<GameCard>>,
    mut imposter_index: Signal<usize>,
) -> Element {
    let player_list = players();
    let imposter_name = &player_list[imposter_index()].name;
    let mut show_confirmation = use_signal(|| false);

    rsx! {
        div { class: "round-end-screen",
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
                                    // Clear all game state for a fresh start
                                    cards.set(Vec::new());
                                    imposter_index.set(0);
                                    round_number.set(1);
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
            
            h1 {
                if imposter_found {
                    "✅ Civilians Win!"
                } else {
                    "😈 Imposter Wins!"
                }
            }
            
            div { class: "round-result",
                p { class: "imposter-reveal",
                    "The imposter was: {imposter_name}"
                }
                
                if imposter_found {
                    p { class: "result-message",
                        "🎉 All civilians get 10 points!"
                    }
                } else {
                    p { class: "result-message",
                        "😈 The imposter gets 20 points!"
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
                    "▶️ Next Round"
                }
                
                button {
                    class: "view-scores-btn",
                    onclick: move |_| {
                        game_screen.set(GameScreen::GameScore);
                    },
                    "🏆 View Scores"
                }
                
                button {
                    class: "new-game-btn",
                    onclick: move |_| {
                        show_confirmation.set(true);
                    },
                    "🔄 New Game"
                }
            }
        }
    }
}