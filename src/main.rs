fn main() {
    for line in std::io::stdin().lines() {
        let mut stack = vec![];

        if let Ok(line) = line {
            let words: Vec<_> = line.split(" ").collect();

            for word in words {
                if let Ok(parsed) = word.parse::<i32>() {
                    stack.push(parsed);
                } else {
                    match word {
                        "+" => add(&mut stack),
                        "-" => sub(&mut stack),
                        "*" => mul(&mut stack),
                        "/" => div(&mut stack),
                        "%" => r#mod(&mut stack),
                        _ => panic!("{:?} could not be parsed", word),
                    }
                }
            }

            println!("stack: {:?}", stack);
        }
    }
}

fn add(stack: &mut Vec<i32>) {
    let rhs = stack.pop().unwrap();
    let lhs = stack.pop().unwrap();
    stack.push(lhs + rhs);
}

fn sub(stack: &mut Vec<i32>) {
    let rhs = stack.pop().unwrap();
    let lhs = stack.pop().unwrap();
    stack.push(lhs - rhs);
}

fn mul(stack: &mut Vec<i32>) {
    let rhs = stack.pop().unwrap();
    let lhs = stack.pop().unwrap();
    stack.push(lhs * rhs);
}

fn div(stack: &mut Vec<i32>) {
    let rhs = stack.pop().unwrap();
    let lhs = stack.pop().unwrap();
    stack.push(lhs / rhs);
}

fn r#mod(stack: &mut Vec<i32>) {
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();
    stack.push(lhs % rhs);
}
