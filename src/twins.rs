#[derive(Debug, Clone)]
#[allow(unused)]
struct Twin {
    is: String,
}

#[allow(unused)]
fn twins() {
  let good_twin =  Twin{ is: "good".to_string() };
  let mut evil = good_twin.clone();
  evil.is = "evil".to_string();

  println!("Good twin: {:?}", good_twin);
}
