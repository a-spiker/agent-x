use dioxus::prelude::*;

use views::Game;

/// Define a views module that contains the UI for our app.
mod views;

/// Server-side functions for disk persistence
#[cfg(feature = "server")]
mod server;

/// The Route enum is used to define the structure of internal routes in our app.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Game {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const GAME_CSS: Asset = asset!("/assets/styling/game.css");

fn main() {
    dioxus::launch(App);
}

/// App is the main component of our app.
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: GAME_CSS }
        Router::<Route> {}
    }
}
