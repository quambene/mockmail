#[derive(Debug, Default)]
pub struct Email {
    pub sender: String,
    pub receiver: String,
    pub subject: String,
    pub plaintext: String,
    pub html: String,
}
