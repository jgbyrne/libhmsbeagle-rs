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
pub struct Instance {
    id: i32,
    details: sys::InstanceDetails,
    live: bool,
    n_states: i32,
    n_sites: i32,
    n_rates: i32,
    n_nodes: i32,
    n_tips: i32,
    tip_partials: bool,
    scaling: bool,
    models: Vec<Model>,
}

impl Instance {
    pub fn new(n_states: i32, n_sites: i32, n_rates: i32, n_nodes: i32,
               n_tips: i32, tip_partials: bool, scaling: bool, models: Vec<Model>) -> Instance {

        let n_internals = n_nodes - n_tips;

        let n_partials = if tip_partials { n_nodes } else { n_internals };
        let n_seqs = if tip_partials { 0 } else { n_tips };

        // A buffer for each internal node and a cumulative buffer 
        let n_scalers =  if scaling { n_internals + 1 } else { 0 };

        let prefs = sys::Flags::SCALING_MANUAL | sys::Flags::SCALERS_LOG;

        let (id, details) = sys::create_instance(n_tips,
                                                 n_partials,
                                                 n_seqs,
                                                 n_states,
                                                 n_sites,
                                                 models.len() as i32,
                                                 n_nodes - 1,
                                                 n_rates,
                                                 n_scalers,
                                                 None,
                                                 prefs,
                                                 sys::Flags::empty());

        for i in 0..(models.len()) {
            sys::set_state_frequencies(id, i as i32, &models[i].state_freqs);
            sys::set_eigen_decomposition(id, i as i32, &models[i].eigenvectors,
                                                       &models[i].inv_eigenvectors,
                                                       &models[i].eigenvalues);
            sys::set_category_rates_with_index(id, i as i32, &models[i].category_rates);
            sys::set_category_weights(id, i as i32, &models[i].category_probs);
        }

        sys::set_pattern_weights(id, &(0..n_sites).map(|_| 1.0).collect::<Vec<f64>>()[..]);

        Instance {
            id,
            details,
            live: true,
            n_states,
            n_sites,
            n_rates,
            n_nodes,
            n_tips,
            tip_partials,
            scaling,
            models
        }
    }

    pub fn scaling_buffer(&self, beagle_internal_id: i32) -> i32 {
        if self.scaling {
            if beagle_internal_id < self.n_tips {
                panic!("Tips have no scaling buffers")
            }
            beagle_internal_id - self.n_tips + 1
        }
        else {
            sys::OpCodes::OP_NONE as i32
        }
    }

    pub fn update_matrices(&mut self, model_id: i32, edge_lengths: Vec<f64>) {
        let matrix_idxs = &(0..(self.n_nodes - 1)).map(|i| i as i32).collect::<Vec<i32>>()[..];
        sys::update_transition_matrices(self.id, model_id, matrix_idxs, None, None, &edge_lengths);
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
        sys::update_partials(self.id, ops, if self.scaling { 0 } else { -1 });
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
                let internal_scalers = (1..=n_internals).collect::<Vec<i32>>();
                sys::reset_scale_factors(self.id, cumulative_scale_idx);
                sys::accumulate_scale_factors(self.id, &internal_scalers, cumulative_scale_idx);
                cumulative_scale_idx 
            },
        };

        let mut outSumLogLikelihood: f64 = 666.0;
        sys::calculate_root_log_likelihoods(self.id, &[root_id], &[model_id], &[model_id], &[cumulative_scale_idx], &mut outSumLogLikelihood);
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

