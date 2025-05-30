pub type Var = usize;           // 0-based

#[derive(Copy, Clone)]
pub struct Lit {
    pub var: Var,
    pub neg: bool,
}

pub type Clause  = Vec<Lit>;
pub type Formula = Vec<Clause>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Val { True, False, Undef }

#[derive(Debug, PartialEq)]
pub struct Model {
    vals: Vec<Val>,
    trail: Vec<Var>,
}

impl Model {
    pub fn new(n: usize) -> Self {
        Self { vals: vec![Val::Undef; n], trail: Vec::new() }
    }
    pub fn value(&self, v: Var) -> Val            { self.vals[v] }
    pub fn assign(&mut self, v: Var, val: Val)    {
        self.vals[v] = val;
        self.trail.push(v);
    }
    pub fn clone(&self) -> Self {
        Self {
            vals: self.vals.clone(),
            trail: self.trail.clone(),
        }
    }
    pub fn is_true(&self, l: Lit) -> bool {
        match (self.value(l.var), l.neg) {
            (Val::True,  false) | (Val::False, true) => true,
            _                                        => false,
        }
    }
}
