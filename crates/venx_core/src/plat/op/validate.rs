impl Plat{
    pub fn validate(&self) -> ValidationResult {
        todo!()
    }
}

enum ValidationResult{
    Correct,
    Error(bool)
}