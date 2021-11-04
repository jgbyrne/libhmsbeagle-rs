// Suppress the flurry of warnings caused by using "C" naming conventions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// This matches bindgen::Builder output
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CStr;
use std::mem::transmute;

mod types;
pub use crate::types::*;

pub fn get_version() -> String {
    unsafe { CStr::from_ptr(beagleGetVersion()).to_str().unwrap().to_owned() }
}

pub fn get_citation() -> String {
    unsafe { CStr::from_ptr(beagleGetCitation()).to_str().unwrap().to_owned() }
}

pub fn get_resource_list() -> Vec<Resource> {
    unsafe {
        let res_list = *beagleGetResourceList();
        let slice: &[BeagleResource] = std::slice::from_raw_parts(res_list.list, res_list.length as usize);
        let mut res_vec = vec![];
        for res in slice {
            res_vec.push(Resource {
                name: CStr::from_ptr(res.name).to_str().unwrap().to_owned(),
                description: CStr::from_ptr(res.description).to_str().unwrap().to_owned(),
                supportFlags: Flags::from_bits(res.supportFlags).unwrap_or(Flags::empty()),
                requiredFlags: Flags::from_bits(res.requiredFlags).unwrap_or(Flags::empty()),
            });
        }
        res_vec
    }
}

pub fn create_instance(tipCount: i32, partialsBufferCount: i32,
                       compactBufferCount: i32, stateCount: i32,
                       patternCount: i32, eigenBufferCount: i32,
                       matrixBufferCount: i32, categoryCount: i32,
                       scaleBufferCount: i32,
                       resourceList: Option<Vec<i32>>, 
                       preferenceFlags: Flags,
                       requirementFlags: Flags) -> (i32, InstanceDetails) {
    unsafe {
        let mut returnInfo = BeagleInstanceDetails {
            resourceNumber: 0,
            resourceName: std::ptr::null_mut(),
            implName: std::ptr::null_mut(),
            implDescription: std::ptr::null_mut(),
            flags: 0,
        };

        let (rl, rl_len) = match resourceList {
            Some(mut rs) => (rs.as_mut_ptr(), rs.len() as i32),
            None => (std::ptr::null_mut(), 0),
        };

        // create an instance of the BEAGLE library
        let instance = beagleCreateInstance(
            tipCount, partialsBufferCount, compactBufferCount,
            stateCount, patternCount, eigenBufferCount,
            matrixBufferCount, categoryCount, scaleBufferCount,
            rl, rl_len, preferenceFlags.bits(),
            requirementFlags.bits(), &mut returnInfo);

        (instance, InstanceDetails {
            resourceNumber: returnInfo.resourceNumber,
            resourceName: CStr::from_ptr(returnInfo.resourceName).to_str().unwrap().to_owned(), 
            implName: CStr::from_ptr(returnInfo.implName).to_str().unwrap().to_owned(), 
            implDescription: CStr::from_ptr(returnInfo.implDescription).to_str().unwrap().to_owned(), 
            flags: Flags::from_bits(returnInfo.flags).unwrap_or(Flags::empty()),
        })
    }
}

pub fn finalize() -> ReturnCode {
    unsafe {
        transmute(beagleFinalize() as i8)
    }
}

pub fn finalize_instance(instance: i32) -> ReturnCode {
    unsafe {
        transmute(beagleFinalizeInstance(instance) as i8)
    }
}

pub fn get_partials(instance: i32,
                    bufferIndex: i32,
                    scaleIndex: i32,
                    outPartials: &mut [f64]) -> ReturnCode {
    unsafe {
        transmute(beagleGetPartials(instance, bufferIndex, scaleIndex, outPartials.as_mut_ptr()) as i8)
    }
}

pub fn get_scale_factors(instance: i32,
                         srcScalingIndex: i32,
                         outScaleFactors: &mut [f64]) -> ReturnCode {
    unsafe {
        transmute(beagleGetScaleFactors(instance, srcScalingIndex, outScaleFactors.as_mut_ptr()) as i8)
    }
}

pub fn get_site_derivatives(instance: i32,
                            outFirstDerivatives: &mut [f64],
                            outSecondDerivatives: &mut [f64]) -> ReturnCode {
    unsafe {
        transmute(beagleGetSiteDerivatives(instance,
                                 outFirstDerivatives.as_mut_ptr(),
                                 outSecondDerivatives.as_mut_ptr()) as i8)
    }
}

pub fn get_site_log_likelihoods(instance: i32,
                                outLogLikelihoods: &mut [f64]) -> ReturnCode {
    unsafe {
        transmute(beagleGetSiteLogLikelihoods(instance, outLogLikelihoods.as_mut_ptr()) as i8)
    }
}

pub fn get_transition_matrix(instance: i32,
                             matrixIndex: i32,
                             outMatrix: &mut [f64]) -> ReturnCode {
    unsafe {
        transmute(beagleGetTransitionMatrix(instance, matrixIndex, outMatrix.as_mut_ptr()) as i8)
    }
}

pub fn remove_scale_factors(instance: i32,
                            scaleIndices: &[i32],
                            cumulativeScaleIndex: i32) -> ReturnCode {
    unsafe {
        transmute(beagleRemoveScaleFactors(instance, scaleIndices.as_ptr(),
                                 scaleIndices.len() as i32, cumulativeScaleIndex) as i8)
    }
}

pub fn remove_scale_factors_by_partition(instance: i32,
                                         scaleIndices: &[i32],
                                         cumulativeScaleIndex: i32,
                                         partitionIndex: i32) -> ReturnCode {
    unsafe {
        transmute(beagleRemoveScaleFactorsByPartition(instance,
                                 scaleIndices.as_ptr(),
                                 scaleIndices.len() as i32, 
                                 cumulativeScaleIndex,
                                 partitionIndex) as i8)
    }
}

pub fn reset_scale_factors(instance: i32, cumulativeScaleIndex: i32) -> ReturnCode {
    unsafe {
        transmute(beagleResetScaleFactors(instance, cumulativeScaleIndex) as i8)
    }
}


pub fn reset_scale_factors_by_partition(instance: i32,
                                        cumulativeScaleIndex: i32,
                                        partitionIndex: i32) -> ReturnCode {
    unsafe {
        transmute(beagleResetScaleFactorsByPartition(instance,
                                           cumulativeScaleIndex,
                                           partitionIndex) as i8)
    }
}

pub fn set_category_rates(instance: i32, inCategoryRates: &[f64]) -> ReturnCode {
    unsafe {
        transmute(beagleSetCategoryRates(instance, inCategoryRates.as_ptr()) as i8)
    }
}

pub fn set_category_rates_with_index(instance: i32,
                                     categoryRatesIndex: i32,
                                     inCategoryRates: &[f64]) -> ReturnCode {
    unsafe {
        transmute(beagleSetCategoryRatesWithIndex(instance,
                                        categoryRatesIndex,
                                        inCategoryRates.as_ptr()) as i8)
    }
}

pub fn set_category_weights(instance: i32,
                            categoryWeightsIndex: i32,
                            inCategoryWeights: &[f64]) -> ReturnCode {
    unsafe {
        transmute(beagleSetCategoryWeights(instance,
                                 categoryWeightsIndex,
                                 inCategoryWeights.as_ptr()) as i8)
    }
}

pub fn set_eigen_decomposition(instance: i32,
                               eigenIndex: i32,
                               inEigenVectors: &[f64],
                               inInverseEigenVectors: &[f64],
                               inEigenValues: &[f64]) -> ReturnCode {
    unsafe {
        transmute(beagleSetEigenDecomposition(instance,
                                    eigenIndex,
                                    inEigenVectors.as_ptr(),
                                    inInverseEigenVectors.as_ptr(),
                                    inEigenValues.as_ptr()) as i8)
    }
}

pub fn set_partials(instance: i32,
                    bufferIndex: i32,
                    inPartials: &[f64]) -> ReturnCode {
    unsafe {
        transmute(beagleSetPartials(instance, bufferIndex, inPartials.as_ptr()) as i8)
    }
}

pub fn set_pattern_partitions(instance: i32,
                              partitionCount: i32,
                              inPatternPartitions: &[i32]) -> ReturnCode {
    unsafe {
        transmute(beagleSetPatternPartitions(instance, partitionCount, inPatternPartitions.as_ptr()) as i8)
    }
}

pub fn set_pattern_weights(instance: i32, inPatternWeights: &[f64]) -> ReturnCode {
    unsafe {
        transmute(beagleSetPatternWeights(instance, inPatternWeights.as_ptr()) as i8)
    }
}

pub fn set_state_frequences(instance: i32, stateFrequenciesIndex: i32, inStateFrequencies: &[f64]) -> ReturnCode {
    unsafe {
        transmute(beagleSetStateFrequencies(instance, stateFrequenciesIndex, inStateFrequencies.as_ptr()) as i8)
    }
}

pub fn set_tip_partials(instance: i32, tipIndex: i32, inPartials: &[f64]) -> ReturnCode {
    unsafe {
        transmute(beagleSetTipPartials(instance, tipIndex, inPartials.as_ptr()) as i8)
    }
}

pub fn set_tip_states(instance: i32, tipIndex: i32, inStates: &[i32]) -> ReturnCode {
    unsafe {
        transmute(beagleSetTipStates(instance, tipIndex, inStates.as_ptr()) as i8)
    }
}

pub fn set_transition_matrices(instance: i32,
                               matrixIndices: &[i32],
                               inMatrices: &[f64],
                               paddedValues: &[f64]) -> ReturnCode {
    unsafe {
        transmute(beagleSetTransitionMatrices(instance,
                                    matrixIndices.as_ptr(),
                                    inMatrices.as_ptr(),
                                    paddedValues.as_ptr(),
                                    matrixIndices.len() as i32) as i8)
    }
}

pub fn set_transition_matrix(instance: i32,
                             matrixIndex: i32,
                             inMatrix: &[f64],
                             paddedValue: f64) -> ReturnCode {
    unsafe {
        transmute(beagleSetTransitionMatrix(instance,
                                  matrixIndex,
                                  inMatrix.as_ptr(),
                                  paddedValue) as i8)
    }
}

pub fn update_partials(instance: i32,
                       operations: Vec<Operation>,
                       cumulativeScaleIndex: i32) -> ReturnCode {
    unsafe {
        let mut beagle_ops = vec![];
        for op in operations {
            beagle_ops.push(BeagleOperation {
          	    destinationPartials: op.destinationPartials,
         	    destinationScaleWrite: op.destinationScaleWrite,
         	    destinationScaleRead: op.destinationScaleRead,
         	    child1Partials: op.child1Partials,
         	    child1TransitionMatrix: op.child1TransitionMatrix,
         	    child2Partials: op.child2Partials,
         	    child2TransitionMatrix: op.child2TransitionMatrix,
            });
        }
        transmute(beagleUpdatePartials(instance,
                             beagle_ops.as_slice().as_ptr(),
                             beagle_ops.len() as i32,
                             cumulativeScaleIndex) as i8)
    }
}


pub fn update_partials_by_partition(instance: i32,
                                    operations: Vec<OperationByPartition>) -> ReturnCode {
    unsafe {
        let mut beagle_ops = vec![];
        for op in operations {
            beagle_ops.push(BeagleOperationByPartition {
          	    destinationPartials: op.destinationPartials,
         	    destinationScaleWrite: op.destinationScaleWrite,
         	    destinationScaleRead: op.destinationScaleRead,
         	    child1Partials: op.child1Partials,
         	    child1TransitionMatrix: op.child1TransitionMatrix,
         	    child2Partials: op.child2Partials,
         	    child2TransitionMatrix: op.child2TransitionMatrix,
                partition: op.partition,
                cumulativeScaleIndex: op.cumulativeScaleIndex,
            });
        }
        transmute(beagleUpdatePartialsByPartition(instance,
                             beagle_ops.as_slice().as_ptr(),
                             beagle_ops.len() as i32) as i8)
    }
}

pub fn update_transition_matrices(instance: i32,
                                  eigenIndex: i32,
                                  probabilityIndices: &[i32],
                                  firstDerivativeIndices: Option<&[i32]>,
                                  secondDerivativeIndices: Option<&[i32]>,
                                  edgeLengths: &[f64]) -> ReturnCode {
    unsafe {
        let fdi_ptr = match firstDerivativeIndices {
            Some(slice) => slice.as_ptr(),
            None => std::ptr::null(),
        };

        let sdi_ptr = match secondDerivativeIndices {
            Some(slice) => slice.as_ptr(),
            None => std::ptr::null(),
        };

        transmute(beagleUpdateTransitionMatrices(instance,
                                       eigenIndex,
                                       probabilityIndices.as_ptr(),
                                       fdi_ptr, sdi_ptr,
                                       edgeLengths.as_ptr(),
                                       probabilityIndices.len() as i32) as i8)
    }
}

pub fn update_transition_matrices_with_multiple_models(instance: i32,
                                  eigenIndices: &[i32],
                                  categoryRateIndices: &[i32],
                                  probabilityIndices: &[i32],
                                  firstDerivativeIndices: Option<&[i32]>,
                                  secondDerivativeIndices: Option<&[i32]>,
                                  edgeLengths: &[f64]) -> ReturnCode {
    unsafe {
        let fdi_ptr = match firstDerivativeIndices {
            Some(slice) => slice.as_ptr(),
            None => std::ptr::null(),
        };

        let sdi_ptr = match secondDerivativeIndices {
            Some(slice) => slice.as_ptr(),
            None => std::ptr::null(),
        };

        transmute(beagleUpdateTransitionMatricesWithMultipleModels(instance,
                                       eigenIndices.as_ptr(),
                                       categoryRateIndices.as_ptr(),
                                       probabilityIndices.as_ptr(),
                                       fdi_ptr, sdi_ptr,
                                       edgeLengths.as_ptr(),
                                       probabilityIndices.len() as i32) as i8)
    }
}

pub fn wait_for_partials(instance: i32,
                         destinationPartials: &[i32]) -> ReturnCode {
    unsafe {
        transmute(beagleWaitForPartials(instance, destinationPartials.as_ptr(),
                              destinationPartials.len() as i32) as i8)
    }
}

#[cfg(test)]
mod tests {
}

