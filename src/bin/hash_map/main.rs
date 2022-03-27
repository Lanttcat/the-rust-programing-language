use std::collections::HashMap;

fn main() {
  let text = "Hello world wonderful world";

  let mut counts = HashMap::new();

  for item in text.split_ascii_whitespace() {
    let count = counts.entry(item).or_insert(0);
    // 这里需要理解解引用的概念
    *count += 1
  }

  print!("{:?}", counts)
}