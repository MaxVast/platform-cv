use askama::Template;

// Structure for context templates
#[derive(Template)]
#[template(path = "backoffice/login.html")]
pub struct LoginBackOfficeTemplate {}

#[derive(Template)]
#[template(path = "backoffice/index.html")]
pub struct HomepageBackOfficeTemplate<'a> {
    pub username: String,
    pub company_name: String,
    pub role: &'a String,
    pub login_session: &'a String,
}
