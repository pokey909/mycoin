extern crate blockchain;
use blockchain::chain::Chain;
use blockchain::network::client;

fn main() {
    let mut c = Chain::new();
    c.add_block("first block");
    c.add_block("next block");

    println!("Chain valid: {}", c.is_valid_chain());
    // c.print_chain();
    println!("{}",c.to_string());
    client::start_client(8000, c);
}

