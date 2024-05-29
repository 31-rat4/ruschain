mod modules {
    pub mod blockchain;
}
use modules::blockchain::BlockChain;

fn print_type_of<T>(_: &T) {
    println!("typeof : {}", std::any::type_name::<T>())
}

fn main() {
    let mut teste: BlockChain = BlockChain::new();
    teste.create_new_block();
    teste.create_new_block();
    teste.create_new_block();
    let ve1 = teste.chain;
    println!("First block timestamp: {:?}", ve1);
}
