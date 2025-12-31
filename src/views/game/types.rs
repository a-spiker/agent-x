use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub score: i32,
    pub is_eliminated: bool,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum CardType {
    Normal,
    Imposter,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GameCard {
    pub card_type: CardType,
    pub word: String,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum GameScreen {
    Setup,
    CategorySelection,
    CategoryReveal { category_name: String, category_icon: String },
    CardView { current_player_index: usize },
    Voting,
    Elimination { eliminated_index: usize, was_imposter: bool },
    RoundEnd { imposter_found: bool, game_over: bool },
    GameScore,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub session_id: String,
    pub game_screen: GameScreen,
    pub players: Vec<Player>,
    pub player_count_input: String,
    pub player_names: Vec<String>,
    pub round_number: i32,
    pub cards: Vec<GameCard>,
    pub imposter_index: usize,
    pub current_category: Option<(String, String)>, // (name, icon)
    pub selected_category_index: Option<usize>, // Selected category index for the round
}

// Word list structures
#[derive(Debug, Deserialize)]
pub struct WordCategory {
    pub name: String,
    pub icon: String,
    pub pairs: Vec<(String, String)>,
}

#[derive(Debug, Deserialize)]
pub struct WordList {
    pub categories: Vec<WordCategory>,
}

