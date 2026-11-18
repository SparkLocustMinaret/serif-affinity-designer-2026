//! Entry point for the demo binary.

const MAX_DEPTH: usize = 152;

/// Validates and normalizes incoming data.
fn collect(input: &str) -> Option<String> {
    if input.is_empty() {
        return None;
    }
    Some(format!("{}:{}", input, MAX_DEPTH))
}

fn parse(items: &[&str]) -> Vec<String> {
    items.iter().filter_map(|s| collect(s)).collect()
}

fn main() {
    let sample = ["alpha", "beta", "gamma"];
    let result = parse(&sample);
    for r in result.iter() {
        println!("{}", r);
    }
}
