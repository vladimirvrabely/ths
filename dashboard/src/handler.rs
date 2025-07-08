use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

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
#[template(path = "index.html")]
struct IndexTemplate;

pub async fn index() -> Result<impl IntoResponse, AppError> {
    let template = IndexTemplate {};
    Ok(Html(template.render()?))
}
