use crate::cdcl::heuristics::{RestartHeuristic, SolverStats};

#[derive(Debug)]
pub struct Never {}

impl RestartHeuristic for Never {
    fn should_restart(&mut self, _: &SolverStats) -> bool {
        false
    }
}

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

#[derive(Debug)]
pub struct LubyHeuristic {
    pub(crate) max_restarts: usize,
    pub(crate) luby_seq: Vec<usize>,
}

fn luby(i: usize) -> usize {
    let mut k = 1;
    while (1 << k) - 1 < i {
        k += 1;
    }

    if i == (1 << k) - 1 {
        1 << (k - 1)
    } else {
        luby(i - (1 << (k - 1)) + 1)
    }
}

impl LubyHeuristic {
    pub fn new(max_restarts: usize) -> LubyHeuristic {
        LubyHeuristic {
            max_restarts,
            luby_seq: vec![],
        }
    }
    fn get(&self, i: usize) -> usize {
        if let Some(l) = self.luby_seq.get(i - 1) {
            *l
        } else {
            luby(i)
        }
    }
}

impl RestartHeuristic for LubyHeuristic {
    fn should_restart(&mut self, stats: &SolverStats) -> bool {
        stats.number_of_restarts < self.max_restarts
            && 32 * self.get(stats.number_of_restarts + 1) >= stats.conflict_count
    }
}
