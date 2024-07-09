struct TextWindow<'a> {
    content: &'a str,
}
impl<'a> TextWindow<'a> {
    fn new(content: &'a str) -> Self {
        TextWindow { content }
    }
    fn display(&self) {
        println!("Text window content: {}", self.content);
    }
}
fn main() {
    let text_window;
    {
        let my_text = "Hello, world!";
        text_window = TextWindow::new(my_text);
    }
     text_window.display();
}
