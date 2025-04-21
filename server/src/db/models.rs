use bcrypt::verify;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    forename: String,
    email: String,
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
    
    pub fn get_forename(&self) -> &str {
        &self.forename
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(&password, &self.password).unwrap_or(false)
    }
}