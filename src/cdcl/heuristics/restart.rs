use crate::cdcl::heuristics::{RestartHeuristic, SolverStats};

#[derive(Debug)]
pub struct FixedIntervalHeuristic {
    pub(crate) fixed_restart_policy: usize,
    pub(crate) max_restarts: usize,
}

impl RestartHeuristic for FixedIntervalHeuristic {
    fn should_restart(&mut self, stats: &SolverStats) -> bool {
        stats.number_of_restarts < self.max_restarts
            && stats.conflict_count >= self.fixed_restart_policy
    }
}

#[derive(Debug)]
pub struct GeometricHeuristic {
    pub(crate) threshold: usize,
    pub(crate) max_restarts: usize,
    pub(crate) factor: f32,
}

impl RestartHeuristic for GeometricHeuristic {
    fn should_restart(&mut self, stats: &SolverStats) -> bool {
        if stats.number_of_restarts < self.max_restarts && stats.conflict_count >= self.threshold {
            self.threshold = (self.threshold as f32 * self.factor) as usize;
            return true;
        }
        false
    }
}
