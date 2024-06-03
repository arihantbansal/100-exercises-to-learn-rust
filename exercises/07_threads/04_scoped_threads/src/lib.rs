use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    let (a, b) = v.split_at(mid);

    thread::scope(|scope| {
        let first_sum = scope.spawn(|| -> i32 { a.iter().sum() });

        let second_sum = scope.spawn(|| -> i32 { b.iter().sum() });

        first_sum.join().unwrap() + second_sum.join().unwrap()
    })
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
