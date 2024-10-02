use yew::prelude::*;
use yew_router::prelude::*;

//// Views/*.html
use crate::views::{
    amm::AMM, index::Index, notfound::NotFound,
    auth::{signin::Signin, register::Register, directions::Directions},
};

#[derive(Debug, PartialEq, Clone, Routable)]
pub enum Route {
    //// views/*.html
    #[at("/")]
    Index,
    #[at("/amm")]
    AMM,
    #[not_found]
    #[at("/notfound")]
    NotFound,

    //// views/auth/*.html
    #[at("/signin")]
    Signin,
    #[at("/register")]
    Register,
    #[at("/directions")]
    Directions,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! {<Index/>},
        Route::AMM => html! {<AMM/>},
        Route::Signin => html! {<Signin/>},
        Route::Register => html! {<Register/>},
        Route::Directions => html! {<Directions/>},
        Route::NotFound => html! {<NotFound/>},
    }
}