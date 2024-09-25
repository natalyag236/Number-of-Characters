use std::collections::HashMap;
use std::io::{self,Write};

// the main exeutes the chararcter counting 
pub fn main() {
    let mut input = String::new();
    print!("Enter a string: ");
    io::stdout().flush().unwrap(); 

   // read the users input from the input var
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input ");
       
   
    // the hashmap stores the character count 
    let mut char_count: HashMap<char, i32> = HashMap::new(); 
    // goes over each character from the input string 
    for ch in input.chars() {
        let count: &mut i32 = char_count.entry(ch).or_insert(0);
        *count += 1;
    }

    let mut max_char: char = ' ';// var to hold the most frequent characters
    let mut max_count: i32 = 0; // var to hold the total num of characters
// goes over the characters to find the most frequent character
    for (ch, count) in &char_count {
        if *count > max_count {
            max_count = *count;
            max_char = *ch;
        }
    }
    
    
    // Prints the most frequent characters and how many times it appears 
    if max_count > 1 {
        println!("The most frequent occurring character is '{}' and appears {} times.", max_char, max_count);
    }else{
        print!("No repeating characters")
    }
}
