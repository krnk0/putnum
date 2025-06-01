use criterion::{black_box, criterion_group, criterion_main, Criterion};
use putnam::{solve, types::*};

fn create_simple_sat() -> (Formula, usize) {
    // (x0 ∨ x1) ∧ (¬x0 ∨ x2) ∧ (¬x1 ∨ ¬x2)
    let formula = vec![
        vec![Lit { var: 0, neg: false }, Lit { var: 1, neg: false }],
        vec![Lit { var: 0, neg: true }, Lit { var: 2, neg: false }],
        vec![Lit { var: 1, neg: true }, Lit { var: 2, neg: true }],
    ];
    (formula, 3)
}

fn create_pigeonhole(n: usize) -> (Formula, usize) {
    // n+1 pigeons, n holes - classically UNSAT
    let mut formula = Vec::new();
    let num_vars = (n + 1) * n;
    
    // Each pigeon must be in at least one hole
    for pigeon in 0..=n {
        let clause: Vec<Lit> = (0..n)
            .map(|hole| Lit { var: pigeon * n + hole, neg: false })
            .collect();
        formula.push(clause);
    }
    
    // No two pigeons in same hole
    for hole in 0..n {
        for p1 in 0..=n {
            for p2 in (p1 + 1)..=n {
                formula.push(vec![
                    Lit { var: p1 * n + hole, neg: true },
                    Lit { var: p2 * n + hole, neg: true },
                ]);
            }
        }
    }
    
    (formula, num_vars)
}

fn create_chain_sat(n: usize) -> (Formula, usize) {
    // (x0 ∨ x1) ∧ (¬x0 ∨ x2) ∧ (¬x1 ∨ x3) ∧ ... - chain of implications
    let mut formula = Vec::new();
    
    formula.push(vec![
        Lit { var: 0, neg: false },
        Lit { var: 1, neg: false },
    ]);
    
    for i in 0..(n - 2) {
        formula.push(vec![
            Lit { var: i, neg: true },
            Lit { var: i + 2, neg: false },
        ]);
        formula.push(vec![
            Lit { var: i + 1, neg: true },
            Lit { var: i + 2, neg: false },
        ]);
    }
    
    (formula, n)
}

fn bench_simple_sat(c: &mut Criterion) {
    let (formula, num_vars) = create_simple_sat();
    
    c.bench_function("simple_3var_sat", |b| {
        b.iter(|| solve(black_box(&formula), black_box(num_vars)))
    });
}

fn bench_pigeonhole(c: &mut Criterion) {
    let mut group = c.benchmark_group("pigeonhole");
    
    for n in [3, 4, 5].iter() {
        let (formula, num_vars) = create_pigeonhole(*n);
        group.bench_with_input(format!("php_{}_{}", n + 1, n), n, |b, _| {
            b.iter(|| solve(black_box(&formula), black_box(num_vars)))
        });
    }
    
    group.finish();
}

fn bench_chain_sat(c: &mut Criterion) {
    let mut group = c.benchmark_group("chain");
    
    for n in [10, 20, 30].iter() {
        let (formula, num_vars) = create_chain_sat(*n);
        group.bench_with_input(format!("chain_{}", n), n, |b, _| {
            b.iter(|| solve(black_box(&formula), black_box(num_vars)))
        });
    }
    
    group.finish();
}

criterion_group!(benches, bench_simple_sat, bench_pigeonhole, bench_chain_sat);
criterion_main!(benches);