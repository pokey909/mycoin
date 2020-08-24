use sha2::{Sha256, Digest};
use sha2::digest::FixedOutput;
use rand::Rng;
use rand::distributions::Uniform;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::prelude::ThreadRng;
use futures::{SinkExt, StreamExt};

#[derive(Clone)]
pub struct Block {
    pub index: usize,
    pub hash: Vec<u8>,
    pub prev_hash: Vec<u8>,
    pub ts: u128,
    pub data: String,
}

impl Block {
    pub fn to_string(&self) -> String {
        format!("index: {}\nhash: {:x?}\nprev_hash:{:x?}\nts: {}\ndata:{}",
                self.index,
                self.hash,
                self.prev_hash,
                self.ts,
                self.data
        )
    }
}

pub fn print_block(block: &Block) {
    println!("index: {}\nhash: {:x?}\nprev_hash:{:x?}\nts: {}\ndata:{}",
             block.index,
             block.hash,
             block.prev_hash,
             block.ts,
             block.data
    )
}

pub struct Chain {
    blocks: Vec<Block>
}

impl Chain {
    fn hash_block(mut block: Block) -> Block {
        let mut hasher = Sha256::new();
        hasher.update(block.index.to_le_bytes());
        hasher.update(block.prev_hash.as_slice());
        hasher.update(block.ts.to_le_bytes());
        hasher.update(block.data.as_bytes());
        block.hash = hasher.finalize_fixed().to_vec();
        block
    }

    fn new_block(prev_blocks: Option<&Block>, data: &str) -> Block {
        let b = Block {
            index: match prev_blocks {
                Some(block) => block.index + 1,
                None => 0,
            },
            hash: vec![],
            prev_hash: match prev_blocks {
                Some(block) => block.hash.clone(),
                None => (0..32).map(|_| ThreadRng::default().sample(&Uniform::<u8>::new(0x0, 0xff))).collect(),
            },
            ts: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis(),
            data: data.to_string(),
        };
        Chain::hash_block(b)
    }

    fn is_valid_block(prev_block: &Block, block: &Block) -> bool{
        prev_block.index == block.index - 1 &&
            prev_block.hash == block.prev_hash &&
            Chain::hash_block(block.clone()).hash == block.hash
    }

    fn replace_chain(&mut self, other_chain: &Vec<Block>) {
        if self.is_valid_chain() && other_chain.len() > self.blocks.len() {
            println!("Found longer chain! Swapping chains...");
            self.blocks = other_chain.to_vec();
            self.broadcast_latest();
        } else {
            println!("Received chain is invalid. Dropping it.");
        }
    }

    fn broadcast_latest(&self) {

    }
    pub fn new() -> Chain {
        Chain {
            blocks: vec![
                Chain::new_block(None, "genesis block")
            ]
        }
    }

    pub fn add_block(&mut self, data: &str) -> Option<&Block> {
        self.blocks.push(Self::new_block(
            self.blocks.last(),
            data,
        ));
        self.blocks.last()
    }

    pub fn is_valid_chain(&self) -> bool {
        for i in 1..self.blocks.len() {
            if !Chain::is_valid_block(&self.blocks[i - 1], &self.blocks[i]) { return false }
        }
        true
    }

    pub fn print_chain(&self) {
        for block in &self.blocks {
            println!("{}",block.to_string());
        }
    }

    pub fn to_string(&self) -> String {
        (&self.blocks).into_iter().map(|s| s.to_string()).collect::<String>()
    }

    pub fn print_nth_block(&self, pos: usize) {
        print_block(&self.blocks[pos]);
    }

    pub fn tamper_block(&mut self, i: usize) {
        self.blocks[i].data = "Harhahraharhar tampered".to_string();
    }
}