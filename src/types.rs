pub type Var = usize;           // 0-based

#[derive(Copy, Clone)]
pub struct Lit {
    pub var: Var,
    pub neg: bool,
}

pub type Clause  = Vec<Lit>;
pub type Formula = Vec<Clause>;

#[derive(Copy, Clone, PartialEq)]
pub enum Val { True, False, Undef }

pub struct Model(pub Vec<Val>);

impl Model {
    pub fn new(n: usize) -> Self { Self(vec![Val::Undef; n]) }
    pub fn assign(&mut self, v: Var, val: Val) { self.0[v] = val }
    /* … first_undef, is_complete などもここに … */
}
