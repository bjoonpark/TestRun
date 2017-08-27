
pub enum Operator {
    // `+`
    Add, 
    // `-`
    Sub,
    // `*`
    Mul,
}

pub enum Token {
    Operator(Operator),
    Operand(isize),
}

/// Evaluates the postix expression.
///
/// Input: a postfix expression, where each element contains an operator or operand.
/// Returns: if the postfix expression is valid, returns `Some(value)`;
///     otherwise, returns `None`.


pub fn inspect(t: &Token) {
   match t {
        &Token::Operator(Operator::Add)  => print!("+ "),
        &Token::Operator(Operator::Sub) => print!("- "),
        &Token::Operator(Operator::Mul) => print!("* "),
        &Token::Operand(i) => print!("{} ", i),   
    }
}

/*
pub fn is_number(t: &Token) {
   match t {
        &Token::Operand(i) => return i,   
    }
}

	  if let x = is_number(token){
	    println!("Number {:?}", x);
	  }
*/

pub fn eval(tokens: &[Token]) -> Option<isize> {
    // TODO
    //unimplemented!();
	
	let mut stack_calc: Vec<isize> = vec![];
		
	
	for token in tokens{
		match token{
			&Token::Operator(Operator::Add) => {
				if stack_calc.len() < 2{
					return None;
				}
				else if stack_calc.len() >= 2{
					let a = stack_calc.pop().unwrap();
					let b = stack_calc.pop().unwrap();
					stack_calc.push(a+b);
				}
				
			}
			&Token::Operator(Operator::Sub) => {
				if stack_calc.len() < 2{
					return None;
				}
				else if stack_calc.len() >= 2{
					let a = stack_calc.pop().unwrap();
					let b = stack_calc.pop().unwrap();
					stack_calc.push(b-a);
				}
			}
			&Token::Operator(Operator::Mul) => {
				if stack_calc.len() < 2{
					return None;
				}
				else if stack_calc.len() >= 2{
					let a = stack_calc.pop().unwrap();
					let b = stack_calc.pop().unwrap();
					stack_calc.push(a*b);
				}
			}
			&Token::Operand(x) => {
				//println!("value = {}", x);
				stack_calc.push(x);                
			}	
		}	
	}
	//If result exists in stack, return result;
	if stack_calc.len() > 0{
		return Some(stack_calc[0]);
	}
	
	////Possible Returns
	//Some(1)
	//return None;
	
	None
}


fn main(){
   //2 4 +
   //let x = eval(&[Token::Operand(2), Token::Operand(4), Token::Operator(Operator::Add)]);
   
   //1 2 3 + *
   //let x = eval(&[Token::Operand(1), Token::Operand(2), Token::Operand(3), Token::Operator(Operator::Add), Token::Operator(Operator::Mul)]);
   
   //1 2 3 - *
   //let x = eval(&[Token::Operand(1), Token::Operand(2), Token::Operand(3), Token::Operator(Operator::Sub), Token::Operator(Operator::Mul)]);
   
   //-999
   //let x = eval(&[Token::Operand(-999)]);
   
   
   
   
   //1 2 + *
   //let x = eval(&[Token::Operand(1), Token::Operand(2), Token::Operator(Operator::Add), Token::Operator(Operator::Mul)]);
   
   //+
   //let x = eval(&[Token::Operator(Operator::Add)]);
   
   
   
   
   //println!("{:?}",x);
}