fn main() {
    println!("Hello, rust!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_test() {
        assert_eq!(2 + 2, 4);
    }
}
