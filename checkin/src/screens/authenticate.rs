use crate::models::pins::Pin;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub struct PinPadOptions {
    pub length: u8,
    pub complete: &'static str,
    pub incomplete: &'static str,
}

impl Default for PinPadOptions {
    fn default() -> PinPadOptions {
        PinPadOptions {
            length: 6,
            complete: "🌟",
            incomplete: "⭐️",
        }
    }
}

#[component]
pub fn PinPad<'a>(active: ReadSignal<String>, options: &'a PinPadOptions) -> impl IntoView {
    view! {
        <div>
            {(0..active().chars().count())
                .map(|_| {
                    view! {
                        <span role="img" aria-label="completed">
                            {options.complete}
                        </span>
                    }
                })
                .collect::<Vec<_>>()}
            {(0..(options.length as usize - active().chars().count()))
                .map(|_| {
                    view! {
                        <span role="img" aria-label="completed">
                            {options.incomplete}
                        </span>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
