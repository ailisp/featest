#[cfg(test)]
mod tests {
    #[test]
    fn should_not_panic() {
        #[cfg(feature = "foo")]
        panic!("should not foo");
        println!("aaa");
    }
}
