use bcrypt::verify;

#[derive(Debug, serde::Deserialize)]
pub struct User {
    pub forename: String,
    pub email: String,
    password: String,
}

impl User {
    pub fn assign(forename: String, email: String, password: String) -> Self {
        Self {
            forename,
            email,
            password: password,
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(&password, &self.password).unwrap_or(false)
    }
}