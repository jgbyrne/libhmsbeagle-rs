extern crate beagle;

#[cfg(test)]
mod tests {
    #[test]
    fn simple_calls() {
        println!("{}", beagle::get_version());
        println!("{}", beagle::get_citation());
    }

    #[test]
    fn new_instance() {
        let details = beagle::create_instance(3, 2, 3, 4, 16, 1, 4, 1, 0, beagle::Flags::empty(), beagle::Flags::empty());
        println!("{:?}", details);
    }

    #[test]
    fn resource_list() {
        let resources = beagle::get_resource_list();
        println!("{:?}", resources);
    }

}
