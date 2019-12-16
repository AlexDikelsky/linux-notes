fn main() {
    for i in (0..111).step_by(10) {
        println!("Val = {}, Real = {}, logs = {}, division = {}", i, (i.to_string()).len(), length_of_num(i), div_by_10(i as f64));
    }
    //for i in (0..111).step_by(10) {
    //    println!("Val = {}, Real len = {}, computed = {}", i, (i.to_string()).len(), length_of_num(i));
    //}
    //for i in (0..111).step_by(10) {
    //    println!("Val = {}, Real len = {}, computed = {}", i, (i.to_string()).len(), div_by_10(i as f64));
    //}
}

fn length_of_num(x: usize) -> usize {
    if x == 0 {
        1
    } else {
        let logarithm = (x as f64).log(10.0);       
        let ceil = logarithm.ceil();
        if logarithm == ceil {
            ceil as usize + 1
        } else {
            ceil as usize
        }
    }
}

fn div_by_10(x: f64) -> usize {
    if x == 0.0 {
        return 1
    } else if x < 1.0 {
        return 0
    } else {
        return div_by_10(x / 10.0) + 1
    }
}
