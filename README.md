# Agent-X: Social Deduction Game

A fun social deduction game built with Dioxus 0.7 where players try to find the imposter among them!

## 🎮 Game Overview

**Agent-X** is a mobile-friendly web game for 3+ players. One player is randomly assigned as the imposter with a different word, while all other players receive the same word. Through discussion and voting, players try to identify the imposter before they blend in successfully.

## 🚀 Quick Start

### Prerequisites
- Rust installed on your system
- Dioxus CLI

### Installation

1. Install Dioxus CLI:
```bash
curl -sSL http://dioxus.dev/install.sh | sh
```

2. Run the game locally:
```bash
dx serve --platform web
```

The game will be available at `http://localhost:8080`

### 📱 Running on Mobile (Same Network)

To access the game from your mobile phone on the same WiFi network:

1. The app is already configured to bind to all network interfaces (`0.0.0.0:8080`)
2. Find your computer's local IP address:
   - **macOS/Linux**: `ifconfig | grep "inet " | grep -v 127.0.0.1`
   - **Windows**: `ipconfig`
3. On your mobile device, open the browser and navigate to:
   ```
   http://YOUR_COMPUTER_IP:8080
   ```
   Example: `http://192.168.1.100:8080`

This allows all players to use the same device or different devices on the network!

## 📱 How to Play

1. **Setup**: Enter the number of players (3+) and their names
2. **Card View**: Each player reveals their card privately (pass the device around)
3. **Discussion**: Players discuss to find who has the odd word
4. **Voting**: Vote for who you think is the imposter
5. **Scoring**: 
   - If imposter found: Regular players get 10 points each
   - If imposter not found: Imposter gets 20 points

## 📂 Project Structure

```
agent-x/
├─ assets/
│  ├─ favicon.ico
│  └─ styling/
│     └─ game.css      # Game styling
├─ src/
│  ├─ main.rs          # App entry point and routing
│  └─ views/
│     ├─ mod.rs        # Views module
│     └─ game.rs       # Game logic and components
└─ Cargo.toml
```

## 🎨 Features

- Beautiful gradient UI with smooth animations
- Mobile-optimized responsive design
- Privacy-focused card reveal system
- Score tracking across multiple rounds
- Supports 3-10 players
- **💾 Auto-save game state** - Resume your game after browser refresh or restart
- **🔐 Session-based persistence** - Each game gets a unique Session ID
- **📱 Cross-device support** - Share Session ID to continue on another device (with server mode)

## 🛠️ Technology

- **Framework**: Dioxus 0.7
- **Language**: Rust
- **Styling**: Custom CSS
- **Persistence**: Browser localStorage + Optional server disk storage
- **Session Management**: UUID-based session IDs

## 💾 Game Persistence & Session Management

Agent-X features automatic game state persistence that allows you to resume your game seamlessly after browser refresh, closing tabs, or even restarting your device.

### ✨ How It Works

**Automatic Save:**
- Game state is saved to your browser's localStorage automatically on every action
- No manual save button needed - everything happens in the background
- Works completely offline, no server required

**What Gets Saved:**
- ✅ Player names and scores
- ✅ Current game screen (Setup, CardView, Voting, etc.)
- ✅ Round number and card assignments
- ✅ Elimination status and imposter identity
- ✅ All game progress

**Session ID:**
- Each game session gets a unique UUID (visible in top-right corner)
- Format: `Session: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`
- Session ID is persistent across browser restarts

### 🔄 Resume Game

**Single Device:**
1. Simply refresh the page or close/reopen the browser
2. The game automatically loads your last state
3. Continue playing from exactly where you left off

**Multiple Devices (Same Network):**
1. Note the Session ID from the top-right corner
2. Open the game on another device: `http://YOUR_IP:8080`
3. The same session will be loaded automatically
4. Both devices share the same game state via localStorage

### 🗑️ Start Fresh Game

**Option 1:** Click the "New Game" button in the game (recommended)

**Option 2:** Clear browser data manually:
```javascript
// Open browser console (F12) and run:
localStorage.clear();
// Then refresh the page
```

### 🔧 Technical Details

**Browser Storage:**
- Uses browser localStorage API
- Storage key: `agent_x_game_{session-id}`
- Session key: `agent_x_session_id`
- Data format: JSON serialized game state

**Data Structure:**
```rust
{
  session_id: String,
  game_screen: GameScreen,
  players: Vec<Player>,
  round_number: i32,
  cards: Vec<GameCard>,
  imposter_index: usize,
  // ... and more
}
```

**Privacy:**
- All data stays in your browser
- No data sent to external servers
- No tracking or analytics
- Completely offline-capable

### 🖥️ Server Disk Persistence (Optional)

For advanced users, server-side persistence can be enabled:

**Enable Server Mode:**
```bash
# Build with server feature
cargo build --features server

# Run fullstack mode
dx serve --features server
```

**Server Storage:**
- Game saves stored in `game_saves/` directory
- One JSON file per session: `{session-id}.json`
- Allows true cross-device synchronization
- Survives server restarts

**Server Functions Available:**
- `save_game_to_disk()` - Save game state to server
- `load_game_from_disk()` - Load game state from server
- `list_saved_games()` - List all saved sessions
- `delete_saved_game()` - Remove a saved session

**Note:** Server persistence requires uncommenting server functions in `src/views/game.rs` (lines marked with comments)

### 🔒 Security & Privacy

- **localStorage Only:** By default, all data stays in your browser
- **No External Calls:** No analytics, tracking, or external API calls
- **Random Session IDs:** Cryptographically secure UUID generation
- **Local-First:** Works 100% offline without any server

### 💡 Best Practices

1. **Regular Play:** Just play normally - saving is automatic
2. **Share Session:** Copy Session ID for friends to join/view
3. **New Session:** Always use "New Game" button for clean state
4. **Browser Compatibility:** Ensure localStorage is enabled (default in all modern browsers)

## 📄 License

MIT License
