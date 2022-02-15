use std::io;

fn main() {
  let mut list = Vec::new();
  let mut input = String::new();
  eprint!("> ");
  loop {
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
      "list" => {
        for (idx, item) in list.iter().enumerate() {
          println!("{}. {}", idx + 1, item);
        }
      }
      "add" => {
        eprint!("What do you want to do? ");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        list.push(input.clone());
      },
      "delete" => {
        eprint!("Which item would you like to remove? ");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.as_str();
        list.retain(|item| item != input);
      },
      "quit" => break,
      unknown => eprintln!("Unknown command {:?}", unknown),
    }
    eprint!("> ");
  }
}
