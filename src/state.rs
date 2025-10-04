use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Debug, Clone, Copy)]
pub struct Amount {
    pub units: u32,
    pub fractions: u32,
}

impl Amount {
    pub fn new_u(units: u32) -> Self {
        Amount {
            units,
            fractions: 0,
        }
    }

    pub fn new(units: u32, fractions: u32) -> Self {
        Amount { units, fractions }
    }
}

#[derive(Debug)]
pub struct Connection {
    pub r1: usize,
    pub g1: usize,
    pub r2: usize,
    pub g2: usize,
    pub weight: f64,
}

impl Connection {
    pub fn new(r1: usize, g1: usize, r2: usize, g2: usize, weight: f64) -> Self {
        Connection {
            r1,
            g1,
            r2,
            g2,
            weight,
        }
    }
}

#[derive(Debug)]
pub struct Group {
    pub free: Vec<Amount>,
    pub request: Amount,
}

#[derive(Debug)]
pub struct State {
    pub groups: Vec<Group>,
    pub connections: Vec<Connection>,
}

pub fn create_empty_4() -> State {
    State {
        groups: vec![Group {
            free: vec![Amount::new_u(4); 4],
            request: Amount::new_u(7),
        }],
        connections: Vec::new(),
    }
}

pub fn create_empty_16() -> State {
    State {
        groups: vec![Group {
            free: vec![Amount::new_u(24); 16],
            request: Amount::new_u(100),
        }],
        connections: Vec::new(),
    }
}

pub fn create_empty_8_8() -> State {
    State {
        groups: vec![
            Group {
                free: vec![Amount::new_u(24); 8],
                request: Amount::new_u(12),
            },
            Group {
                free: vec![Amount::new_u(32); 8],
                request: Amount::new_u(74),
            },
        ],
        connections: Vec::new(),
    }
}

pub fn create_empty_8_8_8() -> State {
    State {
        groups: vec![
            Group {
                free: vec![Amount::new_u(24); 8],
                request: Amount::new_u(48),
            },
            Group {
                free: vec![Amount::new_u(32); 8],
                request: Amount::new_u(74),
            },
            Group {
                free: vec![Amount::new_u(4); 8],
                request: Amount::new_u(30),
            },
        ],
        connections: Vec::new(),
    }
}

pub fn create_empty_32() -> State {
    State {
        groups: vec![Group {
            free: vec![Amount::new_u(120); 32],
            request: Amount::new_u(921),
        }],
        connections: Vec::new(),
    }
}

pub fn create_onlyo_16() -> State {
    State {
        groups: vec![Group {
            free: vec![
                Amount::new_u(4),
                Amount::new_u(0),
                Amount::new_u(2),
                Amount::new_u(1),
                Amount::new_u(4),
                Amount::new_u(0),
                Amount::new_u(2),
                Amount::new_u(1),
                Amount::new_u(0),
                Amount::new_u(0),
                Amount::new_u(0),
                Amount::new_u(0),
                Amount::new_u(3),
                Amount::new_u(2),
                Amount::new_u(3),
                Amount::new_u(2),
            ],
            request: Amount::new_u(24),
        }],
        connections: Vec::new(),
    }
}

pub fn create_onlyo_4() -> State {
    State {
        groups: vec![Group {
            free: vec![
                Amount::new_u(1),
                Amount::new_u(0),
                Amount::new_u(2),
                Amount::new_u(1),
            ],
            request: Amount::new_u(7),
        }],
        connections: Vec::new(),
    }
}

pub fn create_onlyo_32() -> State {
    State {
        groups: vec![Group {
            free: vec![
                Amount::new_u(4),
                Amount::new_u(0),
                Amount::new_u(2),
                Amount::new_u(1),
                Amount::new_u(4),
                Amount::new_u(0),
                Amount::new_u(2),
                Amount::new_u(1),
                Amount::new_u(0),
                Amount::new_u(0),
                Amount::new_u(0),
                Amount::new_u(0),
                Amount::new_u(3),
                Amount::new_u(2),
                Amount::new_u(3),
                Amount::new_u(2),
                Amount::new_u(4),
                Amount::new_u(0),
                Amount::new_u(2),
                Amount::new_u(1),
                Amount::new_u(4),
                Amount::new_u(0),
                Amount::new_u(2),
                Amount::new_u(1),
                Amount::new_u(0),
                Amount::new_u(0),
                Amount::new_u(0),
                Amount::new_u(0),
                Amount::new_u(3),
                Amount::new_u(2),
                Amount::new_u(3),
                Amount::new_u(2),
            ],
            request: Amount::new_u(48),
        }],
        connections: Vec::new(),
    }
}

pub fn create_primes_16() -> State {
    State {
        groups: vec![Group {
            free: vec![
                Amount::new_u(2),
                Amount::new_u(3),
                Amount::new_u(5),
                Amount::new_u(7),
                Amount::new_u(9),
                Amount::new_u(11),
                Amount::new_u(13),
                Amount::new_u(17),
                Amount::new_u(19),
                Amount::new_u(23),
                Amount::new_u(29),
                Amount::new_u(31),
                Amount::new_u(37),
                Amount::new_u(39),
                Amount::new_u(41),
            ],
            request: Amount::new_u(113),
        }],
        connections: Vec::new(),
    }
}

pub fn create_primes_16_n() -> State {
    State {
        groups: vec![Group {
            free: vec![
                Amount::new_u(2),
                Amount::new_u(3),
                Amount::new_u(5),
                Amount::new_u(7),
                Amount::new_u(9),
                Amount::new_u(11),
                Amount::new_u(13),
                Amount::new_u(17),
                Amount::new_u(19),
                Amount::new_u(23),
                Amount::new_u(29),
                Amount::new_u(31),
                Amount::new_u(37),
                Amount::new_u(39),
                Amount::new_u(41),
            ],
            request: Amount::new_u(329),
        }],
        connections: Vec::new(),
    }
}

pub fn create_triplets_16() -> State {
    State {
        groups: vec![Group {
            free: vec![
                Amount::new_u(40),
                Amount::new_u(41),
                Amount::new_u(42),
                Amount::new_u(40),
                Amount::new_u(41),
                Amount::new_u(42),
                Amount::new_u(39),
                Amount::new_u(40),
                Amount::new_u(41),
                Amount::new_u(43),
                Amount::new_u(44),
                Amount::new_u(45),
                Amount::new_u(38),
                Amount::new_u(39),
                Amount::new_u(40),
            ],
            request: Amount::new_u(399),
        }],
        connections: Vec::new(),
    }
}

pub fn create_one2one_12_12() -> State {
    State {
        groups: vec![
            Group {
                free: vec![
                    Amount::new_u(4),
                    Amount::new_u(4),
                    Amount::new_u(4),
                    Amount::new_u(4),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(2),
                    Amount::new_u(5),
                    Amount::new_u(1),
                ],
                request: Amount::new_u(7),
            },
            Group {
                free: vec![
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(1),
                    Amount::new_u(0),
                    Amount::new_u(0),
                    Amount::new_u(1),
                ],
                request: Amount::new_u(7),
            },
        ],
        connections: (0..12)
            .map(|i| Connection::new(0, i, 1, i, 256.0))
            .collect(),
    }
}

pub fn create_one2one_8_8_8() -> State {
    State {
        groups: vec![
            Group {
                free: vec![
                    Amount::new_u(4),
                    Amount::new_u(4),
                    Amount::new_u(4),
                    Amount::new_u(4),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                ],
                request: Amount::new_u(7),
            },
            Group {
                free: vec![
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                ],
                request: Amount::new_u(7),
            },
            Group {
                free: vec![
                    Amount::new_u(2),
                    Amount::new_u(6),
                    Amount::new_u(2),
                    Amount::new_u(3),
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(3),
                    Amount::new_u(3),
                ],
                request: Amount::new_u(12),
            },
        ],
        connections: (0..8)
            .map(|i| Connection::new(0, i, 1, i, 256.0))
            .chain((0..8).map(|i| Connection::new(1, i, 2, i, 256.0)))
            .collect(),
    }
}

pub fn create_all2all_16_16() -> State {
    State {
        groups: vec![
            Group {
                free: vec![
                    Amount::new_u(1),
                    Amount::new_u(4),
                    Amount::new_u(3),
                    Amount::new_u(4),
                    Amount::new_u(5),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(1),
                    Amount::new_u(4),
                    Amount::new_u(3),
                    Amount::new_u(4),
                    Amount::new_u(5),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                ],
                request: Amount::new_u(14),
            },
            Group {
                free: vec![
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(0),
                    Amount::new_u(0),
                    Amount::new_u(0),
                    Amount::new_u(5),
                    Amount::new_u(1),
                    Amount::new_u(3),
                    Amount::new_u(1),
                    Amount::new_u(1),
                ],
                request: Amount::new_u(9),
            },
        ],
        connections: (0..16)
            .flat_map(|i| {
                (0..i)
                    .map(move |j| Connection::new(0, i, 1, j, ((i + j) % 2) as f64 * 128.0 + 128.0))
            })
            .collect(),
    }
}

pub fn create_stairs_32_16_8() -> State {
    State {
        groups: vec![
            Group {
                free: (0..32).map(|i| Amount::new_u(i)).collect(),
                request: Amount::new_u(127),
            },
            Group {
                free: vec![
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(0),
                    Amount::new_u(0),
                    Amount::new_u(0),
                    Amount::new_u(5),
                    Amount::new_u(1),
                    Amount::new_u(3),
                    Amount::new_u(1),
                    Amount::new_u(1),
                ],
                request: Amount::new_u(19),
            },
            Group {
                free: vec![
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(2),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(3),
                    Amount::new_u(5),
                ],
                request: Amount::new_u(10),
            },
        ],
        connections: (0..16)
            .flat_map(|i| (0..1).map(move |j| Connection::new(0, i * 2 + j, 1, i, 256.0)))
            .chain(
                (0..8)
                    .flat_map(|i| (0..1).map(move |j| Connection::new(1, i * 2 + j, 2, i, 348.0))),
            )
            .collect(),
    }
}

pub fn create_diamonds_32_16_32() -> State {
    State {
        groups: vec![
            Group {
                free: (0..32).map(|i| Amount::new_u(4)).collect(),
                request: Amount::new_u(7),
            },
            Group {
                free: (0..32).map(|i| Amount::new_u(4)).collect(),
                request: Amount::new_u(7),
            },
            Group {
                free: (0..32).map(|i| Amount::new_u(4)).collect(),
                request: Amount::new_u(7),
            },
        ],
        connections: (0..16)
            .flat_map(|i| (0..1).map(move |j| Connection::new(0, i * 2 + j, 1, i, 256.0)))
            .chain(
                (0..16)
                    .flat_map(|i| (0..1).map(move |j| Connection::new(2, i * 2 + j, 1, i, 348.0))),
            )
            .collect(),
    }
}

pub fn create_random_18_18(seed: u64, p: f64) -> State {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut connections = Vec::new();
    for i in 0..18 {
        for j in 0..i {
            if rng.random_bool(p) {
                connections.push(Connection::new(
                    0,
                    i,
                    1,
                    j,
                    64.0 * rng.random_range(1..6) as f64,
                ))
            }
        }
    }
    State {
        groups: vec![
            Group {
                free: (0..18)
                    .map(|i| Amount::new_u(rng.random_range(1..=4)))
                    .collect(),
                request: Amount::new_u(12),
            },
            Group {
                free: (0..18)
                    .map(|i| Amount::new_u(rng.random_range(1..=4)))
                    .collect(),
                request: Amount::new_u(12),
            },
        ],
        connections,
    }
}


pub fn create_empty_f_16() -> State {
    State {
        groups: vec![Group {
            free: vec![Amount::new_u(24); 8],
            request: Amount::new(100, 500),
        }],
        connections: Vec::new(),
    }
}

pub fn create_fractions_16() -> State {
    State {
        groups: vec![Group {
            free: vec![
                Amount::new(2, 5000), // 0
                Amount::new(1, 0),
                Amount::new(2, 5500),
                Amount::new(1, 2500),
                Amount::new(1, 3000), // 4
                Amount::new(0, 3600),
                Amount::new(8, 0),
                Amount::new(1, 6500),
                Amount::new(2, 5000), // 8
                Amount::new(1, 0),
                Amount::new(2, 9000),
                Amount::new(1, 2500),
                Amount::new(1, 3000), // 12
                Amount::new(0, 3500),
                Amount::new(0, 9500),
                Amount::new(1, 4500),

            ],
            request: Amount::new(12, 3500),
        }],
        connections: Vec::new(),
    }
}

pub fn create_random_f_18_18(seed: u64, p: f64) -> State {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut connections = Vec::new();
    for i in 0..18 {
        for j in 0..i {
            if rng.random_bool(p) {
                connections.push(Connection::new(
                    0,
                    i,
                    1,
                    j,
                    64.0 * rng.random_range(1..6) as f64,
                ))
            }
        }
    }
    State {
        groups: vec![
            Group {
                free: (0..18)
                    .map(|i| Amount::new(rng.random_range(1..=4), rng.random_range(0..10_000)))
                    .collect(),
                request: Amount::new(12, 4999),
            },
            Group {
                free: (0..18)
                    .map(|i| Amount::new_u(rng.random_range(1..=4)))
                    .collect(),
                request: Amount::new(12, 6010),
            },
        ],
        connections,
    }
}

