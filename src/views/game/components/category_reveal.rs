use dioxus::prelude::*;
use crate::views::game::types::*;

#[component]
pub fn CategoryRevealScreen(
    category_name: String,
    category_icon: String,
    mut game_screen: Signal<GameScreen>,
) -> Element {
    rsx! {
        div { class: "category-reveal-screen",
            div { class: "category-card",
                div { class: "category-icon", "{category_icon}" }
                h1 { class: "category-title", "Category" }
                h2 { class: "category-name", "{category_name}" }
                p { class: "category-hint", 
                    "All players will receive words related to this category"
                }
                button {
                    class: "continue-btn",
                    onclick: move |_| {
                        game_screen.set(GameScreen::CardView { current_player_index: 0 });
                    },
                    "▶️ Start Round"
                }
            }
        }
    }
}

