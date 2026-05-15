use std::iter::Peekable;
use std::vec::IntoIter;

// program entry
fn main()
{
    let input = "(-6+2)*sin(1)%2";
    let input1 = "pi";
    let tokens = tokenize_expression(input1);

    let mut it = tokens.into_iter().peekable();
    let root = parse_expression(&mut it);
    print_tree(&root, 0);
    let result = eval_tree(&root);
    println!("result: {}", result);
}

// enum that represents a token used to classify parts of the expression
#[derive(Debug, PartialEq)]
enum Token
{
    Number(f64),
    Paren(char),        // '(' or ')'
    Operator(char),     // '+', '-', '*', '/', '^', '%', '!', ','
    Identifier(String), // function names, variables and constants like "sin", "x", "pi"
}

// enum that represents a node in the abstract syntax tree (AST)
// Number and Variable are leaf nodes, all others are inner nodes with children
enum Node
{
    Number(f64),
    BinaryExpr
    {
        left: Box<Node>,  // left recursive child node
        op: char,         // binary operator
        right: Box<Node>, // right recursive child node
    },
    UnaryPostfix
    {
        op: char, // postfix operator, e.g. '!'
        child: Box<Node>,
    },
    UnaryPrefix
    {
        op: char, // prefix operator, e.g. unary '-'
        child: Box<Node>,
    },
    FunctionCall
    {
        name: String,    // function name, e.g. "sin"
        args: Vec<Node>, // evaluated argument list
    },
    Variable(String), // named constant or variable, e.g. "pi", "e"
}

// tokenizes a mathematical expression string into a vector of Tokens
// param: input string containing the expression
// return: vector of Tokens in order of appearance
fn tokenize_expression(input: &str) -> Vec<Token>
{
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek()
    {
        match c
        {
            // greedily consume digits and '.' into a single number token
            '0'..='9' | '.' =>
            {
                let mut num_string = String::new();
                while let Some(&c) = chars.peek()
                {
                    if c.is_ascii_digit() || c == '.'
                    {
                        num_string.push(chars.next().unwrap());
                    }
                    else
                    {
                        break;
                    }
                }
                tokens.push(Token::Number(num_string.parse().unwrap_or(0.0)));
            }
            // greedily consume alphanumeric chars into an identifier token
            'a'..='z' | 'A'..='Z' =>
            {
                let mut id_string = String::new();
                while let Some(&c) = chars.peek()
                {
                    if c.is_ascii_alphanumeric()
                    {
                        id_string.push(chars.next().unwrap())
                    }
                    else
                    {
                        break;
                    }
                }
                tokens.push(Token::Identifier(id_string));
            }
            // single-char operators, ',' is included here for use in argument lists
            '+' | '-' | '*' | '/' | '^' | '%' | '!' | ',' =>
            {
                tokens.push(Token::Operator(chars.next().unwrap()));
            }
            '(' | ')' =>
            {
                tokens.push(Token::Paren(chars.next().unwrap()));
            }
            _ if c.is_whitespace() =>
            {
                chars.next();
            }
            _ =>
            {
                println!("unknown char: {}", c);
                chars.next();
            }
        }
    }
    tokens
}

// parses an atomic expression: a number, a parenthesized expression,
// a function call or a variable
// lowest level of the recursive descent — no operators handled here
// param: reference to a peekable token iterator
// return: leaf Node or subtree for parenthesized/function expressions
fn parse_atom(it: &mut Peekable<IntoIter<Token>>) -> Node
{
    match it.next()
    {
        Some(Token::Number(n)) => Node::Number(n),

        // parenthesized sub-expression: recursively parse the inner expression
        Some(Token::Paren('(')) =>
        {
            let result = parse_expression(it);
            // expect closing parenthesis
            match it.next()
            {
                Some(Token::Paren(')')) => result,
                _ => panic!("Expected closing parenthesis ')'"),
            }
        }

        // BUG: it.next() here consumes the next token even if it isn't '('
        // use it.peek() + conditional it.next() instead
        Some(Token::Identifier(name)) => match it.next()
        {
            // function call: parse comma-separated argument list
            Some(Token::Paren('(')) =>
            {
                let mut args = Vec::new();

                if let Some(Token::Paren(')')) = it.peek()
                // empty argument list
                {
                    it.next();
                    Node::FunctionCall { name, args }
                }
                else
                {
                    args.push(parse_expression(it)); // first argument

                    // each ',' introduces another argument
                    while let Some(Token::Operator(',')) = it.peek()
                    {
                        it.next();
                        args.push(parse_expression(it));
                    }

                    if let Some(Token::Paren(')')) = it.next()
                    {
                        Node::FunctionCall { name, args }
                    }
                    else
                    {
                        panic!("expected ')' after args of '{}'", name);
                    }
                }
            }
            // anything other than '(' means the identifier is a variable/constant
            _ => Node::Variable(name),
        },
        Some(t) => panic!("Unexpected token in atom: {:?}", t),
        None => panic!("Unexpected end of input"),
    }
}

// parses a unary prefix operator (currently only unary minus)
// right-associative: ---x is parsed as -(-(-(x)))
// param: reference to a peekable token iterator
// return: UnaryPrefix Node wrapping the operand, or delegates to parse_unary_postfix
fn parse_unary_prefix(it: &mut Peekable<IntoIter<Token>>) -> Node
{
    if let Some(Token::Operator(op @ '-')) = it.peek()
    {
        let op = *op;
        it.next();
        return Node::UnaryPrefix {
            op,
            child: Box::new(parse_unary_prefix(it)), // recursive for chained prefix ops
        };
    }
    else
    {
        parse_unary_postfix(it)
    }
}

// parses unary postfix operators (currently only '!' for factorial)
// left-associative: 5!! wraps as UnaryPostfix(UnaryPostfix(5))
// param: reference to a peekable token iterator
// return: UnaryPostfix Node, or the atom unchanged if no postfix op follows
fn parse_unary_postfix(it: &mut Peekable<IntoIter<Token>>) -> Node
{
    let mut node = parse_atom(it);
    while let Some(Token::Operator(op @ '!')) = it.peek()
    {
        let op = *op;
        it.next();
        node = Node::UnaryPostfix {
            op,
            child: Box::new(node),
        };
    }
    node
}

// parses power expressions (right-associative: 2^3^4 = 2^(3^4))
// param: reference to a peekable token iterator
// return: root of the power expression subtree
fn parse_power(it: &mut Peekable<IntoIter<Token>>) -> Node
{
    let mut left = parse_unary_prefix(it);
    if let Some(Token::Operator('^')) = it.peek()
    {
        it.next();
        // right-recursive call gives right-associativity
        let right = parse_power(it);
        left = Node::BinaryExpr {
            left: Box::new(left),
            op: '^',
            right: Box::new(right),
        };
    }
    left
}

// parses multiplication, division and modulo (left-associative, equal precedence)
// param: reference to a peekable token iterator
// return: root of the term subtree
fn parse_term(it: &mut Peekable<IntoIter<Token>>) -> Node
{
    let mut left = parse_power(it);
    while let Some(token) = it.peek()
    {
        match token
        {
            Token::Operator('*') | Token::Operator('/') | Token::Operator('%') =>
            {
                let op_token = it.next().unwrap();
                let op = if let Token::Operator(c) = op_token
                {
                    c
                }
                else
                {
                    unreachable!("");
                };
                let right = parse_power(it);
                left = Node::BinaryExpr {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    left
}

// parses addition and subtraction (left-associative, lowest precedence)
// entry point of the recursive descent parser
// param: reference to a peekable token iterator
// return: root of the full expression tree
fn parse_expression(it: &mut Peekable<IntoIter<Token>>) -> Node
{
    let mut left = parse_term(it);
    while let Some(token) = it.peek()
    {
        match token
        {
            Token::Operator('+') | Token::Operator('-') =>
            {
                let op_token = it.next().unwrap();
                let op = if let Token::Operator(c) = op_token
                {
                    c
                }
                else
                {
                    unreachable!("");
                };
                let right = parse_term(it);

                left = Node::BinaryExpr {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    left
}

// evaluates an expression tree and returns its numerical result
// param: root Node of the expression tree
// return: calculated f64 result
fn eval_tree(node: &Node) -> f64
{
    match node
    {
        Node::Number(n) =>
        {
            return *n;
        }
        Node::Variable(s) => match s.to_lowercase().as_str()
        {
            // built-in named constants
            "pi" => std::f64::consts::PI,
            "e" => std::f64::consts::E,
            _ => panic!("unknown variable or constant {}", s),
        },

        Node::BinaryExpr { left, op, right } =>
        {
            let left_result = eval_tree(left);
            let right_result = eval_tree(right);
            match op
            {
                '+' => left_result + right_result,
                '-' => left_result - right_result,
                '*' => left_result * right_result,
                // division by zero yields f64::INFINITY, consistent with IEEE 754
                '/' => left_result / right_result,
                '%' => left_result % right_result,
                '^' => left_result.powf(right_result),
                _ => f64::NAN,
            }
        }
        Node::UnaryPrefix { op, child } =>
        {
            let child_result = eval_tree(child);
            match op
            {
                '-' => -child_result,
                _ => f64::NAN,
            }
        }
        Node::UnaryPostfix { op, child } =>
        {
            let child_result = eval_tree(child);
            match op
            {
                // factorial: rounds to nearest integer before computing
                // panics on overflow for large values (u64 limit: 20!)
                '!' => (1..=(child_result.round() as u64)).product::<u64>() as f64,
                '%' => child_result / 100.0,
                _ => f64::NAN,
            }
        }
        Node::FunctionCall { name, args } => match name.as_str()
        {
            // trig functions expect radians
            "sin" =>
            {
                validate_args(name, args, 1);
                eval_tree(&args[0]).sin()
            }
            "cos" =>
            {
                validate_args(name, args, 1);
                eval_tree(&args[0]).cos()
            }
            "tan" =>
            {
                validate_args(name, args, 1);
                eval_tree(&args[0]).tan()
            }
            "sinh" =>
            {
                validate_args(name, args, 1);
                eval_tree(&args[0]).sinh()
            }
            "cosh" =>
            {
                validate_args(name, args, 1);
                eval_tree(&args[0]).cosh()
            }
            "tanh" =>
            {
                validate_args(name, args, 1);
                eval_tree(&args[0]).tanh()
            }
            "sqrt" =>
            {
                validate_args(name, args, 1);
                eval_tree(&args[0]).sqrt()
            }
            "max" =>
            {
                validate_args(name, args, 2);
                let a = eval_tree(&args[0]);
                let b = eval_tree(&args[1]);
                a.max(b)
            }
            "min" =>
            {
                validate_args(name, args, 2);
                let a = eval_tree(&args[0]);
                let b = eval_tree(&args[1]);
                a.min(b)
            }
            // root(x, n) computes the n-th root of x as x^(1/n)
            "root" =>
            {
                validate_args(name, args, 2);
                let a = eval_tree(&args[0]);
                let b = eval_tree(&args[1]);
                a.powf(1.0 / b)
            }
            // log(x, base) — argument order: value first, base second
            "log" =>
            {
                validate_args(name, args, 2);
                let a = eval_tree(&args[0]);
                let b = eval_tree(&args[1]);
                a.log(b)
            }
            "log10" =>
            {
                validate_args(name, args, 1);
                eval_tree(&args[0]).log10()
            }
            "ln" =>
            {
                validate_args(name, args, 1);
                eval_tree(&args[0]).ln()
            }
            "log2" =>
            {
                validate_args(name, args, 1);
                eval_tree(&args[0]).log2()
            }
            _ =>
            {
                panic!("unknown function {}", name);
            }
        },
    }
}

// panics if the number of provided arguments does not match the expected count
// param: function name (for error message), arg slice, expected count
fn validate_args(name: &str, args: &Vec<Node>, num_args: usize)
{
    if args.len() != num_args
    {
        panic!("{} expects exactly {} argument(s)", name, num_args);
    }
}

// prints the expression tree to stdout with indentation showing depth
// param: root Node, current indent level (call with 0 at the root)
fn print_tree(node: &Node, indent: usize)
{
    let spacing = "   ".repeat(indent);
    match node
    {
        Node::Number(n) => println!("{}number: {}", spacing, n),

        Node::BinaryExpr { left, op, right } =>
        {
            println!("{}operator: {}", spacing, op);
            print_tree(left, indent + 1);
            print_tree(right, indent + 1);
        }
        Node::UnaryPrefix { op, child } =>
        {
            println!("{}operator: {}", spacing, op);
            print_tree(child, indent + 1);
        }
        Node::UnaryPostfix { op, child } =>
        {
            println!("{}operator: {}", spacing, op);
            print_tree(child, indent + 1);
        }
        Node::FunctionCall { name, args } =>
        {
            println!("{}function: {}", spacing, name);
            for (i, arg) in args.iter().enumerate()
            {
                println!("{}  arg {}:", spacing, i + 1);
                print_tree(arg, indent + 2);
            }
        }
        Node::Variable(s) => println!("{}variable: {}", spacing, s),
    }
}
