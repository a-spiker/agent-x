use std::fs;
use std::path::PathBuf;

/// Get the directory for storing game saves
fn get_saves_dir() -> PathBuf {
    let mut path = PathBuf::from("game_saves");
    if !path.exists() {
        let _ = fs::create_dir_all(&path);
    }
    path
}

/// Save game state to disk with session ID
pub fn save_game_to_disk(session_id: &str, game_state_json: &str) -> Result<(), String> {
    let saves_dir = get_saves_dir();
    let file_path = saves_dir.join(format!("{}.json", session_id));
    
    fs::write(&file_path, game_state_json)
        .map_err(|e| format!("Failed to save game: {}", e))?;
    
    Ok(())
}

/// Load game state from disk with session ID
pub fn load_game_from_disk(session_id: &str) -> Result<String, String> {
    let saves_dir = get_saves_dir();
    let file_path = saves_dir.join(format!("{}.json", session_id));
    
    if !file_path.exists() {
        return Err("Game save not found".to_string());
    }
    
    fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to load game: {}", e))
}

/// List all saved game sessions
pub fn list_saved_games() -> Result<Vec<String>, String> {
    let saves_dir = get_saves_dir();
    
    let entries = fs::read_dir(&saves_dir)
        .map_err(|e| format!("Failed to read saves directory: {}", e))?;
    
    let mut sessions = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".json") {
                    let session_id = filename.trim_end_matches(".json");
                    sessions.push(session_id.to_string());
                }
            }
        }
    }
    
    Ok(sessions)
}

/// Delete a saved game
pub fn delete_saved_game(session_id: &str) -> Result<(), String> {
    let saves_dir = get_saves_dir();
    let file_path = saves_dir.join(format!("{}.json", session_id));
    
    if !file_path.exists() {
        return Err("Game save not found".to_string());
    }
    
    fs::remove_file(&file_path)
        .map_err(|e| format!("Failed to delete game: {}", e))
}

