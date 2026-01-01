use dioxus::prelude::*;
use crate::views::game::types::{Player, GameScreen};
use crate::views::game::utils::get_random_starting_index;

#[component]
pub fn SetupScreen(
    mut player_count_input: Signal<String>,
    mut player_names: Signal<Vec<String>>,
    mut players: Signal<Vec<Player>>,
    mut game_screen: Signal<GameScreen>,
    mut round_number: Signal<i32>,
    mut starting_player_index: Signal<usize>,
) -> Element {
    let player_count = player_count_input().parse::<usize>().unwrap_or(3).max(3).min(10);
    
    // Initialize player names if needed - ensure this happens before rendering
    let mut current_names = player_names();
    if current_names.len() != player_count {
        let mut names = vec![String::new(); player_count];
        for i in 0..player_count.min(current_names.len()) {
            names[i] = current_names[i].clone();
        }
        current_names = names.clone();
        player_names.set(names);
    }

    rsx! {
        div { class: "setup-screen",
            div { class: "setup-header",
                h1 { "🎮 Agent-X" }
                p { class: "subtitle", "The Social Deduction Game" }
            }
            
            div { class: "player-count-section",
                label { 
                    "👥 Number of Players"
                    span { class: "hint", "(minimum 3)" }
                }
                input {
                    r#type: "number",
                    min: "3",
                    max: "10",
                    value: "{player_count_input}",
                    oninput: move |e| {
                        player_count_input.set(e.value());
                    }
                }
            }
            
            div { class: "player-names-section",
                h2 { "✏️ Player Names" }
                div { class: "player-inputs-grid",
                    for i in 0..player_count {
                        div { class: "player-input",
                            span { class: "player-number", "{i + 1}" }
                            input {
                                r#type: "text",
                                placeholder: "Enter name...",
                                value: "{current_names.get(i).cloned().unwrap_or_default()}",
                                oninput: move |e| {
                                    let mut names = player_names();
                                    // Ensure the vector is large enough
                                    while names.len() <= i {
                                        names.push(String::new());
                                    }
                                    names[i] = e.value();
                                    player_names.set(names);
                                }
                            }
                        }
                    }
                }
            }
            
            button {
                class: "start-game-btn",
                onclick: move |_| {
                    let names = player_names();
                    if names.iter().all(|n| !n.trim().is_empty()) {
                        let new_players: Vec<Player> = names.iter().map(|name| Player {
                            name: name.clone(),
                            score: 0,
                            is_eliminated: false,
                        }).collect();
                        players.set(new_players.clone());
                        round_number.set(1);
                        
                        // Randomize starting player for new game
                        starting_player_index.set(get_random_starting_index(new_players.len()));
                        
                        game_screen.set(GameScreen::CategorySelection);
                    }
                },
                "🚀 Start Game"
            }
        }
    }
}

