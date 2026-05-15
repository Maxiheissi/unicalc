mod eval;

pub struct HystoryEntry
{
    input: String,
    node: Node,
    result: f64,
}

pub struct App
{
    input: String,
    history: Vec<HystoryEntry>,
    quit: bool,
}
