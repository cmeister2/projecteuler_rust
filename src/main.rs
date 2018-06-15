fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use std::fmt::Debug;
    use std::ops::Add;

    #[test]
    fn problem_1() {
        // Use a filter to only get entries in the range that are divisible by
        // 3 or 5, then sum the remaining entries.
        let result: usize = (0..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
        println!("problem_1: {}", result);
    }

    // Problem 2: create a Fibonacci generator.
    #[derive(Debug)]
    struct Fibonacci<T>
    where
        T: Debug,
    {
        curr: T,
        next: T,
    }

    impl<T> Iterator for Fibonacci<T>
    where
        T: Add<Output = T> + Copy + Debug,
    {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            let new_next = self.curr + self.next;
            self.curr = self.next;
            self.next = new_next;
            Some(self.curr)
        }
    }

    impl<T> Fibonacci<T>
    where
        T: Add<Output = T> + Copy + Debug,
    {
        fn new(first: T, second: T) -> Self {
            Fibonacci {
                curr: first,
                next: second,
            }
        }
    }

    #[test]
    fn problem_2() {
        // The sequence starts with the first two terms.
        let f = Fibonacci::new(1, 1);

        // Take elements while the numbers are smaller than 4000000 and then
        // only elements that are even.
        let result: usize = f.take_while(|x| *x <= 4000000).filter(|x| x % 2 == 0).sum();
        println!("problem_2: {}", result);
    }
}
