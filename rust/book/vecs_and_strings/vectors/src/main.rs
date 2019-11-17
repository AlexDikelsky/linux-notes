fn main() {
    let mut v = vec![5, 6, 7, 8, 9];
    
    v.push(9);

    for ele in &v {
       println!("{}", ele); 
    }

}
