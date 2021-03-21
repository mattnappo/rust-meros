use rust_meros::p2p::node::Node;

fn run_node() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 {
        Node::new(args[1].as_str())
            .unwrap()
            .start_listening(args[2].parse::<u16>().unwrap());
    }
    panic!("must specify an identity and a port");
}

fn main() {
    run_node();
}
