fn unzip_1(v: &Vec<u32>) -> Vec<Vec<u32>> {
    let empty = vec![];
    let mut two_empty = vec!(empty.clone(), empty.clone());

    if v.len() == 0 {
        two_empty
    }
    else if v.len() == 1{
        two_empty[0].push(v[0]);
        two_empty
    }
    else {
        two_empty = unzip_1(&v[2..].into());
        two_empty[0].push(v[0]);
        two_empty[1].push(v[1]);
        two_empty
    }
}

fn unzip(v: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut nums = unzip_1(v);
    nums[0].reverse();
    nums[1].reverse();
    nums
}

fn main() {
    let mut v = Vec::new();
    for i in 1..8 {
        v.push(i*11);
    }

    println!("{:?}", unzip(&v));
}
