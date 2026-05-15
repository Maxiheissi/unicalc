mod eval;

// program entry
fn main()
{
    let input = "(-6+2)*sin(1)%2";
    let tokens = eval::tokenize_expression(input);

    let mut it = tokens.into_iter().peekable();
    match eval::parse_expression(&mut it)
    {
        Ok(root) =>
        {
            eval::print_tree(&root, 0);
            match eval::eval_tree(&root)
            {
                Ok(result) => println!("result: {}", result),
                Err(e) => println!("eval error: {:?}", e),
            }
        }
        Err(e) => println!("parse error: {:?}", e),
    }
}
