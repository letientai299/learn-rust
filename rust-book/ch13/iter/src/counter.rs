#[derive(Debug, Clone)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new(init: u32) -> Counter {
        Counter { count: init }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count() {
        let mut n = 3;
        let mut c = Counter::new(n).into_iter();
        for _ in 1..10 {
            n += 1;
            assert_eq!(c.next().unwrap(), n);
        }
    }

    #[test]
    fn sum() {
        let c1 = Counter::new(0).into_iter();
        let c2 = c1.clone();

        let fold_result = c1
            .take_while(|i| *i <= 10)
            .fold(0, |acc, i| acc + i);


        let sum = c2.take_while(|i| *i <= 10).sum();
        assert_eq!(fold_result, sum);
    }
}

