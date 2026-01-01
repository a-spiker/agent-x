// Game module structure
mod types;
mod utils;
mod persistence;
mod components;

// Re-export types for use in components
pub use types::*;
pub use persistence::*;
// utils is used internally by components

use dioxus::prelude::*;
use components::*;

const _GAME_CSS: Asset = asset!("/assets/styling/game.css");

/// Main Game component
#[component]
pub fn Game() -> Element {
    // Initialize game state - load from localStorage if available
    let mut session_id = use_signal(|| String::new());
    let mut game_screen = use_signal(|| GameScreen::Setup);
    let mut players = use_signal(|| Vec::<Player>::new());
    let mut player_count_input = use_signal(|| String::from("3"));
    let mut player_names = use_signal(|| Vec::<String>::new());
    let mut round_number = use_signal(|| 1);
    let mut cards = use_signal(|| Vec::<GameCard>::new());
    let mut imposter_index = use_signal(|| 0usize);
    let mut current_category = use_signal(|| None::<(String, String)>);
    let mut selected_category_index = use_signal(|| None::<usize>);
    let mut hide_imposter_identity = use_signal(|| false);
    let mut current_round_words = use_signal(|| None::<(String, String)>);
    let mut starting_player_index = use_signal(|| 0usize);
    let mut initialized = use_signal(|| false);
    
    // Initialize once on mount
    use_effect(move || {
        if !initialized() {
            // Try to load existing session
            let sid = load_session_id().unwrap_or_else(|| {
                let id = generate_session_id();
                save_session_id(&id);
                id
            });
            
            session_id.set(sid.clone());
            
            // Try to load saved game state for this session
            if let Some(saved_state) = load_game_state(&sid) {
                game_screen.set(saved_state.game_screen);
                players.set(saved_state.players);
                player_count_input.set(saved_state.player_count_input);
                player_names.set(saved_state.player_names);
                round_number.set(saved_state.round_number);
                cards.set(saved_state.cards);
                imposter_index.set(saved_state.imposter_index);
                current_category.set(saved_state.current_category);
                selected_category_index.set(saved_state.selected_category_index);
                hide_imposter_identity.set(saved_state.hide_imposter_identity);
                current_round_words.set(saved_state.current_round_words);
                starting_player_index.set(saved_state.starting_player_index);
            }
            
            initialized.set(true);
        }
    });
    
    // Auto-save game state whenever it changes (but only after initialization)
    use_effect(move || {
        if initialized() && !session_id().is_empty() {
            let state = GameState {
                session_id: session_id(),
                game_screen: game_screen(),
                players: players(),
                player_count_input: player_count_input(),
                player_names: player_names(),
                round_number: round_number(),
                cards: cards(),
                imposter_index: imposter_index(),
                current_category: current_category(),
                selected_category_index: selected_category_index(),
                hide_imposter_identity: hide_imposter_identity(),
                current_round_words: current_round_words(),
                starting_player_index: starting_player_index(),
            };
            save_game_state(&state);
        }
    });
    
    rsx! {
        document::Stylesheet { href: _GAME_CSS }
        div { class: "game-container",
            match game_screen() {
                GameScreen::Setup => rsx! {
                    SetupScreen {
                        player_count_input,
                        player_names,
                        players,
                        game_screen,
                        round_number,
                        starting_player_index,
                    }
                },
                GameScreen::CategorySelection => rsx! {
                    CategorySelectionScreen {
                        game_screen,
                        selected_category_index,
                    }
                },
                GameScreen::CategoryReveal { category_name, category_icon } => rsx! {
                    CategoryRevealScreen {
                        category_name,
                        category_icon,
                        game_screen,
                        hide_imposter_identity,
                    }
                },
                GameScreen::CardView { current_player_index } => rsx! {
                    CardViewScreen {
                        current_player_index,
                        players,
                        cards,
                        imposter_index,
                        game_screen,
                        current_category,
                        selected_category_index,
                        hide_imposter_identity,
                        current_round_words,
                        starting_player_index,
                    }
                },
                GameScreen::Voting => rsx! {
                    VotingScreen {
                        players,
                        game_screen,
                        imposter_index,
                        cards,
                        current_category,
                        starting_player_index,
                    }
                },
                GameScreen::Elimination { eliminated_index, was_imposter } => rsx! {
                    EliminationScreen {
                        players,
                        eliminated_index,
                        was_imposter,
                        game_screen,
                        round_number,
                        cards,
                        imposter_index,
                    }
                },
                GameScreen::RoundEnd { imposter_found, game_over } => rsx! {
                    RoundEndScreen {
                        players,
                        imposter_found,
                        game_over,
                        game_screen,
                        round_number,
                        cards,
                        imposter_index,
                        current_round_words,
                        starting_player_index,
                    }
                },
                GameScreen::GameScore => rsx! {
                    GameScoreScreen {
                        players,
                        round_number,
                        game_screen,
                        cards,
                        imposter_index,
                        starting_player_index,
                    }
                },
            }
        }
    }
}

