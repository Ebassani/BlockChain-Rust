use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    amount: f64,
    sender: String,
    receiver: String,
}

impl Data {
    pub fn new(amount: f64, sender: String, receiver: String,) -> Self {
        Data {
            amount,
            sender,
            receiver
        }
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
