// use crate::views::{header::AuthHeader, home::Home, login::Login, not_found::NotFound};
use crate::views::{
    navbar::NavBar ,client_companies::ClientCompanies, cvs::Cvs, home::Home, job_functions::JobFunctions,
    keywords::Keywords, page404::PageNotFound, users::Users,
};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

// #[derive(Routable, Clone)]
// pub enum Route {
//     #[layout(AuthHeader)]
//     #[route("/")]
//     Home {},

//     // https://dioxuslabs.com/learn/0.4/router/reference/routes#query-segments
//     #[route("/login?:query_string")]
//     Login { query_string: String },
//     #[end_layout]
//     #[route("/:..route")]
//     NotFound { route: Vec<String> },
// }

#[derive(Clone, Routable, Debug, PartialEq)]
pub(crate) enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/keywords/")]
        Keywords {},
        // #[route("/keyword/:id")]
        // Keyword { id: i32 },
        #[route("/clients/")]
        ClientCompanies {},
        #[route("/cvs/")]
        Cvs {},
        #[route("/job_functions/")]
        JobFunctions {},
        #[route("/users/")]
        Users {},
    #[end_layout]
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}
