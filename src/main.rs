use std::fmt;

trait Reducible {
    fn is_reducible(&self) -> bool;
    fn reduce(&self) -> Expression;
}

#[derive(Clone)]
struct Number(i32);

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
            Expression::AddExpr(_) | Expression::MultiplyExpr(_) => true,
            Expression::NumberExpr(_) => false,
        }
    }

    fn reduce(&self) -> Expression {
        match self {
            Expression::AddExpr(x) => Add::reduce(x),
            Expression::MultiplyExpr(x) => Multiply::reduce(x),
            Expression::NumberExpr(x) => Expression::NumberExpr(x.clone()),
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

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::AddExpr(x) => write!(f, "{}", x),
            Expression::MultiplyExpr(x) => write!(f, "{}", x),
            Expression::NumberExpr(x) => write!(f, "{}", x),
        }
    }
}

fn main() {
    let e = Expression::AddExpr(Add {
        left: Box::new(Expression::NumberExpr(Number(5))),
        right: Box::new(Expression::MultiplyExpr(Multiply {
            left: Box::new(Expression::NumberExpr(Number(4))),
            right: Box::new(Expression::NumberExpr(Number(4))),
        })),
    });

    let a = Expression::AddExpr(Add {
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

    let mut machine = Machine { expression: e };
    machine.run();
}
