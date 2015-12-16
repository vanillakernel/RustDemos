use std::io;

fn main() {

 println!("Input a string and I will reverse it!");

 println!("Dat string:");

 let mut guess = String::new();

 io::stdin().read_line(&mut guess)
    .expect("Failed to read line.");
 //Trim that string
 let trimmed = &guess.trim();
 for c in trimmed.chars() {
        print!("{},  ", c);
 }

 //Return each character
 let mut range = 0..trimmed.len();
 let mut character_vec = Vec::new();
 let characters = trimmed.chars();
 //let reversed = characters.reverse();
 

 loop {
  match range.next() {
    Some(x) => {
      let character = trimmed.chars().nth(x);  
      //println!("{:?}", character);
      character_vec.push(character);
    }
    None => {
    break 
    }    
  println!("{:?}", character_vec.reverse());
  }
 } 
}
