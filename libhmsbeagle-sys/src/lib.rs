// Suppress the flurry of warnings caused by using "C" naming conventions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// This matches bindgen::Builder output
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn basic_test() {
        unsafe {
            let resourceList = std::ptr::null_mut();

            let mut returnInfo = BeagleInstanceDetails {
                resourceNumber: 0,
                resourceName: std::ptr::null_mut(),
                implName: std::ptr::null_mut(),
                implDescription: std::ptr::null_mut(),
                flags: 0,
            };

            // create an instance of the BEAGLE library
            let instance = beagleCreateInstance(
                        3,             /* Number of tip data elements (input) */
                        2,             /* Number of partials buffers to create (input) -- internal node count */
                        3,             /* Number of compact state representation buffers to create -- for use with setTipStates (input) */
                        4,             /* Number of states in the continuous-time Markov chain (input) -- DNA */
                        16,            /* Number of site patterns to be handled by the instance (input) -- not compressed in this case */
                        1,             /* Number of eigen-decomposition buffers to allocate (input) */
                        4,             /* Number of transition matrix buffers (input) -- one per edge */
                        1,             /* Number of rate categories */
                        0,             /* Number of scaling buffers -- can be zero if scaling is not needed*/
                        resourceList,  /* List of potential resource on which this instance is allowed (input, NULL implies no restriction */
                        0,             /* Length of resourceList list (input) -- not needed to use the default hardware config */
                        0,             /* Bit-flags indicating preferred implementation charactertistics, see BeagleFlags (input) */
                        0,             /* Bit-flags indicating required implementation characteristics, see BeagleFlags (input) */
                        &mut returnInfo
                        );

            assert_eq!(returnInfo.resourceNumber, 0);
        }
    }
}
