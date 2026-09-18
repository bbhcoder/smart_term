use crate::history::queries;

pub struct UserSession {
    pub current_user: Option<String>,
}

impl UserSession {
    pub fn new() -> Self {
        Self { current_user: queries::get_def_user() }
    }

    pub fn login(&mut self, user: &str) {
        self.current_user = Some(user.to_string());
    }

    pub fn logout(&mut self) {
        self.current_user = queries::get_def_user();
    }

    pub fn active_user(&self) -> &str {
        self.current_user.as_deref().unwrap_or("default")
    }
}