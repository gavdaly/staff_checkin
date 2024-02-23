use axum::extract::FromRef;
use leptos::LeptosOptions;
use leptos_router::RouteListing;

#[derive(Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub routes: Vec<RouteListing>,
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.leptos_options.clone()
    }
}