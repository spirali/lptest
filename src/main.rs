mod solver;
mod state;

use crate::solver::{solve, HqSolution};
use crate::state::{create_diamonds_32_16_32, create_empty_4, create_empty_8_8, create_empty_8_8_8, create_empty_16, create_empty_32, create_one2one_8_8_8, create_one2one_12_12, create_onlyo_4, create_onlyo_16, create_onlyo_32, create_primes_16, create_primes_16_n, create_stairs_32_16_8, create_triplets_16, create_random_18_18, State};
use clap::{Parser, ValueEnum};
use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variable, variables};
use microlp::{ComparisonOp, OptimizationDirection, Problem};

#[derive(ValueEnum, Copy, Clone, Debug)]
enum SolverType {
    #[clap(name = "highs")]
    Highs,
    #[clap(name = "microlp")]
    Microlp,
}

#[derive(Parser)]
struct Opts {
    solver: SolverType,
    benchmark: String,

    #[clap(long)]
    repeats: Option<usize>,
}

fn run_solver(state: &State, solver_type: SolverType) -> Option<HqSolution> {
    match solver_type {
        SolverType::Highs => solve(state, good_lp::solvers::highs::highs),
        SolverType::Microlp => solve(state, good_lp::solvers::microlp::microlp),
    }
}


fn main() -> Result<(), String> {
    let opts = Opts::parse();
    let state = match opts.benchmark.as_str() {
        "empty_4" => create_empty_4(),
        "empty_16" => create_empty_16(),
        "empty_32" => create_empty_32(),
        "empty_8_8" => create_empty_8_8(),
        "empty_8_8_8" => create_empty_8_8_8(),
        "onlyo_4" => create_onlyo_4(),
        "onlyo_16" => create_onlyo_16(),
        "onlyo_32" => create_onlyo_32(),
        "primes_16" => create_primes_16(),
        "primes_16_n" => create_primes_16_n(),
        "triplets_16" => create_triplets_16(),
        "one2one_12_12" => create_one2one_12_12(),
        "one2one_8_8_8" => create_one2one_8_8_8(),
        "all2all_16_16" => create_empty_16(),
        "stairs_32_16_8" => create_stairs_32_16_8(),
        "diamonds_16_32_16" => create_diamonds_32_16_32(),
        "random_18_18_v1" => create_random_18_18(1234, 0.5),
        "random_18_18_v2" => create_random_18_18(4242424242, 0.2),
        "random_18_18_v3" => create_random_18_18(8888881, 0.8),
        _ => return Err(format!("Invalid benchmark name: {}", opts.benchmark)),
    };

    if let Some(repeats) = opts.repeats {
        let mut dummy:f64 = 0.0;
        for _ in 0..repeats {
            let solution = run_solver(&state, opts.solver);
            if let Some(solution) = solution {
                dummy += solution.objective;
            }
        }
        println!("dummy {}", dummy);
    } else {
        let solution = run_solver(&state, opts.solver);

        match solution {
            Some(solution) => {
                println!("Solution: {}", solution.objective);
                println!("Groups: {:?}", solution.groups);
            }
            None => {
                println!("No solution found");
            }
        }
    }
    Ok(())
}
