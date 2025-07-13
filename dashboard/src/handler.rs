use askama::Template;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
};

use crate::model::Measurement;
use crate::state::SharedState;

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    status: StatusCode,
    app_error: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("could not render template")]
    Render(#[from] askama::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let tmpl = ErrorTemplate {
            status,
            app_error: format!("{self}"),
        };
        if let Ok(body) = tmpl.render() {
            (status, [("HX-Push-Url", "/error")], Html(body)).into_response()
        } else {
            (status, [("HX-Push-Url", "/error")], "Something went wrong").into_response()
        }
    }
}

#[derive(Template)]
#[template(path = "index.html", blocks=["main"])]
struct IndexTemplate {
    measurement: Measurement,
}

pub async fn index(
    headers: HeaderMap,
    State(mut state): State<SharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::debug!("GET / request");

    let measurement = state
        .0
        .get_latest_measurement()
        .await
        .expect("should have msr");

    let template = IndexTemplate { measurement };

    if headers.get("hx-request").is_some() {
        Ok(Html(template.as_main().render()?).into_response())
    } else {
        Ok(Html(template.render()?).into_response())
    }
}

#[derive(Template)]
#[template(path = "history.html", blocks=["main"])]
struct HistoryTemplate {}

pub async fn history(
    headers: HeaderMap,
    State(mut _state): State<SharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::debug!("GET /history request");

    let template = HistoryTemplate {};

    if headers.get("hx-request").is_some() {
        Ok(Html(template.as_main().render()?).into_response())
    } else {
        Ok(Html(template.render()?).into_response())
    }
}
