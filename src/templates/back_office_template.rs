use askama::Template;
// Structure for context templates
#[derive(Template)]
#[template(path = "backoffice/login.html")]
pub struct LoginBackOfficeTemplate {}

#[derive(Template)]
#[template(path = "backoffice/index.html")]
pub struct HomepageBackOfficeTemplate {}
