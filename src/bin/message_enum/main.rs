enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    match self {
      Message::Quit => println!("Quiting"),
      Message::Move { x, y } => println!("Moving: x-{}, y-{}", x, y),
      Message::Write(t) => println!("Wriing {}", t),
      Message::ChangeColor(r, g, b) => println!("Color is {}, {}, {}", r, g, b)
    }
  }
}

fn main() {
  let quit_message = Message::Quit;
  quit_message.call();
  let move_message = Message::Move { x: 32, y: 24 };
  move_message.call();
  let write_message = Message::Write("HaHa".to_string());
  write_message.call();
  let color_message = Message::ChangeColor(2, 3, 4);
  color_message.call();
}