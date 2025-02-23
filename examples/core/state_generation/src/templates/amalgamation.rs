use perseus::{RenderFnResultWithCause, Request, States, Template};
use sycamore::prelude::{view, Html, View};

#[perseus::make_rx(PageStateRx)]
pub struct PageState {
    pub message: String,
}

#[perseus::template_rx(AmalgamationPage)]
pub fn amalgamation_page(state: PageStateRx) -> View<G> {
    view! {
        p { (format!("The message is: '{}'", state.message.get())) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("amalgamation")
        // We'll generate some state at build time and some more at request time
        .build_state_fn(get_build_state)
        .request_state_fn(get_request_state)
        // But Perseus doesn't know which one to use, so we provide a function to unify them
        .amalgamate_states_fn(amalgamate_states)
        .template(amalgamation_page)
}

#[perseus::autoserde(amalgamate_states)]
pub fn amalgamate_states(states: States) -> RenderFnResultWithCause<Option<PageState>> {
    // We know they'll both be defined, and Perseus currently has to provide both as serialized strings
    let build_state = serde_json::from_str::<PageState>(&states.build_state.unwrap())?;
    let req_state = serde_json::from_str::<PageState>(&states.request_state.unwrap())?;

    Ok(Some(PageState {
        message: format!(
            "Hello from the amalgamation! (Build says: '{}', server says: '{}'.)",
            build_state.message, req_state.message
        ),
    }))
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(_path: String, _locale: String) -> RenderFnResultWithCause<PageState> {
    Ok(PageState {
        message: "Hello from the build process!".to_string(),
    })
}

#[perseus::autoserde(request_state)]
pub async fn get_request_state(
    _path: String,
    _locale: String,
    _req: Request,
) -> RenderFnResultWithCause<PageState> {
    Ok(PageState {
        message: "Hello from the server!".to_string(),
    })
}
