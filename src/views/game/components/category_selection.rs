use dioxus::prelude::*;
use crate::views::game::{types::*, utils::get_all_categories};

#[component]
pub fn CategorySelectionScreen(
    mut game_screen: Signal<GameScreen>,
    mut selected_category_index: Signal<Option<usize>>,
) -> Element {
    // Use a signal to store categories so they live long enough
    let categories = use_signal(|| get_all_categories());
    let cats = categories.read();
    
    rsx! {
        div { class: "category-selection-screen",
            div { class: "selection-header",
                h1 { "🎯 Choose Your Category" }
                p { class: "selection-subtitle", "Pick a theme for this round" }
            }
            
            div { class: "categories-grid",
                for (index, category) in cats.iter().enumerate() {
                    {
                        let cat_name = category.name.clone();
                        let cat_icon = category.icon.clone();
                        rsx! {
                            div { 
                                key: "{index}",
                                class: "category-card-selectable",
                                onclick: move |_| {
                                    selected_category_index.set(Some(index));
                                    game_screen.set(GameScreen::CategoryReveal { 
                                        category_name: cat_name.clone(),
                                        category_icon: cat_icon.clone() 
                                    });
                                },
                                div { class: "category-icon-large", "{cat_icon}" }
                                h3 { class: "category-name-selectable", "{cat_name}" }
                                p { class: "category-pairs-count", 
                                    "{category.pairs.len()} word pairs" 
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

