fn main() {
    let input = String::from("ABC DEF");
    let index_of_first = first_word(&input);

    println!("The index = {}", index_of_first);
}

fn first_word(s: &String) -> usize {
    //Have to convert it to bytes here for some reason
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        //Check vs the empty string in bytes
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
