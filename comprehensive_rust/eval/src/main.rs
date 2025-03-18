/// 두 개의 하위 표현식에서 실행할 연산입니다.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// 트리 형식의 표현식입니다.
#[derive(Debug)]
enum Expression {
    /// 두 개의 하위 표현식에 관한 연산입니다.
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// 리터럴 값
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Value(v) => Ok(v),
        Expression::Op { op, left, right } => match op {
            Operation::Add => Ok(eval(*left)? + eval(*right)?),
            Operation::Sub => Ok(eval(*left)? - eval(*right)?),
            Operation::Mul => Ok(eval(*left)? * eval(*right)?),
            Operation::Div => {
                let devisor = eval(*right)?;
                if devisor == 0 {
                    return Err(String::from("0으로 나누기"));
                }
                Ok(eval(*left)? / devisor)
            }
        },
    }
}

fn eval_answer(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Op { op, left, right } => {
            let left = match eval(*left) {
                Ok(v) => v,
                e @ Err(_) => return e,
            };
            let right = match eval(*right) {
                Ok(v) => v,
                e @ Err(_) => return e,
            };
            Ok(match op {
                Operation::Add => left + right,
                Operation::Sub => left - right,
                Operation::Mul => left * right,
                Operation::Div => {
                    if right == 0 {
                        return Err(String::from("0으로 나누기"));
                    } else {
                        left / right
                    }
                }
            })
        }
        Expression::Value(v) => Ok(v),
    }
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(85)
    );
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("0으로 나누기"))
    );
}

fn main() {
    println!("Hello, eval! {:?}", eval(Expression::Value(19)));
    println!(
        "Hello, eval_answer! {:?}",
        eval_answer(Expression::Value(19))
    );
}
