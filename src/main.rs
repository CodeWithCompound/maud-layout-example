use axum::{
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use maud::{html, Markup, DOCTYPE};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

struct MaudResponse(Markup);

impl IntoResponse for MaudResponse {
    fn into_response(self) -> Response {
        Html(self.0.into_string()).into_response()
    }
}

macro_rules! layout {
    ($title:expr, $content:expr) => {
        html! {
            (DOCTYPE)
            html lang="de" {
                head {
                    meta charset="utf-8";
                    title { ($title) }
                    //link rel="stylesheet" href="/static/style.css";
                 style {
                    r#"
                    .btn {
                        display: inline-block;
                        padding: 0.5em 1em;
                        background: #b3b3b3;
                        color: white;
                        text-decoration: none;
                        border-radius: 4px;
                        font-family: sans-serif;
                    }
                    .btn:hover {
                        background: #707070;
                    }
                    "#
                }
                }
                body {
                    main {
                        ($content) 
                    }
                }
            }
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(main_handler))
        .route("/2", get(sec_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())

}

async fn main_handler() -> MaudResponse {
    let content = html! {
        h1 { "test" }
        a.btn href="/2" {"secondary"}
    };
    MaudResponse(layout!("main", content))
}

async fn sec_handler() -> MaudResponse {
    let content = html! {
        h1 { "second test" }
        a.btn href="/" { "main page" }
    };
    MaudResponse(layout!("secondary", content))
}
