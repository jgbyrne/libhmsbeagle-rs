extern crate beagle;

#[cfg(test)]
mod tests {
    #[test]
    fn new_instance_test() {
        let details = beagle::create_instance(3, 2, 3, 4, 16, 1, 4, 1, 0, beagle::Flags::empty(), beagle::Flags::empty());
        println!("{:?}", details);
    }
}
