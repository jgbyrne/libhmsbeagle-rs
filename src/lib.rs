pub extern crate beagle_sys;
mod types;

pub use types::*;
pub use beagle_sys as sys;

#[cfg(test)]
mod tests {
    use crate::sys;

    #[test]
    fn test_get_resources() {
        let resources = sys::get_resource_list();
        println!("{:#?}", resources);
    }
}
