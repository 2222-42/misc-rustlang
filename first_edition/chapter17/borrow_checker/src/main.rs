#[derive(Debug)]
pub struct Test {
    pub content: Option<String>,
}

impl Test {
    // pub fn add_text(&mut self, text: String) {
    pub fn add_text(self, text: String) -> Self {
        if self.check_not_f(&text) {
            self.set(text)
        } else {
            self
        }
    }

    fn set(mut self, text: String) -> Self {
        self.content = Some(format!("{}{}", self.content.take().unwrap(), text));
        self
    }

    fn check_not_f(&self, text: &String) -> bool {
        text != "fuck"
    }
}

fn main() {
    let mut test = Test {
        content: Some("test".to_string()),
    };

    let result = test.add_text("test".to_string());
    println!("{:?}", result);
}
