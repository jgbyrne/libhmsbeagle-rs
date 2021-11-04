extern crate beagle;

#[cfg(test)]
mod tests {
    #[test]
    fn test_beagle() {
        println!("{}", beagle::get_version());
        println!("{}", beagle::get_citation());

        let details = beagle::create_instance(3, 2, 3, 4, 16, 1, 4, 1, 0, beagle::Flags::empty(), beagle::Flags::empty());
        println!("{:?}", details);
        
        let resources = beagle::get_resource_list();
        println!("{:?}", resources);
        
        assert_eq!(0, beagle::finalize());
    }
}
