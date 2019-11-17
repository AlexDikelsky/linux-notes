pub use std::collections::HashMap;

pub struct Sieve {
    composites: HashMap<u64, Vec<u64>>,
    x: u64,
}

impl Sieve {
    pub fn new() -> Sieve {
        Sieve {
            composites: HashMap::new(),
            x: 2,
        }
    }
}

impl Iterator for Sieve {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let x = self.x;
        self.x = self.x + 1;
        match self.composites.get(&x) {
            Some(numbers) => {
                for _num in numbers.to_owned() {
                    self.composites
                        .entry(x + _num)
                        .and_modify(|v| v.push(_num))
                        .or_insert(vec![_num]);
                }
                self.composites.remove(&x);
                self.next()
            }
            None => {
                self.composites.insert(x * x, vec![x]);
                Some(x)
            }
        }
    }
}
