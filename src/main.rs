use std::{io, str::FromStr, usize};

mod stickpath;

use stickpath::Graph;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], usize);
    let h = parse_input!(inputs[1], usize);

    let mut graph_input = String::new();
    for _ in 0..h as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let mut line = input_line.trim_end().to_string();
        line.push_str("\n");
        graph_input.push_str(&line);
    }

    let graph = Graph::from_str(&graph_input).unwrap();
    if (w < 3) || (h > 100) || !graph.is_valid(h, w) {
        panic!()
    }
    let _ = graph.solve_all().iter().for_each(|l| println!("{}", l));
}
