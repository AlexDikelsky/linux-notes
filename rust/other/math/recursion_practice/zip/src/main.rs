fn zip(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    let mut x = Vec::new();
    if a.len() == 0 {
        return x
    }
    else if b.len() == 0 {
        x.push(a[0]);
        x
    }
    else {
        let mut x = zip(&a[1..].into(), &b[1..].into());
        x.insert(0, b[0]);
        x.insert(0, a[0]);
        x
    }
}

fn main() {
    let mut a = Vec::new();
    let mut b = Vec::new();

    for i in 1..3 {
        a.push(i*10);
        println!("{:?} is {:?} + {:?}", zip(&a, &b), &a, &b);
        b.push(i*100);
        println!("{:?} is {:?} + {:?}", zip(&a, &b), &a, &b);
    }
    let mut y = vec!(11, 22, 33, 44);
    let mut z = vec!(55, 66, 77);
    println!("{:?}", zip(&y, &z));
}
