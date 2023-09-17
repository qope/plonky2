use plonky2::fri::reduction_strategies::FriReductionStrategy;
use plonky2::fri::{FriConfig, FriParams};

pub struct StarkConfig {
    pub num_columns: usize,

    pub num_public_inputs: usize,

    pub num_constants: usize,

    pub security_bits: usize,

    /// The number of challenge points to generate, for IOPs that have soundness errors of (roughly)
    /// `degree / |F|`.
    pub num_challenges: usize,

    pub fri_config: FriConfig,
}

impl StarkConfig {
    /// A typical configuration with a rate of 2, resulting in fast but large proofs.
    /// Targets ~100 bit conjectured security.
    pub fn standard_fast_config(
        num_columns: usize,
        num_public_inputs: usize,
        num_constants: usize,
    ) -> Self {
        Self {
            num_columns,
            num_public_inputs,
            num_constants,
            security_bits: 100,
            num_challenges: 2,
            fri_config: FriConfig {
                rate_bits: 1,
                cap_height: 4,
                proof_of_work_bits: 16,
                reduction_strategy: FriReductionStrategy::ConstantArityBits(4, 5),
                num_query_rounds: 84,
            },
        }
    }

    pub(crate) fn fri_params(&self, degree_bits: usize) -> FriParams {
        self.fri_config.fri_params(degree_bits, false)
    }
}
