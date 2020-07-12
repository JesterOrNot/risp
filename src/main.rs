mod lexer;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
  let mut rl = Editor::<()>::new();
  if rl.load_history("history.txt").is_err() {
    println!("No previous history.");
  }
  loop {
    let readline = rl.readline(">> ");
    match readline {
      Ok(line) => {
        println!("{:?}", lexer::tokenize(line));
      }
      Err(ReadlineError::Interrupted) => {
        println!("CTRL-C");
        break;
      }
      Err(ReadlineError::Eof) => {
        println!("CTRL-D");
        break;
      }
      Err(err) => {
        println!("Error: {:?}", err);
        break;
      }
    }
  }
  rl.save_history("history.txt").unwrap();
}
