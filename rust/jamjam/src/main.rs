mod app;

#[cfg(feature = "ssr")]
use axum::response::sse::{Event, KeepAlive, Sse};
#[cfg(feature = "ssr")]
use futures_util::stream::Stream;
#[cfg(feature = "ssr")]
use std::convert::Infallible;

use console_error_panic_hook;

#[cfg(feature = "ssr")]
async fn counter_events() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    use app::get_server_count;
    use app::ssr_imports::*;
    use axum::response::sse::{Event, KeepAlive, Sse};
    use futures::StreamExt;

    let stream = futures::stream::once(async { get_server_count().await.unwrap_or(0) })
        .chain(COUNT_CHANNEL.clone())
        .map(|value| Ok(Event::default().data(format!("event: message\ndata: {value}\n\n"))));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::routing::get;
    use axum::Router;
    use jamjam::app::*;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    // get useful errors https://book.leptos.dev/getting_started/leptos_dx.html
    console_error_panic_hook::set_once();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(Counters);

    let app = Router::new()
        // server sent events cant be done with leptos server functions (yet?)
        // so we need to register it explicitly
        .route("/api/events", get(counter_events))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
