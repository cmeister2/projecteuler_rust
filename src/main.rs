fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn problem_1() {
        // Use a filter to only get entries in the range that are divisible by
        // 3 or 5, then sum the remaining entries.
        let result: usize = (0..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
        println!("problem_1: {}", result);
    }
}
