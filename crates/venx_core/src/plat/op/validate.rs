impl Plat {
    pub fn validate(&self) -> ValidationResult {
        // TODO: Hashsum (Wrong one: this plat is corrupted or compromised)
        todo!()
    }
}

enum ValidationResult {
    Correct,
    Error(bool),
}
