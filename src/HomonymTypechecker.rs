use crate::HomonymParser::Expression;
use std::collections::HashMap;
//TODO: Don't change the parser, but add type fields to Expression::Operator to allow for types to be saved. Then, this function will recursively step through the parsed expression tree to determine if the types are correct
// The Recursive calls should pass in a set of defined types, as well as a context including the defined variables of each type
// For programs with multiple expressions, make a typechecking call on each expression in sequence, building up the context/types.
// Given a successful typechecking of all expressions, do the actually evaluation. This will build up a similar context, but instead of
// simply tracking what variables exist of each type, it will provide evaluations
// Treat function calls as their own little programs, starting them with their own context including passed in arguments.
// The typechecker should make sure that values returned by functions are the correct type
pub fn check_expression_types(
    expression: Expression,
    typename: &str,
    varmap: &mut HashMap<String, String>,
) -> bool {
    match expression {
        Expression::BOOLEAN(_) => typename == "bool",
        Expression::FLTVAL(_) => typename == "float",
        Expression::INTVAL(_) => typename == "int",
        Expression::STRINGVAL(_) => typename == "string",
        Expression::TEXT(varname) => match varmap.get(&varname) {
            Some(vartype) => vartype == typename,
            None => panic!("{} is not a defined variable!", varname),
        },
        Expression::PLUS(t1, t2, e1, e2)
        | Expression::MINUS(t1, t2, e1, e2)
        | Expression::TIMES(t1, t2, e1, e2)
        | Expression::DIVIDEDBY(t1, t2, e1, e2) => {
            check_expression_types(*e1, &t1, varmap) && check_expression_types(*e2, &t2, varmap)
        }
        // THESE DON'T ACTUALLY DO THE SAME THING, it's not checking if the let evaluates to typename
        // SOLUTION: CHECK IN PARSER THAT LET EXPRESSION IS ALWAYS ROOT OF TREE, THEN ABORT THERE IF NOT
        // Eventually: for expr in program, if expr is a let, only typecheck the RHS expr, then add variable to type dict.
        Expression::RETURN(ret_type, expr) => check_expression_types(*expr, &ret_type, varmap),
        Expression::LET(varname, vartype, expr) => {
            varmap.insert(varname, vartype.clone());
            return check_expression_types(*expr, &vartype, varmap);
        }
        _ => panic!("I don't know how to typecheck that {:?}!", expression),
    }
}
