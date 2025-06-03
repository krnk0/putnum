/// Variable identifier using 0-based indexing.
///
/// Variables are represented as simple integers, with the first variable
/// being 0, the second being 1, and so on. This is converted from
/// DIMACS format which uses 1-based indexing.
///
/// # Examples
///
/// ```
/// use putnam::types::Var;
/// 
/// let first_var: Var = 0;
/// let second_var: Var = 1;
/// ```
pub type Var = usize;

/// A literal represents a variable or its negation.
///
/// In Boolean satisfiability, a literal is either a variable (positive literal)
/// or the negation of a variable (negative literal).
///
/// # Fields
///
/// * `var` - The variable identifier
/// * `neg` - Whether this literal is negated (`true`) or positive (`false`)
///
/// # Examples
///
/// ```
/// use putnam::types::{Lit, Var};
///
/// let x1 = Lit { var: 0, neg: false };  // Represents x₁
/// let not_x1 = Lit { var: 0, neg: true };   // Represents ¬x₁
/// ```
#[derive(Copy, Clone)]
pub struct Lit {
    /// The variable this literal refers to
    pub var: Var,
    /// Whether this literal is negated
    pub neg: bool,
}

/// A clause is a disjunction (OR) of literals.
///
/// In CNF format, each clause represents a logical OR of its literals.
/// For example, `(x₁ ∨ ¬x₂ ∨ x₃)` would be represented as a vector
/// of three literals.
///
/// # Examples
///
/// ```
/// use putnam::types::{Clause, Lit};
///
/// // Represents (x₁ ∨ ¬x₂)
/// let clause: Clause = vec![
///     Lit { var: 0, neg: false },  // x₁
///     Lit { var: 1, neg: true },   // ¬x₂
/// ];
/// ```
pub type Clause = Vec<Lit>;

/// A formula in Conjunctive Normal Form (CNF).
///
/// A CNF formula is a conjunction (AND) of clauses. The formula is
/// satisfiable if there exists an assignment of truth values to variables
/// that makes all clauses true.
///
/// # Examples
///
/// ```
/// use putnam::types::{Formula, Clause, Lit};
///
/// // Represents (x₁ ∨ x₂) ∧ (¬x₁ ∨ x₃)
/// let formula: Formula = vec![
///     vec![Lit { var: 0, neg: false }, Lit { var: 1, neg: false }],  // (x₁ ∨ x₂)
///     vec![Lit { var: 0, neg: true }, Lit { var: 2, neg: false }],   // (¬x₁ ∨ x₃)
/// ];
/// ```
pub type Formula = Vec<Clause>;

/// The truth value of a variable in the current model.
///
/// During the search process, variables can be assigned `True` or `False`,
/// or remain `Undef` (undefined) if not yet assigned.
///
/// # Examples
///
/// ```
/// use putnam::types::Val;
///
/// let assigned_true = Val::True;
/// let assigned_false = Val::False;
/// let unassigned = Val::Undef;
/// ```
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Val {
    /// Variable is assigned true
    True,
    /// Variable is assigned false
    False,
    /// Variable is not yet assigned
    Undef,
}

/// A model represents the current state of variable assignments.
///
/// The model tracks both the current truth values of all variables and
/// maintains a trail of assignments for efficient backtracking during
/// the DPLL search process.
///
/// # Examples
///
/// ```
/// use putnam::types::{Model, Val};
///
/// let mut model = Model::new(3);  // Create model for 3 variables
/// model.assign(0, Val::True);     // Assign x₁ = true
/// assert_eq!(model.value(0), Val::True);
/// ```
#[derive(Debug, PartialEq)]
pub struct Model {
    /// Current truth value for each variable
    vals: Vec<Val>,
    /// Assignment trail for backtracking (in assignment order)
    trail: Vec<Var>,
}

impl Model {
    /// Creates a new model with `n` variables, all initially undefined.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of variables in the model
    ///
    /// # Returns
    ///
    /// A new `Model` with all variables set to `Val::Undef`
    ///
    /// # Examples
    ///
    /// ```
    /// use putnam::types::{Model, Val};
    ///
    /// let model = Model::new(3);
    /// assert_eq!(model.value(0), Val::Undef);
    /// assert_eq!(model.value(1), Val::Undef);
    /// assert_eq!(model.value(2), Val::Undef);
    /// ```
    pub fn new(n: usize) -> Self {
        Self { vals: vec![Val::Undef; n], trail: Vec::new() }
    }
    /// Gets the current truth value of a variable.
    ///
    /// # Arguments
    ///
    /// * `v` - The variable to query
    ///
    /// # Returns
    ///
    /// The current truth value (`True`, `False`, or `Undef`)
    ///
    /// # Examples
    ///
    /// ```
    /// use putnam::types::{Model, Val};
    ///
    /// let mut model = Model::new(2);
    /// model.assign(0, Val::True);
    /// assert_eq!(model.value(0), Val::True);
    /// assert_eq!(model.value(1), Val::Undef);
    /// ```
    pub fn value(&self, v: Var) -> Val { self.vals[v] }
    /// Assigns a truth value to a variable and records it in the trail.
    ///
    /// This method both sets the variable's value and adds it to the assignment
    /// trail for later backtracking.
    ///
    /// # Arguments
    ///
    /// * `v` - The variable to assign
    /// * `val` - The truth value to assign
    ///
    /// # Examples
    ///
    /// ```
    /// use putnam::types::{Model, Val};
    ///
    /// let mut model = Model::new(2);
    /// model.assign(0, Val::True);
    /// model.assign(1, Val::False);
    /// assert_eq!(model.value(0), Val::True);
    /// assert_eq!(model.value(1), Val::False);
    /// ```
    pub fn assign(&mut self, v: Var, val: Val) {
        self.vals[v] = val;
        self.trail.push(v);
    }
    /// Creates a deep copy of this model.
    ///
    /// This is used during the DPLL search to create independent copies
    /// for exploring different branches of the search tree.
    ///
    /// # Returns
    ///
    /// A new `Model` with the same variable assignments and trail
    ///
    /// # Examples
    ///
    /// ```
    /// use putnam::types::{Model, Val};
    ///
    /// let mut original = Model::new(2);
    /// original.assign(0, Val::True);
    /// 
    /// let copy = original.clone();
    /// assert_eq!(copy.value(0), Val::True);
    /// ```
    pub fn clone(&self) -> Self {
        Self {
            vals: self.vals.clone(),
            trail: self.trail.clone(),
        }
    }
    /// Checks if a literal is satisfied by the current assignment.
    ///
    /// A literal is satisfied if:
    /// - It's a positive literal and the variable is `True`
    /// - It's a negative literal and the variable is `False`
    ///
    /// # Arguments
    ///
    /// * `l` - The literal to check
    ///
    /// # Returns
    ///
    /// `true` if the literal is satisfied, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use putnam::types::{Model, Lit, Val};
    ///
    /// let mut model = Model::new(2);
    /// model.assign(0, Val::True);
    /// model.assign(1, Val::False);
    ///
    /// let pos_lit = Lit { var: 0, neg: false };  // x₁
    /// let neg_lit = Lit { var: 1, neg: true };   // ¬x₂
    ///
    /// assert!(model.is_true(pos_lit));  // x₁ is true
    /// assert!(model.is_true(neg_lit));  // ¬x₂ is true (since x₂ is false)
    /// ```
    pub fn is_true(&self, l: Lit) -> bool {
        match (self.value(l.var), l.neg) {
            (Val::True,  false) | (Val::False, true) => true,
            _                                        => false,
        }
    }
}
