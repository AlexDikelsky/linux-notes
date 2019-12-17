fn rule_1(x: &mut Vec<char>) -> &Vec<char> {
    if x[x.len()-1] == 'I' {
        x.push('U')
    }
    x
}
fn rule_2(x: &mut Vec<char>) -> &Vec<char> {
    for i in 1..(x.len()) {
        x.push(x[i]);
    };
    x
}
fn rule_3(x: &mut Vec<char>) -> &Vec<char> { //Shouldn't replace all IIIs
    let mut i = 0; 
    let found = false;
    while i < x.len()-2 && !found {
        if x[i] == 'I' && x[i+1] == 'I'&& x[i+2] == 'I' {
            x.remove(i);
            x.remove(i);
            x.remove(i);
            x.insert(i, 'I');
        }
        i += 1;
    }
    x
}
fn rule_4(x: &mut Vec<char>) -> &Vec<char> {
    let mut i = 0;
    let found = false;
    while i < x.len()-1 && !found {
        if x[i] == 'U' && x[i+1] == 'U' {
            x.remove(i);
            x.remove(i);
        }
        i += 1
    }
    x
}

fn un_nest(mut x: Vec<Vec<char>>) -> Vec<char> {
    let mut result = vec![];
    for i in x {
        for j in i {
            result.push(j);
        }
    }
    result
}

fn test_all(mut start: Vec<char>, target: Vec<char>, n: usize) -> bool {
    let mut v:Vec<Vec<char>> = vec![vec![]];
    let unchanged = start;
    for _i in 0..n {
        start = unchanged.clone();
        start = rule_1(&mut start).to_vec();
        v.push(un_nest(all(&mut start))); 
        
        start = unchanged.clone();
        start = rule_2(&mut start).to_vec();
        v.push(un_nest(all(&mut start))); 
       
        start = unchanged.clone();
        start = rule_3(&mut start).to_vec();
        v.push(rule_1(&mut start).to_vec());
        v.push(rule_2(&mut start).to_vec());
        v.push(rule_3(&mut start).to_vec());
        v.push(rule_4(&mut start).to_vec());

        start = unchanged.clone();
        start = rule_4(&mut start).to_vec();
        v.push(rule_1(&mut start).to_vec());
        v.push(rule_2(&mut start).to_vec());
        v.push(rule_3(&mut start).to_vec());
        v.push(rule_4(&mut start).to_vec());   
    
    }
    println!("{:?}", v);
    v.contains(&target)
}

fn all(x: &mut Vec<char>) -> Vec<Vec<char>> {
    vec![rule_1(&mut x.to_vec()).to_vec(), 
         rule_2(&mut x.to_vec()).to_vec(), 
         rule_3(&mut x.to_vec()).to_vec(), 
         rule_4(&mut x.to_vec()).to_vec(), 
    ]
}

fn main() {
    println!("{}", test_all(vec!['M', 'I'], vec!['M', 'I', 'I'], 3)); 
   /* let tests = vec![
        vec!['M', 'I'],
        vec!['M', 'I', 'U'],
        vec!['M', 'I', 'U', 'I', 'U'],//Far left branch
        vec!['M', 'I', 'I'],
        vec!['M', 'I', 'I', 'U'],
        vec!['M', 'I', 'I', 'I'],
        vec!['M', 'U', 'U'],
        vec!['M', 'I', 'U', 'U', 'U', 'U'],
        vec!['M', 'I', 'I', 'I', 'I', 'I'],
    ];
    for val in tests {
        println!("{:?}: {:?}", val, rule_3(&mut val.clone()));
        println!("{:?}: {:?} ", val.len(), rule_3(&mut val.clone()).len());
    }*/

}
