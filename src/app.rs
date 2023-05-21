use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Debug)]
pub enum SoundEvent {
    Off,
    Play,
    Pause,
    Default,
}

pub fn match_event(event: SoundEvent) -> &'static str {
    let state = match event {
        SoundEvent::Off => "Off Event",
        SoundEvent::Play => "Play Event",
        SoundEvent::Pause => "Pause event",
        SoundEvent::Default => "Default event", 
    };
    state
   }

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        <Title text="Welcome to Leptos"/>

        <Header />

        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Header(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <header>
            <h1>"Odio"</h1>
        </header>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let default_state = match_event(SoundEvent::Off);
    

    let (sound_state, set_sound_state) = create_signal(cx, default_state);

    let play_sound = move |_| {
        println!("{:#?}", sound_state);
        set_sound_state(match_event(SoundEvent::Play));
    };

    view! { cx,
        <h3>"HOME"</h3>
        <button on:click=play_sound>{sound_state}</button>
    }
}
