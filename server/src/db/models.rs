use bcrypt::verify;

// User structure
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    forename: String,
    email: String,
    password: String,
}

// Implementation for user methods
impl User {
    pub fn assign(forename: String, email: String, password: String) -> Self {
        Self {
            forename,
            email,
            password: password,
        }
    }
    
    // getter method for email
    pub fn get_email(&self) -> &str {
        &self.email
    }

    // verifies hashed password
    pub fn verify_password(&self, password: &str) -> bool {
        verify(&password, &self.password).unwrap_or(false)
    }
}