const MAX: usize = 12;
fn main() {
    let mut sums = vec![];

    let mut i = 1;
    while i < MAX {
        let mut j = i;
        let i_square = i*i;
        while j < MAX {
            sums.push(i_square + j*j);
            j += 1;
        }
        i += 1;
    }
    find_dup(sums);
}

fn find_dup(list: Vec<usize>) {
    let mut i = 0;
    while i < list.len() {
        let mut j = i+1;
        while j < list.len() {
            if list[i] == list[j] {
                println!("{}", list[i]);
            }
            j += 1;
        }
        i += 1;
    }
}

