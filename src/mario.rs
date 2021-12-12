pub enum QuestionMarkBox {
    Empty,
    Money(u32),
    PowerUp { effect: String, amount: String },
}

#[allow(unused)]
struct Mario {}

impl Mario {
    #[allow(unused)]
    fn open_box(&self, qmbox: QuestionMarkBox) -> String {
        match qmbox {
            QuestionMarkBox::Empty => "shucks!".to_string(),
            QuestionMarkBox::Money(amt) => format!("I got ${}!!!", amt),
            QuestionMarkBox::PowerUp { effect, amount } => {
                format!("Received {} effect for +{}.", effect, amount)
            }
        }
    }
}
