use crate::HomonymParser::Expression;
//TODO: Don't change the parser, but add type fields to Expression::Operator to allow for types to be saved. Then, this function will recursively step through the parsed expression tree to determine if the types are correct
pub fn check_expression_types(expression: Expression) -> bool {
    true
}
