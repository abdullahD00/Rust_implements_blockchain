use myblockchainlib::*;

fn main () {
    let difficulty_const = 0x000fffffffffffffffffffffffffffff;

    let mut initial_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Abdullah".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "DÜNDAR".to_owned(),
                    value: 7,
                },
            ],
        },
    ], difficulty_const);

    initial_block.mine();

    println!("Mined initial block {:?}", &initial_block);

    let mut last_hash = initial_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(initial_block).expect("Failed to add initial block");

    let mut block = Block::new(1, now(), last_hash, vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Rust".to_owned(),
                    value: 536,
                },
            ],
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[0].clone(),
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Türkiye".to_owned(),
                    value: 360,
                },
                transaction::Output {
                    to_addr: "Sinop".to_owned(),
                    value: 12,
                },
            ],
        },
    ], difficulty_const);

    block.mine();

    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    blockchain.update_with_block(block).expect("Failed to add block");
}