use leptos::prelude::*;
use leptos::web_sys::SubmitEvent;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{FlatRoutes, Route, Router, A},
    StaticSegment,
};

#[cfg(feature = "ssr")]
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct GlobalQueue {
    pub queue: Vec<String>,
    pub count: usize,
    pub leptos_options: LeptosOptions,
}

#[cfg(feature = "ssr")]
pub mod ssr_imports {
    pub use broadcaster::BroadcastChannel;
    pub use std::sync::atomic::{AtomicI32, Ordering};
    use std::sync::LazyLock;

    pub static COUNT: AtomicI32 = AtomicI32::new(0);

    pub static COUNT_CHANNEL: LazyLock<BroadcastChannel<i32>> =
        LazyLock::new(BroadcastChannel::<i32>::new);
}

#[server]
#[cfg_attr(feature = "ssr", instrument)]
pub async fn get_server_count() -> Result<i32, ServerFnError> {
    use ssr_imports::*;

    Ok(COUNT.load(Ordering::Relaxed))
}

#[server]
#[cfg_attr(feature = "ssr", instrument)]
pub async fn adjust_server_count(delta: i32, msg: String) -> Result<i32, ServerFnError> {
    use ssr_imports::*;

    let new = COUNT.load(Ordering::Relaxed) + delta;
    COUNT.store(new, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&new).await;
    println!("message = {:?}", msg);
    Ok(new)
}

#[server]
#[cfg_attr(feature = "ssr", instrument)]
pub async fn clear_server_count() -> Result<i32, ServerFnError> {
    use ssr_imports::*;

    COUNT.store(0, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&0).await;
    Ok(0)
}

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1"
                />
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options.clone()/>
                <MetaTags/>
            </head>
            <body>
                <Counters/>
            </body>
        </html>
    }
}

#[component]
pub fn Counters() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/jamjam.css"/>
        <Router>
            <main>
                <FlatRoutes fallback=|| "Not found.">
                    <Route path=StaticSegment("/") view=MultiuserCounter/>
                    <Route path=StaticSegment("/search") view=SearchResult/>
                </FlatRoutes>
            </main>
        </Router>
    }
}

#[component]
pub fn SearchResult() -> impl IntoView {
    view! {
        <p>"barf!"</p>
    }
}

#[server]
#[cfg_attr(feature = "ssr", instrument)]
pub async fn search_query(query: String) -> Result<(), ServerFnError> {
    leptos_axum::redirect("/search");
    Ok(())
}

// This is a kind of "multi-user" counter
// It relies on a stream of server-sent events (SSE) for the counter's value
// Whenever another user updates the value, it will update here
// This is the primitive pattern for live chat, collaborative editing, etc.
#[component]
pub fn MultiuserCounter() -> impl IntoView {
    let dec = Action::new(|_: &()| adjust_server_count(-1, "dec dec goose".into()));
    let inc = Action::new(|_: &()| adjust_server_count(1, "inc inc moose".into()));
    let clear = Action::new(|_: &()| clear_server_count());

    #[cfg(not(feature = "ssr"))]
    let multiplayer_value = {
        use futures::StreamExt;
        use send_wrapper::SendWrapper;

        let mut source = SendWrapper::new(
            gloo_net::eventsource::futures::EventSource::new("/api/events")
                .expect("couldn't connect to SSE stream"),
        );
        let ret =
            ReadSignal::from_stream_unsync(source.subscribe("message").unwrap().map(|value| {
                match value {
                    Ok(value) => value.1.data().as_string().expect("expected string value"),
                    Err(_) => "0".to_string(),
                }
            }));

        on_cleanup(move || source.take().close());
        ret
    };

    let (song_search, set_name) = signal("".to_string());
    let input_element: NodeRef<leptos::html::Input> = NodeRef::new();
    let song_query = Action::new(|query: &String| search_query(query.to_string()));

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name.set(value.clone());
        song_query.dispatch(value);
    };

    #[cfg(feature = "ssr")]
    let (multiplayer_value, _) = signal(None::<i32>);

    view! {
        <header>
            <div class="welcome">
                <div>
                    <a href="http://picasion.com/gl/mhp3">
                        <img src="http://i.picasion.com/gl/93/mhp3.gif" width="350" height="59" border="0" alt="glitter maker"> </img>
                    </a>
                    <p>"welcome to music heaven :3"</p>
                </div>
                <div>
                    <p>"plz join the party! ⸜(｡˃ ᵕ ˂ )⸝♡"</p>
                    <div>
                        <form on:submit=on_submit> // on_submit defined below
                            <input type="text"
                                value=song_search
                                node_ref=input_element
                            />
                            <input type="submit" value="check this sick beat!"/>
                        </form>
                    </div>
                </div>
            </div>
        </header>
        <main>
        <div class="booty">
            <p class="separator">"currently playing!!!!"</p>
            <div class="sidecontainer">
                <div class="songinfo">
                    <p>"song:  " {song_search}</p>
                    <p>"artist:"</p>
                </div>
                <div class="boykisser">
                    <a href="/gif/boykisser-dance-wFwDKk" title="boykisser dance">
                        <img src="https://i.makeagif.com/media/5-29-2024/wFwDKk.gif" width="140" height="100" border="0" alt="boykisser dance"></img></a>
                </div>
            </div>
            <div>
                <p class="separator">"queued music"</p>
                <h1>"Server-Side Counters"</h1>
                <p>"Each of these counters stores its data in the same variable on the server."</p>
                <p>
                    "The value is shared across connections. Try opening this is another browser tab to see what I mean."
                </p>
            </div>
            <nav>
                <ul>
                    <li>
                        <A href="/">"Multi-User"</A>
                    </li>
                </ul>
            </nav>
            <div>
                <h2>"Multi-User Counter"</h2>
                <p>
                    "This one uses server-sent events (SSE) to live-update when other users make changes."
                </p>
                <div>
                    <button on:click=move |_| { clear.dispatch(()); }>"Clear"</button>
                    <button on:click=move |_| { dec.dispatch(()); }>"-1"</button>
                    <span>
                        "Multiplayer Value: " {move || multiplayer_value.get().unwrap_or_default()}
                    </span>
                    <button on:click=move |_| { inc.dispatch(()); }>"+1"</button>
                </div>
            </div>
        </div>
        </main>
    }
}

// #[server]
// pub async fn jam_jam_queue() -> Result<usize, ServerFnError> {
//     let state = expect_context::<GlobalQueue>();
//     state.count += 1;
//     Ok(state.count)
// }
