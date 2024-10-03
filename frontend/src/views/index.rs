use yew::*;
use yew_router::prelude::*;
use crate::{
    views::style::{
        // stylesheet::header_style,
    },
    route::Route,
    // handlers::{
    //     index_handler
    // },
};

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
            <header>
                <div id="header__menu">
                    <Link<Route> to={Route::Index}>{ "Home" }</Link<Route>>
                </div>
                <div id="header__account">
                    <Link<Route> to={Route::Signin} class={"register_style"}>{ "Sign In" }</Link<Route>>
                    <Link<Route> to={Route::Register} class={"register_style"}>{ "Register" }</Link<Route>>
                </div>
            </header>
            <main>
                <section>
                    <p>{ "Welcome to Bank" }</p>
                </section>
            </main>
            <footer>
                <h4>{ "© 2024 MyWebsite" }</h4>
            </footer>
        </>
    }
}