pub extern crate beagle_sys;
mod types;

pub use types::*;
pub use beagle_sys as sys;

#[cfg(test)]
mod tests {

/*
    use crate::newick;

    #[test]
    fn test_newick() {
        println!("{:?}", newick::parse(String::from("((D:0.2,(F:0.1,G:0.1)E:0.11)B:0.15,C:0.3)A;")));
    }

    #[test]
    fn test_beagle() {
        let (inst, details)= beagle::create_instance(5, //tips
                                           3, //partials
                                           5, //sequences
                                           4, //states
                                           3, //patterns
                                           1, //models
                                           8, //trans matrices
                                           1, //rate categories
                                           0, //scale buffers
                                           None, beagle::Flags::empty(), beagle::Flags::empty());
        println!("Instance: {:?}", inst);
        println!("Details: {:?}", details);

        // A = 0, C = 1, G = 2, T = 3, ? = 4
        //
        let seq0: [i32; 3] = [1, 3, 3];
        let seq1: [i32; 3] = [1, 2, 3];
        let seq2: [i32; 3] = [1, 0, 3];
        let seq3: [i32; 3] = [0, 4, 3];
        let seq4: [i32; 3] = [1, 0, 3];

        beagle::set_tip_states(inst, 0, &seq0);
        beagle::set_tip_states(inst, 1, &seq1);
        beagle::set_tip_states(inst, 2, &seq2);
        beagle::set_tip_states(inst, 3, &seq3);
        beagle::set_tip_states(inst, 4, &seq4);

        let state_freqs: [f64; 4] = [0.25, 0.25, 0.25, 0.25];

        let eigenvalues: [f64; 4] = [-4.0/3.0, -4.0/3.0, -4.0/3.0, 0.0];

        let eigenvectors: [f64;16] = [-1.0, 1.0, 1.0, 1.0,
                                       0.0, 0.0, 1.0, 1.0,
                                       0.0, 1.0, 0.0, 1.0,
                                       1.0, 0.0, 0.0, 1.0];

        let inv_eigenvectors: [f64;16] = [-0.25, -0.25, -0.25,  0.75,
                                          -0.25, -0.25,  0.75, -0.25,
                                          -0.25,  0.75, -0.25, -0.25,
                                           0.25,  0.25,  0.25,  0.25];

        beagle::set_state_frequencies(inst, 0, &state_freqs);
        beagle::set_eigen_decomposition(inst, 0, &eigenvectors, &inv_eigenvectors, &eigenvalues);
        
        let rates: [f64; 1] = [1.0];
        let probs: [f64; 1] = [1.0];
        beagle::set_category_rates_with_index(inst, 0, &rates);
        beagle::set_category_weights(inst, 0, &probs);
        beagle::set_pattern_weights(inst, &[2.0, 1.0, 5.0]);

        beagle::update_transition_matrices(inst, 0, &[0, 1, 2, 3, 4, 5, 6, 7], None, None, &[0.0, 0.23, 0.11, 0.38, 0.10, 0.12, 0.13, 0.15]);

        let op_vec = vec![
        beagle::Operation {
            destinationPartials: 5,
            destinationScaleWrite: -1, //OP_NONE
            destinationScaleRead: -1, //OP_NONE
            child1Partials: 4,
            child1TransitionMatrix: 4,
            child2Partials: 2,
            child2TransitionMatrix: 2,
        },
        beagle::Operation {
            destinationPartials: 6,
            destinationScaleWrite: -1, //OP_NONE
            destinationScaleRead: -1, //OP_NONE
            child1Partials: 1,
            child1TransitionMatrix: 1,
            child2Partials: 5,
            child2TransitionMatrix: 5,
        },
        beagle::Operation {
            destinationPartials: 7,
            destinationScaleWrite: -1, //OP_NONE
            destinationScaleRead: -1, //OP_NONE
            child1Partials: 6,
            child1TransitionMatrix: 6,
            child2Partials: 3,
            child2TransitionMatrix: 3,
        }
        ];

        beagle::update_partials(inst, op_vec, -1);

        let mut outSumLogLikelihood: f64 = 666.0;
        beagle::calculate_root_log_likelihoods(inst, &[7], &[0], &[0], &[-1], &mut outSumLogLikelihood);
        println!("Root Sum Log Likelihood: {}", &outSumLogLikelihood);
        beagle::calculate_edge_log_likelihoods(inst, &[7], &[0], &[0], None, None, &[0], &[0], &[-1], &mut outSumLogLikelihood, None, None);
        println!("Edge Sum Log Likelihood: {}", &outSumLogLikelihood);

        assert_eq!(beagle::ReturnCode::SUCCESS, beagle::finalize());
    }

    #[test]
    fn test_beagle_partial_tips() {

        let human_str: &str = "AGAAATATGTCTGATAAAAGAGTTACTTTGATAGAGTAAATAATAGGAGCTTAAACCCCCTTATTTCTACTAGGACTATGAGAATCGAACCCATCCCTGAGAATCCAAAATTCTCCGTGCCACCTATCACACCCCATCCTAAGTAAGGTCAGCTAAATAAGCTATCGGGCCCATACCCCGAAAATGTTGGTTATACCCTTCCCGTACTAAGAAATTTAGGTTAAATACAGACCAAGAGCCTTCAAAGCCCTCAGTAAGTTG-CAATACTTAATTTCTGTAAGGACTGCAAAACCCCACTCTGCATCAACTGAACGCAAATCAGCCACTTTAATTAAGCTAAGCCCTTCTAGACCAATGGGACTTAAACCCACAAACACTTAGTTAACAGCTAAGCACCCTAATCAAC-TGGCTTCAATCTAAAGCCCCGGCAGG-TTTGAAGCTGCTTCTTCGAATTTGCAATTCAATATGAAAA-TCACCTCGGAGCTTGGTAAAAAGAGGCCTAACCCCTGTCTTTAGATTTACAGTCCAATGCTTCA-CTCAGCCATTTTACCACAAAAAAGGAAGGAATCGAACCCCCCAAAGCTGGTTTCAAGCCAACCCCATGGCCTCCATGACTTTTTCAAAAGGTATTAGAAAAACCATTTCATAACTTTGTCAAAGTTAAATTATAGGCT-AAATCCTATATATCTTA-CACTGTAAAGCTAACTTAGCATTAACCTTTTAAGTTAAAGATTAAGAGAACCAACACCTCTTTACAGTGA";
        let chimp_str: &str = "AGAAATATGTCTGATAAAAGAATTACTTTGATAGAGTAAATAATAGGAGTTCAAATCCCCTTATTTCTACTAGGACTATAAGAATCGAACTCATCCCTGAGAATCCAAAATTCTCCGTGCCACCTATCACACCCCATCCTAAGTAAGGTCAGCTAAATAAGCTATCGGGCCCATACCCCGAAAATGTTGGTTACACCCTTCCCGTACTAAGAAATTTAGGTTAAGCACAGACCAAGAGCCTTCAAAGCCCTCAGCAAGTTA-CAATACTTAATTTCTGTAAGGACTGCAAAACCCCACTCTGCATCAACTGAACGCAAATCAGCCACTTTAATTAAGCTAAGCCCTTCTAGATTAATGGGACTTAAACCCACAAACATTTAGTTAACAGCTAAACACCCTAATCAAC-TGGCTTCAATCTAAAGCCCCGGCAGG-TTTGAAGCTGCTTCTTCGAATTTGCAATTCAATATGAAAA-TCACCTCAGAGCTTGGTAAAAAGAGGCTTAACCCCTGTCTTTAGATTTACAGTCCAATGCTTCA-CTCAGCCATTTTACCACAAAAAAGGAAGGAATCGAACCCCCTAAAGCTGGTTTCAAGCCAACCCCATGACCTCCATGACTTTTTCAAAAGATATTAGAAAAACTATTTCATAACTTTGTCAAAGTTAAATTACAGGTT-AACCCCCGTATATCTTA-CACTGTAAAGCTAACCTAGCATTAACCTTTTAAGTTAAAGATTAAGAGGACCGACACCTCTTTACAGTGA";
        let gorilla_str: &str = "AGAAATATGTCTGATAAAAGAGTTACTTTGATAGAGTAAATAATAGAGGTTTAAACCCCCTTATTTCTACTAGGACTATGAGAATTGAACCCATCCCTGAGAATCCAAAATTCTCCGTGCCACCTGTCACACCCCATCCTAAGTAAGGTCAGCTAAATAAGCTATCGGGCCCATACCCCGAAAATGTTGGTCACATCCTTCCCGTACTAAGAAATTTAGGTTAAACATAGACCAAGAGCCTTCAAAGCCCTTAGTAAGTTA-CAACACTTAATTTCTGTAAGGACTGCAAAACCCTACTCTGCATCAACTGAACGCAAATCAGCCACTTTAATTAAGCTAAGCCCTTCTAGATCAATGGGACTCAAACCCACAAACATTTAGTTAACAGCTAAACACCCTAGTCAAC-TGGCTTCAATCTAAAGCCCCGGCAGG-TTTGAAGCTGCTTCTTCGAATTTGCAATTCAATATGAAAT-TCACCTCGGAGCTTGGTAAAAAGAGGCCCAGCCTCTGTCTTTAGATTTACAGTCCAATGCCTTA-CTCAGCCATTTTACCACAAAAAAGGAAGGAATCGAACCCCCCAAAGCTGGTTTCAAGCCAACCCCATGACCTTCATGACTTTTTCAAAAGATATTAGAAAAACTATTTCATAACTTTGTCAAGGTTAAATTACGGGTT-AAACCCCGTATATCTTA-CACTGTAAAGCTAACCTAGCGTTAACCTTTTAAGTTAAAGATTAAGAGTATCGGCACCTCTTTGCAGTGA";

        let partial_seq = |seq: &str| {
            let mut partials: Vec<f64> = vec![];
            for c in seq.chars() {
                match c {
                    'A' => partials.extend_from_slice(&[1.0,0.0,0.0,0.0]),
                    'C' => partials.extend_from_slice(&[0.0,1.0,0.0,0.0]),
                    'G' => partials.extend_from_slice(&[0.0,0.0,1.0,0.0]),
                    'T' => partials.extend_from_slice(&[0.0,0.0,0.0,1.0]),
                    _   => partials.extend_from_slice(&[1.0,1.0,1.0,1.0]),
                }
            }
            partials
        };
        let (inst, details) = beagle::create_instance(3, //tips
                                               5, //partials
                                               0, //sequences
                                               4, //states
                                               human_str.len() as i32, //patterns
                                               1, //models
                                               4, //trans matrices
                                               4, //rate categories
                                               0, //scale buffers
                                               None, beagle::Flags::empty(), beagle::Flags::empty());
        println!("Instance: {:?}", inst);
        println!("Details: {:?}", details);

        beagle::set_tip_partials(inst, 0, &partial_seq(human_str)[..]);
        beagle::set_tip_partials(inst, 1, &partial_seq(chimp_str)[..]);
        beagle::set_tip_partials(inst, 2, &partial_seq(gorilla_str)[..]);

        let state_freqs: [f64; 4] = [0.25, 0.25, 0.25, 0.25];

        let eigenvalues: [f64; 4] = [0.0, -1.3333333333333333, -1.3333333333333333, -1.3333333333333333];

        let eigenvectors: [f64;16] = [
                1.0,  2.0,  0.0,  0.5,
                1.0,  -2.0,  0.5,  0.0,
                1.0,  2.0, 0.0,  -0.5,
                1.0,  -2.0,  -0.5,  0.0
        ];

        let inv_eigenvectors: [f64;16] = [
                0.25,  0.25,  0.25,  0.25,
                0.125,  -0.125,  0.125,  -0.125,
                0.0,  1.0,  0.0,  -1.0,
                1.0,  0.0,  -1.0,  0.0
        ];

        beagle::set_state_frequencies(inst, 0, &state_freqs);
        beagle::set_eigen_decomposition(inst, 0, &eigenvectors, &inv_eigenvectors, &eigenvalues);
        
        let rates: [f64; 4] = [0.03338775, 0.25191592, 0.82026848, 2.89442785];
        let probs: [f64; 4] = [0.25, 0.25, 0.25, 0.25];
        beagle::set_category_rates_with_index(inst, 0, &rates);
        beagle::set_category_weights(inst, 0, &probs);
        beagle::set_pattern_weights(inst, &(0..human_str.len()).map(|_| 1.0).collect::<Vec<f64>>()[..]);

        beagle::update_transition_matrices(inst, 0, &[0, 1, 2, 3], None, None, &[0.1, 0.1, 0.2, 0.1]);

        let op_vec = vec![
        beagle::Operation {
            destinationPartials: 3,
            destinationScaleWrite: -1, //OP_NONE
            destinationScaleRead: -1, //OP_NONE
            child1Partials: 0,
            child1TransitionMatrix: 0,
            child2Partials: 1,
            child2TransitionMatrix: 1,
        },
        beagle::Operation {
            destinationPartials: 4,
            destinationScaleWrite: -1, //OP_NONE
            destinationScaleRead: -1, //OP_NONE
            child1Partials: 2,
            child1TransitionMatrix: 2,
            child2Partials: 3,
            child2TransitionMatrix: 3,
        }
        ];

        beagle::update_partials(inst, op_vec, -1);

        let mut outSumLogLikelihood: f64 = 666.0;
        beagle::calculate_root_log_likelihoods(inst, &[4], &[0], &[0], &[-1], &mut outSumLogLikelihood);
        println!("Root Sum Log Likelihood: {}", &outSumLogLikelihood);
        //beagle::calculate_edge_log_likelihoods(inst, &[7], &[0], &[0], None, None, &[0], &[0], &[-1], &mut outSumLogLikelihood, None, None);
        //println!("Edge Sum Log Likelihood: {}", &outSumLogLikelihood);

        assert_eq!(beagle::ReturnCode::SUCCESS, beagle::finalize());
    }
*/
}
