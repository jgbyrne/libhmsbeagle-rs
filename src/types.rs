// =-=-=-=-= types.rs =-=-=-=-=
// This file implements a more idiomatic Rust API for BEAGLE 3
// :: It encapsulates the thin wrapper interface of libhmsbeagle-sys
// :: Implements an RAII style BEAGLE `Instance` type
// :: Provides support for alternate partial/matrix buffers (for trial/revert)

use crate::sys;

#[derive(Debug)]
pub struct Model {
    pub state_freqs: Vec<f64>,

    pub eigenvalues: Vec<f64>,
    pub eigenvectors: Vec<f64>,
    pub inv_eigenvectors: Vec<f64>,

    pub category_rates: Vec<f64>,
    pub category_probs: Vec<f64>,
}

#[derive(Debug)]
pub struct MatrixUpdate {
    pub model_id: i32,
    pub node_id: i32,
    pub edge_length: f64,
}

impl MatrixUpdate {
    pub fn new(model_id: i32, node_id: i32, edge_length: f64) -> Self {
        MatrixUpdate {
            model_id,
            node_id,
            edge_length
        }
    }
}

#[derive(Debug)]
pub struct Alternates {
    // number of buffers before alternates
    n_partials_core: i32,
    n_matrices_core: i32,
    n_scalers_core: i32,

    alt_partials: Vec<bool>, // len n_internals
    alt_matrices: Vec<bool>, // len n_nodes - 1
}

#[derive(Debug)]
pub struct Instance {
    id: i32,
    details: sys::InstanceDetails,
    live: bool,
    n_states: i32,
    n_sites: i32,
    n_rates: i32,
    n_nodes: i32,
    n_tips: i32,

    n_models: i32,

    // number of buffers including alternates
    n_partials: i32,
    n_matrices: i32,
    n_scalers: i32,

    tip_partials: bool,
    scaling: bool,
    alternates: Option<Alternates>,
}

impl Instance {
    pub fn new(n_states: i32, n_sites: i32, n_rates: i32, n_nodes: i32, n_tips: i32, n_models: i32,
               tip_partials: bool, scaling: bool, alternates: bool) -> Instance {

        let n_internals = n_nodes - n_tips;

        let n_partials_core = if tip_partials { n_nodes } else { n_internals };
        let n_partials = if alternates { n_partials_core + n_internals } else { n_partials_core };
        let n_seqs = if tip_partials { 0 } else { n_tips };

        let n_matrices_core = n_nodes - 1;
        let n_matrices = if alternates { n_matrices_core * 2 } else { n_matrices_core };

        // A buffer for each internal node and a cumulative buffer 
        let n_scalers_core =  if scaling { n_internals + 1 } else { 0 };
        let n_scalers = if scaling {
            if alternates { n_scalers_core + n_internals } else { n_scalers_core }
        } else { 0 };

        let prefs = sys::Flags::SCALING_MANUAL | sys::Flags::SCALERS_LOG;

        let (id, details) = sys::create_instance(n_tips,
                                                 n_partials,
                                                 n_seqs,
                                                 n_states,
                                                 n_sites,
                                                 n_models,
                                                 n_matrices,
                                                 n_rates,
                                                 n_scalers,
                                                 None,
                                                 prefs,
                                                 sys::Flags::empty());

        sys::set_pattern_weights(id, &(0..n_sites).map(|_| 1.0).collect::<Vec<f64>>()[..]);

        let alts = match alternates {
            false => None,
            true => {
                Some(Alternates {
                    n_partials_core,
                    n_matrices_core,
                    n_scalers_core,

                    alt_partials: (0..n_internals).map(|_| false).collect::<Vec<bool>>(),
                    alt_matrices: (0..(n_nodes - 1)).map(|_| false).collect::<Vec<bool>>(),
                })
            },
        };

        Instance {
            id,
            details,
            live: true,
            n_states,
            n_sites,
            n_rates,
            n_nodes,
            n_tips,
            n_models,

            n_partials,
            n_matrices,
            n_scalers,

            alternates: alts,

            scaling,
            tip_partials,
        }
    }

    pub fn set_models(&self, models: &Vec<Model>) {
        let num_models = self.n_models as usize;
        assert!(models.len() == num_models);
        for i in 0..num_models {
            sys::set_state_frequencies(self.id, i as i32, &models[i].state_freqs);
            sys::set_eigen_decomposition(self.id, i as i32, &models[i].eigenvectors,
                                                       &models[i].inv_eigenvectors,
                                                       &models[i].eigenvalues);
            sys::set_category_rates_with_index(self.id, i as i32, &models[i].category_rates);
            sys::set_category_weights(self.id, i as i32, &models[i].category_probs);
        }
    }

    pub fn build_operation(&self, parent_id: i32, left_id: i32, right_id: i32) -> sys::Operation {
        sys::Operation {
            destinationPartials: self.partials_buffer(parent_id),
            destinationScaleWrite: self.scaling_buffer(parent_id),
            destinationScaleRead: sys::OpCodes::OP_NONE as i32,
            child1Partials: self.partials_buffer(left_id),
            child1TransitionMatrix: self.matrix_buffer(left_id),
            child2Partials: self.partials_buffer(right_id),
            child2TransitionMatrix: self.matrix_buffer(right_id),
        }
    }

    pub fn partials_buffer(&self, node_id: i32) -> i32 {
        assert!(node_id < self.n_nodes);
        if let Some(alt) = &self.alternates {
            if node_id >= self.n_tips {
                let internal_offset = node_id - self.n_tips;
                if alt.alt_partials[internal_offset as usize] {
                    // |<     n_partials_core    >|
                    // [ n_tips ] [ n_internals ] | [ n_internals ]
                    return alt.n_partials_core + internal_offset;
                }
            }
        }
        node_id
    }

    pub fn matrix_buffer(&self, node_id: i32) -> i32 {
        assert!(node_id < self.n_nodes - 1);
        if let Some(alt) = &self.alternates {
            if alt.alt_matrices[node_id as usize] {
                // n_matrices_core = n_nodes - 1
                return alt.n_matrices_core + node_id;
            }
        }
        node_id
    }

    pub fn scaling_buffer(&self, internal_id: i32) -> i32 {
        assert!(internal_id >= self.n_tips);
        assert!(internal_id < self.n_nodes);
        if self.scaling {
            let internal_offset = internal_id - self.n_tips;
            if let Some(alt) = &self.alternates {
                if alt.alt_partials[internal_offset as usize] {
                    // |<  n_scalers_core >|
                    // [1] [ n_internals ] | [ n_internals ]
                    return alt.n_scalers_core + internal_offset;
                }
            }
            1 + internal_offset 
        }
        else {
            sys::OpCodes::OP_NONE as i32
        }
    }

    pub fn is_alt_partials(&self, node_id: i32) -> bool {
        assert!(node_id < self.n_nodes);
        if node_id >= self.n_tips {
            let alts = self.alternates.as_ref().expect("No alternates");
            alts.alt_partials[(node_id - self.n_tips) as usize]
        }
        else {
            false
        }
    }

    pub fn is_alt_matrix(&self, node_id: i32) -> bool {
        assert!(node_id < self.n_nodes - 1);
        let alts = self.alternates.as_ref().expect("No alternates");
        alts.alt_matrices[node_id as usize]
    }

    pub fn flip_alt_partials(&mut self, node_id: i32) {
        assert!(node_id < self.n_nodes);
        if node_id >= self.n_tips {
            let alts = self.alternates.as_mut().expect("No alternates");
            let idx = (node_id - self.n_tips) as usize; 
            alts.alt_partials[idx] = !alts.alt_partials[idx];
        }
    }

    pub fn flip_alt_matrix(&mut self, node_id: i32) {
        assert!(node_id < self.n_nodes - 1);
        let alts = self.alternates.as_mut().expect("No alternates");
        let idx = node_id as usize;
        alts.alt_matrices[idx] = !alts.alt_matrices[idx];
    }

    pub fn update_matrices(&mut self, updates: Vec<MatrixUpdate>) {
        if updates.len() == 0 { return; }

        let mut matrix_idxs = vec![];
        let mut edge_lengths = vec![];
        let mut model_idxs = vec![];

        let primary_model = updates[0].model_id;
        let mut single_model = true;
        for u in updates {
            if u.model_id != primary_model { single_model = true };
            matrix_idxs.push(self.matrix_buffer(u.node_id));
            edge_lengths.push(u.edge_length);
            model_idxs.push(u.model_id);
        }

        if single_model {
            sys::update_transition_matrices(self.id, primary_model, &matrix_idxs, None, None, &edge_lengths);
        }
        else {
            sys::update_transition_matrices_with_multiple_models(self.id, &model_idxs, &model_idxs, &matrix_idxs, None, None, &edge_lengths);
        }

    }

    pub fn set_tip_data_partial(&mut self, tip_id: i32, partial: Vec<f64>) {
        if self.tip_partials {
            sys::set_tip_partials(self.id, tip_id, &partial);
        }
        else {
            panic!("Cannot set partial data on this instance")
        }
    }

    pub fn set_tip_data_sequence(&mut self, tip_id: i32, sequence: Vec<i32>) {
        if !self.tip_partials {
            sys::set_tip_states(self.id, tip_id, &sequence);
        }
        else {
            panic!("Cannot set sequence data on this instance")
        }
    }

    pub fn perform_operations(&mut self, ops: Vec<sys::Operation>) {
        sys::update_partials(self.id, ops, -1);
    }
    
    pub fn wait_for_partials(&mut self, dests: Vec<i32>) {
        sys::wait_for_partials(self.id, &dests);
    }

    pub fn calculate_root_log_likelihood(&mut self, root_id: i32, model_id: i32) -> f64 {
        let cumulative_scale_idx = match self.scaling {
            false => { sys::OpCodes::OP_NONE as i32 },
            true => {
                let cumulative_scale_idx = 0;
                let n_internals = self.n_nodes - self.n_tips;
                let mut internal_scalers = vec![];
                for i in 0..n_internals {
                    internal_scalers.push(self.scaling_buffer(self.n_tips + i));
                }

                sys::reset_scale_factors(self.id, cumulative_scale_idx);
                sys::accumulate_scale_factors(self.id, &internal_scalers, cumulative_scale_idx);
                cumulative_scale_idx 
            },
        };

        let mut outSumLogLikelihood: f64 = 666.0;
        sys::calculate_root_log_likelihoods(self.id, &[self.partials_buffer(root_id)], &[model_id], &[model_id], &[cumulative_scale_idx], &mut outSumLogLikelihood);
        outSumLogLikelihood
    }

    pub fn instance_id(&self) -> i32 {
        self.id
    }

    pub fn teardown(&mut self) -> sys::ReturnCode {
        if self.live {
            let ret = sys::finalize_instance(self.id);
            self.live = false;
            ret
        }
        else {
            panic!("Cannot teardown dead instance");
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        if self.live { sys::finalize_instance(self.id); }
    }
}

