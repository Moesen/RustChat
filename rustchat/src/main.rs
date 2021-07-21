mod test;

struct Cli {
  pattern: String,
  path: std::path::PathBuf,
}

fn main() {
  let pattern = std::env::args().nth(1).expect("no pattern given");
  let path = std::env::args().nth(2).expect("no path given");
  let str_path = String::from(&path);
  let args = Cli {
    pattern: pattern,
    path: std::path::PathBuf::from(path),
  }; 
  print!("pattern:\t {}\npath:\t\t {}", &args.pattern, str_path);

  test::hej();
}