use crate::views::game::types::{GameCard, CardType, WordList, WordCategory};

// Include the YAML file at compile time
const WORDS_YAML: &str = include_str!("../../../words.yaml");

/// Load word categories from YAML
fn load_word_categories() -> WordList {
    serde_yaml::from_str(WORDS_YAML).expect("Failed to parse words.yaml")
}

/// Get all available categories for selection
pub fn get_all_categories() -> Vec<WordCategory> {
    load_word_categories().categories
}

/// Helper function to generate cards for the round with a specific category
pub fn generate_cards_for_category(player_count: usize, category_index: usize) -> (Vec<GameCard>, usize, String, String) {
    use getrandom::getrandom;
    
    let word_list = load_word_categories();
    
    // Use the selected category
    let category = &word_list.categories[category_index % word_list.categories.len()];
    
    // Get random bytes for word pair selection
    let mut buf_word = [0u8; 8];
    let _ = getrandom(&mut buf_word);
    let random_word = u64::from_le_bytes(buf_word);
    
    // Get SEPARATE random bytes for imposter selection (ensures true randomness)
    let mut buf_imposter = [0u8; 8];
    let _ = getrandom(&mut buf_imposter);
    let random_imposter = u64::from_le_bytes(buf_imposter);
    
    // Select random word pair from the chosen category
    let pair_index = (random_word as usize) % category.pairs.len();
    let (normal_word, imposter_word) = &category.pairs[pair_index];
    
    // Select random imposter index (using separate random value)
    let imposter_idx = (random_imposter as usize) % player_count;
    
    let mut cards = Vec::new();
    for i in 0..player_count {
        if i == imposter_idx {
            cards.push(GameCard {
                card_type: CardType::Imposter,
                word: imposter_word.clone(),
            });
        } else {
            cards.push(GameCard {
                card_type: CardType::Normal,
                word: normal_word.clone(),
            });
        }
    }
    
    (cards, imposter_idx, category.name.clone(), category.icon.clone())
}
