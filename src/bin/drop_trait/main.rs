struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping the data: {}", self.data)
  }
}

fn main() {
  // 这里为什么清理_b?
  let _a = CustomSmartPointer { data: "Struct a".to_string() };
  let _b = CustomSmartPointer { data: "Struct b".to_string() };

  println!("Created Struct")
}