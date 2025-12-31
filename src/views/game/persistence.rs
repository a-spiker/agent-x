use crate::views::game::types::GameState;

// ============================================================================
// Session Management & Persistence Functions
// ============================================================================

/// Generate a unique session ID
pub fn generate_session_id() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string()
}

/// Load session ID from localStorage
pub fn load_session_id() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::window;
        
        let window = window()?;
        let storage = window.local_storage().ok()??;
        storage.get_item("agent_x_session_id").ok()?
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

/// Save session ID to localStorage
pub fn save_session_id(_session_id: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::window;
        
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("agent_x_session_id", _session_id);
            }
        }
    }
}

/// Load game state from localStorage
pub fn load_game_state(session_id: &str) -> Option<GameState> {
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::window;
        
        let window = window()?;
        let storage = window.local_storage().ok()??;
        let key = format!("agent_x_game_{}", session_id);
        let json = storage.get_item(&key).ok()??;
        serde_json::from_str(&json).ok()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = session_id;
        None
    }
}

/// Save game state to localStorage and optionally to server disk
pub fn save_game_state(_state: &GameState) {
    // Save to browser localStorage
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::window;
        
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(json) = serde_json::to_string(_state) {
                    let key = format!("agent_x_game_{}", _state.session_id);
                    let _ = storage.set_item(&key, &json);
                }
            }
        }
    }
}

// ============================================================================
// Server Functions (for fullstack mode with disk persistence)
// ============================================================================

// Uncomment these when running in fullstack mode with server feature
/*
#[server(SaveGameToDisk)]
async fn save_game_to_disk(session_id: String, game_state: String) -> Result<(), ServerFnError> {
    crate::server::save_game_to_disk(&session_id, &game_state)
        .map_err(|e| ServerFnError::ServerError(e))
}

#[server(LoadGameFromDisk)]
async fn load_game_from_disk(session_id: String) -> Result<String, ServerFnError> {
    crate::server::load_game_from_disk(&session_id)
        .map_err(|e| ServerFnError::ServerError(e))
}

#[server(ListSavedGames)]
async fn list_saved_games() -> Result<Vec<String>, ServerFnError> {
    crate::server::list_saved_games()
        .map_err(|e| ServerFnError::ServerError(e))
}
*/

