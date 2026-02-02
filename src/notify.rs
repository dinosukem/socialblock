pub struct ConsoleNotifier;

impl ConsoleNotifier {
    pub fn new() -> Self {
        Self
    }

    pub fn info(&self, msg: &str) {
        println!("[INFO] {msg}");
    }
}

impl Default for ConsoleNotifier {
    fn default() -> Self {
        Self::new()
    }
}
