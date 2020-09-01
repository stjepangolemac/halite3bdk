pub struct Output;

impl Output {
    pub fn new() -> Self {
        Output
    }

    pub fn send(&self, msg: &str) {
        println!("{}", msg);
    }
}
