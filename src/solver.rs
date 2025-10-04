use crate::state::State;
use good_lp::{Expression, Solution, Solver, SolverModel, constraint, variable, variables};

pub struct HqSolution {
    pub groups: Vec<Vec<usize>>,
    pub objective: f64,
}

pub fn solve(state: &State, solver: impl Solver) -> Option<HqSolution> {
    let mut variables = variables!();
    let mut obj: Expression = 0.into();

    let gvars: Vec<Vec<_>> = state
        .groups
        .iter()
        .map(|g| {
            let rf = g.request.fractions;
            g.free
                .iter()
                .map(|free| {
                    let v = variables.add(variable().binary());
                    if rf == 0 {
                        obj.add_mul(-1024.0 - (free.units as f64 / 32.0), v);
                    } else {
                        let f = free.fractions;
                        if f >= rf {
                            obj.add_mul(-1024.0 - (f as f64 / 10_000.0 / 16.0), v);
                        } else {
                            obj.add_mul(-1024.0, v);
                        }
                    }
                    v
                })
                .collect()
        })
        .collect();

    let conn_vars: Vec<_> = state
        .connections
        .iter()
        .map(|c| {
            let v = variables.add(variable());
            obj.add_mul(c.weight, v);
            v
        })
        .collect();

    let mut p = variables.maximise(&obj).using(solver);

    for (group, vars) in state.groups.iter().zip(gvars.iter()) {
        if group.request.units > 0 {
            let mut cst: Expression = 0.into();
            for (c, v) in group.free.iter().zip(vars.iter()) {
                cst.add_mul(c.units as f64, *v);
            }
            p.add_constraint(constraint!(cst >= group.request.units as f64));
        }
        let rf = group.request.fractions;
        if rf > 0 {
            let mut cst: Expression = 0.into();
            for (c, v) in group.free.iter().zip(vars.iter()) {
                let units = if c.fractions >= rf {
                    c.units + 1
                } else {
                    c.units
                };
                cst.add_mul(units as f64, *v);
            }
            p.add_constraint(constraint!(cst >= (group.request.units + 1) as f64));
        }
    }

    for (conn, conn_var) in state.connections.iter().zip(conn_vars.iter()) {
        let v1 = gvars[conn.r1][conn.g1];
        let v2 = gvars[conn.r2][conn.g2];
        p.add_constraint(constraint!(*conn_var <= v1));
        p.add_constraint(constraint!(*conn_var <= v2));
    }

    let solution = p.solve().ok()?;
    let objective = solution.eval(obj);

    let result: Vec<_> = gvars
        .iter()
        .map(|vars| {
            vars.iter()
                .enumerate()
                .filter_map(|(i, v)| (solution.value(*v) > 0.5).then(|| i))
                .collect::<Vec<usize>>()
        })
        .collect();

    Some(HqSolution {
        groups: result,
        objective,
    })
}
