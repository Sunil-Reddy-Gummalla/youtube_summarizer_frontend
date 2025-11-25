use leptos::{prelude::*, reactive::spawn_local};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};
use serde::{Deserialize, Serialize};
use pulldown_cmark::{Options, Parser, html};
use dotenvy::dotenv;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/youtube_summarizer_frontend.css"/>

        // sets the document title
        <Title text="Youtube Summarizer"/>

        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Serialize)]
struct SummarizeRequest {
    url: String,
}

#[derive(Deserialize, Clone)]
struct SummarizeResponse {
    summary: Option<String>,
    error: Option<String>,
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (value, set_value) = signal("".to_string());
    let (result, set_result) = signal("Summary will appear here...".to_string());
    let (loading, set_loading) = signal(false);

    let is_disabled = move || value.get().trim().is_empty();
    let html_output = move || {
        let md = result.get();
        let mut html_output = String::new();

        let parser = Parser::new_ext(&md, Options::all());
        html::push_html(&mut html_output, parser);

        html_output
    };


    let summarize = move |_| {
        set_loading.set(true);
        let url_val = value.get();
        set_result.set("Loading summary...".to_string());

        spawn_local(async move {
            let client = reqwest::Client::new();
            let body = SummarizeRequest { url: url_val.clone() };
            dotenv().ok();
            let server_url = std::env::var("SERVER_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string());
            let request_url = format!("{}/summarize", server_url);

            match client
                .post(&request_url)
                .json(&body)
                .send()
                .await
            {
                Ok(resp) => match resp.json::<SummarizeResponse>().await {
                    Ok(data) => {
                        if let Some(summary) = data.summary {
                            set_result.set(summary);
                        } else if let Some(error) = data.error {
                            set_result.set(error);
                        } else {
                            set_result.set("API returned no summary or error.".to_string());
                        }
                    }
                    Err(e) => set_result.set(format!("JSON parse error: {}", e)),
                },
                Err(e) => set_result.set(format!("Network error: {}", e)),
            }

            set_loading.set(false);
        });
    };

    view! {
        <h1>"Welcome to Youtube Summarizer!"</h1>
        <div class="input-wrapper">
            <label class="input-label">Enter Youtube Url</label>
            <input
                class="custom-input"
                type="text"
                placeholder="Paste Youtube Url here..."
                on:input=move |ev| set_value.set(event_target_value(&ev))
            />
            <button
                class="custom-button"
                disabled=move || is_disabled()
                on:click=summarize
            >
                {move || if loading.get() { "Summarizing..." } else { "Summarize" }}
            </button>
        </div>

        <div class="result-box">
               <div class="summary" inner_html=html_output />
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <h1>"Not Found"</h1>
    }
}
