use dioxus::prelude::*;
use crate::views::game::types::*;

#[component]
pub fn CategoryRevealScreen(
    category_name: String,
    category_icon: String,
    mut game_screen: Signal<GameScreen>,
    mut hide_imposter_identity: Signal<bool>,
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
                
                // Hard Mode Toggle
                div { class: "hard-mode-toggle",
                    label { class: "toggle-label",
                        input {
                            r#type: "checkbox",
                            class: "toggle-checkbox",
                            checked: hide_imposter_identity(),
                            oninput: move |evt| {
                                hide_imposter_identity.set(evt.value() == "true");
                            }
                        }
                        span { class: "toggle-text",
                            "🔥 Hard Mode"
                        }
                    }
                    p { class: "hard-mode-description",
                        if hide_imposter_identity() {
                            "The imposter won't know they're the imposter!"
                        } else {
                            "Enable to hide the imposter's identity from themselves"
                        }
                    }
                }
                
                div { class: "action-buttons-row",
                    button {
                        class: "back-btn",
                        onclick: move |_| {
                            game_screen.set(GameScreen::CategorySelection);
                        },
                        "← Back to Categories"
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
}

