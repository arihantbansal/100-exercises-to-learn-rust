use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let leaked_v = Vec::leak(v);
    let (a, b) = leaked_v.split_at(leaked_v.len() / 2);

    let first_sum = thread::spawn(|| -> i32 { a.iter().sum() });
    let second_sum = thread::spawn(|| -> i32 { b.iter().sum() });

    first_sum.join().unwrap() + second_sum.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
