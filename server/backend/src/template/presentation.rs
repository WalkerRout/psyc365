use askama::Template;

#[derive(Template)]
#[template(path = "presentation.html")]
pub struct Presentation;
