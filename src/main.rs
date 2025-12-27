use iced::widget::{Svg, column, container, row, svg};
use iced::{Element, Length, Task};

struct App {}
struct Message {}

fn get_svg(path: &str, size: f32) -> Svg<'_> {
  let svg_name = path.replace(".svg", "");

  let svg_path = format!("svg/{svg_name}.svg");
  let svg = svg(iced::advanced::svg::Handle::from_path(svg_path))
    .width(size)
    .height(size);

  svg
}

impl App {
  fn new() -> (Self, Task<Message>) {
    (Self {}, Task::none())
  }

  fn update(&mut self, _: Message) -> Task<Message> {
    Task::none()
  }

  fn view(&self) -> Element<'_, Message> {
    container(column![
      row![
        get_svg("git.svg", 20.0),
        get_svg("git.svg", 30.0),
        get_svg("git.svg", 40.0)
      ].spacing(10),
      row![
        get_svg("ts.svg", 20.0),
        get_svg("ts.svg", 30.0),
        get_svg("ts.svg", 40.0)
      ].spacing(10),
      row![
        get_svg("md.svg", 20.0),
        get_svg("md.svg", 30.0),
        get_svg("md.svg", 40.0)
      ].spacing(10)
    ].spacing(20))
    .width(Length::Fill)
    .center_x(Length::Fill)
    .padding(40)
    .into()
  }
}

fn main() -> iced::Result {
  iced::application(App::new, App::update, App::view).run()
}
