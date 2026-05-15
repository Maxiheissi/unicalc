use std::iter::Peekable;
use std::vec::IntoIter;

//module that handles tokenizing input, creatig a abstract syntax tree and calculating the result

// all errors that can occur during parsing or evaluation
#[derive(Debug)]
pub enum CalcError
{
    UnexpectedToken(String),
    UnexpectedEnd,
    UnknownFunction(String),
    UnknownVariable(String),
    WrongArgCount
    {
        name: String,
        expected: usize,
        got: usize,
    },
    UnclosedParen,
}

// enum that represents a token used to classify parts of the expression
#[derive(Debug, PartialEq)]
pub enum Token
{
    Number(f64),
    Paren(char),        // '(' or ')'
    Operator(char),     // '+', '-', '*', '/', '^', '%', '!', ','
    Identifier(String), // function names, variables and constants like "sin", "x", "pi"
}

// enum that represents a node in the abstract syntax tree (AST)
// Number and Variable are leaf nodes, all others are inner nodes with children
pub enum Node
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
pub fn tokenize_expression(input: &str) -> Vec<Token>
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
pub fn parse_atom(it: &mut Peekable<IntoIter<Token>>) -> Result<Node, CalcError>
{
    match it.next()
    {
        Some(Token::Number(n)) => Ok(Node::Number(n)),

        // parenthesized sub-expression: recursively parse the inner expression
        Some(Token::Paren('(')) =>
        {
            let result = parse_expression(it)?;
            match it.next()
            {
                Some(Token::Paren(')')) => Ok(result),
                _ => Err(CalcError::UnclosedParen),
            }
        }

        // peek at the next token to decide between function call and variable
        // without consuming a token that belongs to the surrounding expression
        Some(Token::Identifier(name)) =>
        {
            if let Some(Token::Paren('(')) = it.peek()
            {
                it.next(); // consume '('
                let mut args = Vec::new();

                if let Some(Token::Paren(')')) = it.peek()
                // empty argument list
                {
                    it.next();
                    Ok(Node::FunctionCall { name, args })
                }
                else
                {
                    args.push(parse_expression(it)?); // first argument

                    // each ',' introduces another argument
                    while let Some(Token::Operator(',')) = it.peek()
                    {
                        it.next();
                        args.push(parse_expression(it)?);
                    }

                    match it.next()
                    {
                        Some(Token::Paren(')')) => Ok(Node::FunctionCall { name, args }),
                        _ => Err(CalcError::UnclosedParen),
                    }
                }
            }
            else
            {
                // anything other than '(' means the identifier is a variable/constant
                Ok(Node::Variable(name))
            }
        }
        Some(t) => Err(CalcError::UnexpectedToken(format!("{:?}", t))),
        None => Err(CalcError::UnexpectedEnd),
    }
}

// parses a unary prefix operator (currently only unary minus)
// right-associative: ---x is parsed as -(-(-(x)))
// param: reference to a peekable token iterator
// return: UnaryPrefix Node wrapping the operand, or delegates to parse_unary_postfix
pub fn parse_unary_prefix(it: &mut Peekable<IntoIter<Token>>) -> Result<Node, CalcError>
{
    if let Some(Token::Operator(op @ '-')) = it.peek()
    {
        let op = *op;
        it.next();
        // recursive for chained prefix ops
        return Ok(Node::UnaryPrefix {
            op,
            child: Box::new(parse_unary_prefix(it)?),
        });
    }
    parse_unary_postfix(it)
}

// parses unary postfix operators (currently only '!' for factorial)
// left-associative: 5!! wraps as UnaryPostfix(UnaryPostfix(5))
// param: reference to a peekable token iterator
// return: UnaryPostfix Node, or the atom unchanged if no postfix op follows
pub fn parse_unary_postfix(it: &mut Peekable<IntoIter<Token>>) -> Result<Node, CalcError>
{
    let mut node = parse_atom(it)?;
    while let Some(Token::Operator(op @ '!')) = it.peek()
    {
        let op = *op;
        it.next();
        node = Node::UnaryPostfix {
            op,
            child: Box::new(node),
        };
    }
    Ok(node)
}

// parses power expressions (right-associative: 2^3^4 = 2^(3^4))
// param: reference to a peekable token iterator
// return: root of the power expression subtree
pub fn parse_power(it: &mut Peekable<IntoIter<Token>>) -> Result<Node, CalcError>
{
    let mut left = parse_unary_prefix(it)?;
    if let Some(Token::Operator('^')) = it.peek()
    {
        it.next();
        // right-recursive call gives right-associativity
        let right = parse_power(it)?;
        left = Node::BinaryExpr {
            left: Box::new(left),
            op: '^',
            right: Box::new(right),
        };
    }
    Ok(left)
}

// parses multiplication, division and modulo (left-associative, equal precedence)
// param: reference to a peekable token iterator
// return: root of the term subtree
pub fn parse_term(it: &mut Peekable<IntoIter<Token>>) -> Result<Node, CalcError>
{
    let mut left = parse_power(it)?;
    while let Some(token) = it.peek()
    {
        match token
        {
            Token::Operator('*') | Token::Operator('/') | Token::Operator('%') =>
            {
                let op = if let Token::Operator(c) = it.next().unwrap()
                {
                    c
                }
                else
                {
                    unreachable!("");
                };
                let right = parse_power(it)?;
                left = Node::BinaryExpr {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    Ok(left)
}

// parses addition and subtraction (left-associative, lowest precedence)
// entry point of the recursive descent parser
// param: reference to a peekable token iterator
// return: root of the full expression tree
pub fn parse_expression(it: &mut Peekable<IntoIter<Token>>) -> Result<Node, CalcError>
{
    let mut left = parse_term(it)?;
    while let Some(token) = it.peek()
    {
        match token
        {
            Token::Operator('+') | Token::Operator('-') =>
            {
                let op = if let Token::Operator(c) = it.next().unwrap()
                {
                    c
                }
                else
                {
                    unreachable!("");
                };
                let right = parse_term(it)?;
                left = Node::BinaryExpr {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    Ok(left)
}

// evaluates an expression tree and returns its numerical result
// param: root Node of the expression tree
// return: calculated f64 result
pub fn eval_tree(node: &Node) -> Result<f64, CalcError>
{
    match node
    {
        Node::Number(n) => Ok(*n),

        // built-in named constants
        Node::Variable(s) => match s.to_lowercase().as_str()
        {
            "pi" => Ok(std::f64::consts::PI),
            "e" => Ok(std::f64::consts::E),
            _ => Err(CalcError::UnknownVariable(s.clone())),
        },

        Node::BinaryExpr { left, op, right } =>
        {
            let left_result = eval_tree(left)?;
            let right_result = eval_tree(right)?;
            match op
            {
                '+' => Ok(left_result + right_result),
                '-' => Ok(left_result - right_result),
                '*' => Ok(left_result * right_result),
                // divisiMöchtest du direkt mit der einfachen REPL-Schleife (Schritt 1) starten und vielleicht das oben besprochene Feature einbauen, dass man Variablen wie x = 5 speichern kann, oder willst du dich direkt in das Abenteuer ratatui-Vollbild-Interface stürzen?on by zero yields f64::INFINITY, consistent with IEEE 754
                '/' => Ok(left_result / right_result),
                '%' => Ok(left_result % right_result),
                '^' => Ok(left_result.powf(right_result)),
                _ => Ok(f64::NAN),
            }
        }
        Node::UnaryPrefix { op, child } =>
        {
            let child_result = eval_tree(child)?;
            match op
            {
                '-' => Ok(-child_result),
                _ => Ok(f64::NAN),
            }
        }
        Node::UnaryPostfix { op, child } =>
        {
            let child_result = eval_tree(child)?;
            match op
            {
                // factorial: rounds to nearest integer before computing
                // overflows silently for n > 20 (u64 limit)
                '!' => Ok((1..=(child_result.round() as u64)).product::<u64>() as f64),
                '%' => Ok(child_result / 100.0),
                _ => Ok(f64::NAN),
            }
        }
        Node::FunctionCall { name, args } =>
        {
            match name.as_str()
            {
                // trig functions expect radians
                "sin" =>
                {
                    validate_args(name, args, 1)?;
                    Ok(eval_tree(&args[0])?.sin())
                }
                "cos" =>
                {
                    validate_args(name, args, 1)?;
                    Ok(eval_tree(&args[0])?.cos())
                }
                "tan" =>
                {
                    validate_args(name, args, 1)?;
                    Ok(eval_tree(&args[0])?.tan())
                }
                "sinh" =>
                {
                    validate_args(name, args, 1)?;
                    Ok(eval_tree(&args[0])?.sinh())
                }
                "cosh" =>
                {
                    validate_args(name, args, 1)?;
                    Ok(eval_tree(&args[0])?.cosh())
                }
                "tanh" =>
                {
                    validate_args(name, args, 1)?;
                    Ok(eval_tree(&args[0])?.tanh())
                }
                "sqrt" =>
                {
                    validate_args(name, args, 1)?;
                    Ok(eval_tree(&args[0])?.sqrt())
                }
                "max" =>
                {
                    validate_args(name, args, 2)?;
                    let a = eval_tree(&args[0])?;
                    let b = eval_tree(&args[1])?;
                    Ok(a.max(b))
                }
                "min" =>
                {
                    validate_args(name, args, 2)?;
                    let a = eval_tree(&args[0])?;
                    let b = eval_tree(&args[1])?;
                    Ok(a.min(b))
                }
                // root(x, n) computes the n-th root of x as x^(1/n)
                "root" =>
                {
                    validate_args(name, args, 2)?;
                    let a = eval_tree(&args[0])?;
                    let b = eval_tree(&args[1])?;
                    Ok(a.powf(1.0 / b))
                }
                // log(x, base) — argument order: value first, base second
                "log" =>
                {
                    validate_args(name, args, 2)?;
                    let a = eval_tree(&args[0])?;
                    let b = eval_tree(&args[1])?;
                    Ok(a.log(b))
                }
                "log10" =>
                {
                    validate_args(name, args, 1)?;
                    Ok(eval_tree(&args[0])?.log10())
                }
                "ln" =>
                {
                    validate_args(name, args, 1)?;
                    Ok(eval_tree(&args[0])?.ln())
                }
                "log2" =>
                {
                    validate_args(name, args, 1)?;
                    Ok(eval_tree(&args[0])?.log2())
                }
                _ => Err(CalcError::UnknownFunction(name.clone())),
            }
        }
    }
}

// returns Err if the number of provided arguments does not match the expected count
// param: function name (for error message), arg slice, expected count
pub fn validate_args(name: &str, args: &[Node], num_args: usize) -> Result<(), CalcError>
{
    if args.len() != num_args
    {
        return Err(CalcError::WrongArgCount {
            name: name.to_string(),
            expected: num_args,
            got: args.len(),
        });
    }
    Ok(())
}

// prints the expression tree to stdout with indentation showing depth
// param: root Node, current indent level (call with 0 at the root)
pub fn print_tree(node: &Node, indent: usize)
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
