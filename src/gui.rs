use iced::{widget::column, Element, Sandbox, Settings};
pub fn launch() {
	GuiOgas::run(Settings::default()).unwrap();
}
struct GuiOgas {}
#[derive(Debug)]
enum Message {}
impl Sandbox for GuiOgas {
	type Message = Message;
	fn new() -> Self {
		GuiOgas {}
	}
	fn title(&self) -> String {
		String::from("OGAS")
	}
	fn update(&mut self, _message: Self::Message) {}
	fn view(&self) -> Element<Message> {
		column![].into()
	}
}
