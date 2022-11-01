use crate::calculation::operator::Operator;
use crate::calculation::Calculation;

pub fn combine_operands_and_operators(
    mut operands: Vec<Calculation>,
    operators: Vec<Operator>,
) -> Calculation {
    match operands.len() {
        0 => panic!("no operands"), // the parsing code above should have handled this
        1 => return operands.pop().expect("checked"),
        _ => {} // let's handle the non-triavial cases below
    }

    let mut operands_iter = operands.drain(..);
    let mut lookahead_iter = operators.iter().skip(1);

    let mut result = operands_iter.next().expect("checked");
    let mut tmp_operator: Option<Operator> = None;
    let mut tmp_result: Option<Calculation> = None;

    for (operand, operator) in operands_iter.zip(operators.iter()) {
        // check if we can append a delayed operation
        if let Some(delayed_operator) = tmp_operator {
            if delayed_operator.rank() == operator.rank() {
                // then insert the delayed op into the tree
                result = Calculation::Unevaluated {
                    lhs: Box::new(tmp_result.take().expect("checked")),
                    operator: tmp_operator.take().expect("checked"),
                    rhs: Box::new(result),
                };
            }
        }

        if let Some(lookahead) = lookahead_iter.next() {
            if lookahead.rank() > operator.rank() {
                // then delay the expansion of the current tree and store it away
                tmp_result = Some(result);
                tmp_operator = Some(*operator);
                result = operand;
                continue;
            }
        }
        result = Calculation::Unevaluated {
            lhs: Box::new(result),
            operator: *operator,
            rhs: Box::new(operand),
        }
    }

    if let Some(operator) = tmp_operator {
        result = Calculation::Unevaluated {
            lhs: Box::new(tmp_result.take().expect("checked")),
            operator,
            rhs: Box::new(result),
        }
    }
    result
}
