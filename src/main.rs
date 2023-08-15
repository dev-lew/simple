use std::{collections::HashMap, fmt};

type Env = HashMap<String, Expression>;

trait Reducible {
    fn is_reducible(&self) -> bool;
    fn reduce(&self, environment: Env) -> Expression;
}

#[derive(Clone)]
struct Number(i32);

#[derive(Clone)]
enum Boolean {
    True,
    False,
}

#[derive(Clone)]
struct LessThan {
    left: Box<Expression>,
    right: Box<Expression>,
}

#[derive(Clone)]
struct Add {
    left: Box<Expression>,
    right: Box<Expression>,
}

#[derive(Clone)]
struct Multiply {
    left: Box<Expression>,
    right: Box<Expression>,
}

#[derive(Clone)]
struct Variable {
    name: String,
}

#[derive(Clone)]
enum Expression {
    AddExpr(Add),
    MultiplyExpr(Multiply),
    NumberExpr(Number),
    BooleanExpr(Boolean),
    LessThanExpr(LessThan),
    VariableExpr(Variable),
    StrExpr(String),
}

struct Machine {
    expression: Expression,
    environment: Env,
}

impl Machine {
    fn step(&mut self) -> () {
        self.expression = self.expression.reduce(self.environment.clone());
    }

    fn run(&mut self) -> () {
        while self.expression.is_reducible() {
            println!("{}", self.expression);
            self.step()
        }

        println!("{}", self.expression);
    }
}

impl Reducible for Expression {
    fn is_reducible(&self) -> bool {
        match self {
            Expression::AddExpr(_)
            | Expression::MultiplyExpr(_)
            | Expression::LessThanExpr(_)
            | Expression::VariableExpr(_) => true,
            Expression::NumberExpr(_) | Expression::BooleanExpr(_) | Expression::StrExpr(_) => {
                false
            }
        }
    }

    fn reduce(&self, environment: Env) -> Expression {
        match self {
            Expression::AddExpr(x) => Add::reduce(x, environment),
            Expression::MultiplyExpr(x) => Multiply::reduce(x, environment),
            Expression::LessThanExpr(x) => LessThan::reduce(x, environment),
            Expression::VariableExpr(x) => Variable::reduce(x, environment),
            Expression::NumberExpr(x) => Expression::NumberExpr(x.clone()),
            Expression::BooleanExpr(x) => Expression::BooleanExpr(x.clone()),
            Expression::StrExpr(x) => Expression::StrExpr(x.clone()),
            _ => panic!("Not Implemented"),
        }
    }
}

impl Add {
    fn reduce(&self, environment: Env) -> Expression {
        if self.left.is_reducible() {
            Expression::AddExpr(Add {
                left: Box::new(self.left.reduce(environment)),
                right: self.right.clone(),
            })
        } else if self.right.is_reducible() {
            Expression::AddExpr(Add {
                left: self.left.clone(),
                right: Box::new(self.right.reduce(environment)),
            })
        } else {
            match (&*self.left, &*self.right) {
                (Expression::NumberExpr(Number(x)), Expression::NumberExpr(Number(y))) => {
                    Expression::NumberExpr(Number(x + y))
                }
                (_, _) => panic!("Invalid expression"),
            }
        }
    }
}

impl Multiply {
    fn reduce(&self, environment: Env) -> Expression {
        if self.left.is_reducible() {
            Expression::MultiplyExpr(Multiply {
                left: Box::new(self.left.reduce(environment)),
                right: self.right.clone(),
            })
        } else if self.right.is_reducible() {
            Expression::MultiplyExpr(Multiply {
                left: self.left.clone(),
                right: Box::new(self.right.reduce(environment)),
            })
        } else {
            match (&*self.left, &*self.right) {
                (Expression::NumberExpr(Number(x)), Expression::NumberExpr(Number(y))) => {
                    Expression::NumberExpr(Number(x * y))
                }
                (_, _) => panic!("Invalid expression"),
            }
        }
    }
}

impl LessThan {
    fn reduce(&self, environment: Env) -> Expression {
        if self.left.is_reducible() {
            Expression::LessThanExpr(LessThan {
                left: Box::new(self.left.reduce(environment)),
                right: self.right.clone(),
            })
        } else if self.right.is_reducible() {
            Expression::LessThanExpr(LessThan {
                left: self.left.clone(),
                right: Box::new(self.right.reduce(environment)),
            })
        } else {
            match (&*self.left, &*self.right) {
                (Expression::NumberExpr(Number(x)), Expression::NumberExpr(Number(y))) => {
                    Expression::BooleanExpr(Boolean::from(x < y))
                }
                (_, _) => panic!("Invalid expression"),
            }
        }
    }
}

impl Variable {
    fn reduce(&self, environment: Env) -> Expression {
        match environment.get(&self.name).unwrap() {
            Expression::NumberExpr(x) => Expression::NumberExpr(x.clone()),
            Expression::StrExpr(x) => Expression::StrExpr(x.clone()),
            _ => panic!("Invalid variable"),
        }
    }
}
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}", self.left, self.right)
    }
}

impl fmt::Display for Multiply {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} * {}", self.left, self.right)
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}

impl fmt::Display for LessThan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} < {}", self.left, self.right)
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::AddExpr(x) => write!(f, "{}", x),
            Expression::MultiplyExpr(x) => write!(f, "{}", x),
            Expression::NumberExpr(x) => write!(f, "{}", x),
            Expression::LessThanExpr(x) => write!(f, "{}", x),
            Expression::BooleanExpr(x) => write!(f, "{}", x),
            Expression::VariableExpr(x) => write!(f, "{}", x),
            Expression::StrExpr(x) => write! {f, "{}", x},
        }
    }
}

impl From<bool> for Boolean {
    fn from(b: bool) -> Self {
        if b {
            Self::True
        } else {
            Self::False
        }
    }
}

fn main() {
    let a = Expression::AddExpr(Add {
        left: Box::new(Expression::NumberExpr(Number(5))),
        right: Box::new(Expression::MultiplyExpr(Multiply {
            left: Box::new(Expression::NumberExpr(Number(4))),
            right: Box::new(Expression::NumberExpr(Number(4))),
        })),
    });

    let b = Expression::AddExpr(Add {
        left: Box::new(Expression::NumberExpr(Number(5))),
        right: Box::new(Expression::NumberExpr(Number(4))),
    });

    let c = Expression::AddExpr(Add {
        left: Box::new(Expression::NumberExpr(Number(5))),
        right: Box::new(Expression::AddExpr(Add {
            left: Box::new(Expression::NumberExpr(Number(5))),
            right: Box::new(Expression::NumberExpr(Number(6))),
        })),
    });

    let d = Expression::LessThanExpr(LessThan {
        left: Box::new(a),
        right: Box::new(c),
    });

    let x = Variable {
        name: "x".to_string(),
    };
    let y = Variable {
        name: "y".to_string(),
    };

    let mut env: Env = HashMap::new();
    env.insert("x".to_string(), Expression::NumberExpr(Number(3)));
    env.insert("y".to_string(), Expression::NumberExpr(Number(4)));

    let e = Expression::AddExpr(Add {
        left: Box::new(Expression::VariableExpr(x)),
        right: Box::new(Expression::VariableExpr(y)),
    });

    let mut machine = Machine {
        expression: e,
        environment: env,
    };

    machine.run();
}
