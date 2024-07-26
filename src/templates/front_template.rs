use askama::Template;
// Structure for context templates
#[derive(Template)]
#[template(path = "homepage/index.html")]
pub struct HomeTemplate {}
