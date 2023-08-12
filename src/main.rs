use std::fmt;

trait Reducible {
    fn is_reducible(&self) -> bool;
    fn reduce(&self) -> Expression;
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
enum Expression {
    AddExpr(Add),
    MultiplyExpr(Multiply),
    NumberExpr(Number),
    BooleanExpr(Boolean),
    LessThanExpr(LessThan),
}

struct Machine {
    expression: Expression,
}

impl Machine {
    fn step(&mut self) -> () {
        self.expression = self.expression.reduce();
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
            Expression::AddExpr(_) | Expression::MultiplyExpr(_) | Expression::LessThanExpr(_)=> true,
            Expression::NumberExpr(_) | Expression::BooleanExpr(_) => false,
        }
    }

    fn reduce(&self) -> Expression {
        match self {
            Expression::AddExpr(x) => Add::reduce(x),
            Expression::MultiplyExpr(x) => Multiply::reduce(x),
	    Expression::LessThanExpr(x) => LessThan::reduce(x),
            Expression::NumberExpr(x) => Expression::NumberExpr(x.clone()),
	    Expression::BooleanExpr(x) => Expression::BooleanExpr(x.clone()),
            _ => panic!("Not Implemented"),
        }
    }
}

impl Add {
    fn reduce(&self) -> Expression {
        if self.left.is_reducible() {
            Expression::AddExpr(Add {
                left: Box::new(self.left.reduce()),
                right: self.right.clone(),
            })
        } else if self.right.is_reducible() {
            Expression::AddExpr(Add {
                left: self.left.clone(),
                right: Box::new(self.right.reduce()),
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
    fn reduce(&self) -> Expression {
        if self.left.is_reducible() {
            Expression::MultiplyExpr(Multiply {
                left: Box::new(self.left.reduce()),
                right: self.right.clone(),
            })
        } else if self.right.is_reducible() {
            Expression::MultiplyExpr(Multiply {
                left: self.left.clone(),
                right: Box::new(self.right.reduce()),
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
    fn reduce(&self) -> Expression {
        if self.left.is_reducible() {
            Expression::LessThanExpr(LessThan {
                left: Box::new(self.left.reduce()),
                right: self.right.clone(),
            })
        } else if self.right.is_reducible() {
            Expression::LessThanExpr(LessThan {
                left: self.left.clone(),
                right: Box::new(self.right.reduce()),
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

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::AddExpr(x) => write!(f, "{}", x),
            Expression::MultiplyExpr(x) => write!(f, "{}", x),
            Expression::NumberExpr(x) => write!(f, "{}", x),
	    Expression::LessThanExpr(x) => write!(f, "{}", x),
	    Expression::BooleanExpr(x) => write!(f, "{}", x),
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

    let d = Expression::LessThanExpr(LessThan {left: Box::new(a), right: Box::new(c)});
    let mut machine = Machine { expression: d };
    machine.run();
}
