//--------------------faculty doesnt work rn------------------------

use std::iter::Peekable;
use std::vec::IntoIter;

//programm entry
fn main()
{
    let input = "(-6+2)*sin(1)%2";
    let tokens = tokenize_expression(input);
    println!("Tokens: {:?}", tokens);

    let mut it = tokens.into_iter().peekable();
    let root = parse_expression(&mut it);
    print_tree(&root, 0);
    let result = eval_tree(&root);
    println!("result: {}", result);
}
//A enum that represents a token used to classify the expression
#[derive(Debug, PartialEq)]
enum Token
{
    Number(f64),
    Paren(char),        // '(' oder ')'
    Operator(char),     // '+', '-', '*', '/'
    Identifier(String), // "sin", "x", "pi"
}

// A enum that represent a node of a abstact syntax tree.
// can be either a atomic number (leaf node)
// or a binary expression that consists of a left node, right node and a opertion
// a unary expression containing a operation and a child Node
// a function call containing a function name and arg Node
enum Node
{
    Number(f64),
    BinaryExpr
    {
        left: Box<Node>,  //left rekursive childnode
        op: char,         //binary operation
        right: Box<Node>, //right rekursice childnode
    },
    UnaryExpr
    {
        op: char,
        child: Box<Node>,
    },
    FunctionCall
    {
        name: String,
        arg: Box<Node>,
    },
}

//function to tokenize a mathematial expression string into a vector of Tokens
// param: input string containing expression
// return: vector of Tokens
fn tokenize_expression(input: &str) -> Vec<Token>
{
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek()
    {
        match c
        {
            '0'..='9' | '.' =>
            //numbers
            {
                let mut num_string = String::new();
                while let Some(&c) = chars.peek()
                {
                    if c.is_ascii_digit()
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
            'a'..='z' | 'A'..='Z' =>
            //indentifiers
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

            '+' | '-' | '*' | '/' | '^' | '%' | '!' =>
            //operations
            {
                tokens.push(Token::Operator(chars.next().unwrap()));
            }
            '(' | ')' =>
            {
                tokens.push(Token::Paren(chars.next().unwrap()));
            }
            _ if c.is_whitespace() =>
            //skip whitspaces
            {
                chars.next();
            }
            _ =>
            //error for other chars
            {
                println!("unknown char: {}", c);
                chars.next();
            }
        }
    }
    tokens
}

//parser that puts atomic expressions like numbers and functions into nodes
//if a open parentheses is encountered rekursive search for a new epression is started
//param: reference to a Token iterator
//return: a atomic Node or tree if parenthises are found
fn parse_atom(it: &mut Peekable<IntoIter<Token>>) -> Node
{
    match it.next()
    {
        Some(Token::Number(n)) => Node::Number(n),
        Some(Token::Paren('(')) =>
        {
            let result = parse_expression(it);
            // Expect closing parenthesis
            match it.next()
            {
                Some(Token::Paren(')')) => result,
                _ => panic!("Expected closing parenthesis ')'"),
            }
        }
        Some(Token::Identifier(name)) => match it.next()
        {
            Some(Token::Paren('(')) =>
            {
                let arg = parse_expression(it);
                if let Some(Token::Paren(')')) = it.next()
                {
                    Node::FunctionCall {
                        name,
                        arg: Box::new(arg),
                    }
                }
                else
                {
                    panic!("expected ')' ending argument");
                }
            }
            _ => panic!("expected '(' after funcion name"),
        },
        Some(t) => panic!("Unexpected token in atom: {:?}", t),
        None => panic!("Unexpected end of input"),
    }
}

fn parse_unary(it: &mut Peekable<IntoIter<Token>>) -> Node
{
    if let Some(Token::Operator(op @ '-')) = it.peek()
    {
        let op = *op;
        it.next();
        return Node::UnaryExpr {
            op,
            child: Box::new(parse_unary(it)),
        };
    }
    parse_atom(it)
}

//parser that creates a tree for power operations (2²)
// param: reference to a Token iterator
// return: root to the power expression tree
fn parse_power(it: &mut Peekable<IntoIter<Token>>) -> Node
{
    let mut left = parse_unary(it);

    if let Some(Token::Operator('^')) = it.peek()
    {
        it.next();
        let right = parse_power(it);
        left = Node::BinaryExpr {
            left: Box::new(left),
            op: '^',
            right: Box::new(right),
        };
    }
    left
}

//parser that creates a tree for a term
// param: reference to a Token iterator
// return: root to term tree
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

//parser that returns a tree for a mathematical expression
// param: reference to a Token iterator
// return: root to expression tree
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

//function that calculates the result of a given expression tree
//param: root Node to expression tree
//return: calculated f64 value of given tree
fn eval_tree(node: &Node) -> f64
{
    match node
    {
        Node::Number(n) =>
        {
            return *n;
        }
        Node::BinaryExpr { left, op, right } =>
        {
            let left_result = eval_tree(left);
            let right_result = eval_tree(right);
            match op
            {
                '+' => left_result + right_result,
                '-' => left_result - right_result,
                '*' => left_result * right_result,
                '/' => left_result / right_result,
                '%' => left_result % right_result,
                '^' => left_result.powf(right_result),
                _ => f64::NAN,
            }
        }
        Node::UnaryExpr { op, child } =>
        {
            let child_result = eval_tree(child);
            match op
            {
                '-' => -child_result,
                '!' => factorial(child_result as u64) as f64,
                _ => child_result,
            }
        }
        Node::FunctionCall { name, arg } =>
        {
            let arg_result = eval_tree(arg);
            match name.as_str()
            {
                "sin" => arg_result.sin(),
                "cos" => arg_result.cos(),
                "sinh" => arg_result.sinh(),
                "cosh" => arg_result.cosh(),
                "sqrt" => arg_result.sqrt(),
                _ =>
                {
                    panic!("unknown function {}", name);
                }
            }
        }
    }
}

fn factorial(n: u64) -> u64
{
    (1..=n).product()
}

//function to print a expression tree
fn print_tree(node: &Node, indent: usize)
{
    let spacing = "   ".repeat(indent);
    match node
    {
        Node::Number(n) =>
        {
            println!("{}number: {}", spacing, n);
        }
        Node::BinaryExpr { left, op, right } =>
        {
            println!("{}operator: {}", spacing, op);
            print_tree(left, indent + 1);
            print_tree(right, indent + 1);
        }
        Node::UnaryExpr { op, child } =>
        {
            println!("{}operator: {}", spacing, op);
            print_tree(child, indent + 1);
        }
        Node::FunctionCall { name, arg } =>
        {
            println!("{}function: {}", spacing, name);
            print_tree(arg, indent + 1);
        }
    }
}

// use crossterm::{
//     ExecutableCommand,
//     event::{self, KeyCode, KeyEvent, KeyEventKind},
//     terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
// };a
// use ratatui::{
//     Frame, Terminal,
//     backend::CrosstermBackend,
//     layout::{Constraint, Direction, Layout},
//     widgets::{Block, Borders, Paragraph},
// };
// use std::io::{Result, stdout};

// fn main() -> Result<()> {
//     // --- 1. Terminal Setup ---
//     // Switch to the alternate screen (so the terminal clears on start)
//     stdout().execute(EnterAlternateScreen)?;
//     // Enable raw mode to capture keyboard input immediately
//     enable_raw_mode()?;

//     // Initialize the Ratatui terminal with the Crossterm backend
//     let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

//     // Application State
//     let mut input = String::new();
//     let mut history: Vec<String> = Vec::new();

//     // --- 2. Main Application Loop ---
//     loop {
//         // Render the user interface
//         terminal.draw(|f| draw_ui(f, &input, &history))?;

//         // Check for terminal events (keyboard, resize, etc.)
//         if event::poll(std::time::Duration::from_millis(16))? {
//             if let event::Event::Key(key) = event::read()? {
//                 // Only process the event if a key was pressed (ignores release events)
//                 if key.kind == KeyEventKind::Press {
//                     // Pass input logic to our handler
//                     let should_quit = handle_input(key, &mut input, &mut history);

//                     // Exit the loop if the handler returns true
//                     if should_quit {
//                         break;
//                     }
//                 }
//             }
//         }
//     }

//     // --- 3. Cleanup ---
//     // Restore the terminal to its original state before exiting
//     disable_raw_mode()?;
//     stdout().execute(LeaveAlternateScreen)?;
//     Ok(())
// }

// /// Processes keyboard input and updates the application state.
// /// Returns true if the application should exit.
// fn handle_input(key: KeyEvent, input: &mut String, history: &mut Vec<String>) -> bool {
//     match key.code {
//         // Quit the application
//         KeyCode::Char('q') => {
//             return true;
//         }

//         // Append characters to the current input string
//         KeyCode::Char(c) => {
//             input.push(c);
//         }

//         // Remove the last character
//         KeyCode::Backspace => {
//             input.pop();
//         }

//         // Submit the calculation to history
//         KeyCode::Enter => {
//             if !input.is_empty() {
//                 history.push(format!("{}", input));
//                 input.clear();
//             }
//         }

//         _ => {}
//     }
//     false
// }

// /// Renders the UI widgets into the terminal frame.
// fn draw_ui(f: &mut Frame, input: &str, history: &[String]) {
//     // Define vertical layout sections
//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([
//             Constraint::Min(0),    // Top: History (grows to fill space)
//             Constraint::Length(3), // Middle: Input field (fixed height)
//             Constraint::Length(1), // Bottom: Status/Help bar
//         ])
//         .split(f.area());

//     // 1. History Widget (shows previous inputs)
//     f.render_widget(
//         Paragraph::new(history.join("\n"))
//             .block(Block::default().title(" unicalc ").borders(Borders::ALL)),
//         chunks[0],
//     );

//     // 2. Input Widget (shows what the user is currently typing)
//     f.render_widget(
//         Paragraph::new(input).block(Block::default().title(" Input ").borders(Borders::ALL)),
//         chunks[1],
//     );

//     // 3. Footer Widget (simple help text without borders)
//     f.render_widget(
//         Paragraph::new("[Q] quit | [Enter] submit").block(Block::default().borders(Borders::NONE)),
//         chunks[2],
//     );
// }
