use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes, A}, path, StaticSegment
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // sets the document title
        <Title text="Leptos app" />

        // content for this welcome page
        <Router>
            <main>
                <div class="container">
                    <Routes fallback=|| ()>

                    <Route path=StaticSegment("") view=HomePage />
                    <ParentRoute view=Parent path=path!("")>
                        <Route path=path!("subpage") view=SubPage />
                    </ParentRoute>
                    </Routes>
                </div>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
       <Meta property="og:title" content="Main page" />
        <Meta name="description" content="c"/>
        <Meta property="og:description" content="c."/>
        <div>test1</div>
        <A href="subpage">
            "To subpage"
        </A>
    }
}

#[component]
fn SubPage() -> impl IntoView {
    view! {
        <Meta property="og:title" content="Subpage" />
        <Meta name="description" content="d"/>
        <Meta property="og:description" content="d"/>
        <div>test</div>
        <div>Stuff</div>
    }
}

#[component]
fn Parent() -> impl IntoView {
    view! {
        <div class="header-container">
            <A href="/">
                "To home"
            </A>
        </div>
        <Outlet />
    }
}