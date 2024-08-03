use crate::models::company::Company;
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
    #[allow(dead_code)]
    pub login_session: &'a String,
}

#[derive(Template)]
#[template(path = "backoffice/add_user.html")]
pub struct AddUserBackOfficeTemplate<'a> {
    pub list_company: &'a Vec<Company>,
    pub role: &'a String,
}
