use std::str::FromStr;

use crate::chart::*;
use crate::error_template::{AppError, ErrorTemplate};
use crate::felt_fraction::FeltFraction;
use leptos::*;
use leptos_dom::helpers::debounce;
use leptos_meta::*;
use leptos_router::*;
use logging::log;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-playground.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="fraction" view=Fraction/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        // <Chart/>
    }
}

#[component]
fn Fraction() -> impl IntoView {
    let (value, set_value) = create_signal(FeltFraction::zero());

    let update_value = move |ev| {
        let input_value = event_target_value(&ev);
        set_value.set(FeltFraction::from_str(&input_value).unwrap());
    };

    view! {
        <h1>"Fraction"</h1>
        <input on:input=update_value />
        // Logujemy wartość

        <p>{create_memo(move |_| value.get().to_string())}</p>
    }
}
