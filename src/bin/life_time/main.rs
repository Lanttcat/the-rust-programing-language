fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
  return if first.len() > second.len() {
    first
  } else {
    second
  }

}

fn main() {
  let x = "xxx";
  let y = "yy";
  let result = longest(x, y);

  println!("Longest: {}", result);
}