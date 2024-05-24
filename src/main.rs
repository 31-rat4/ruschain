mod modules {
    pub mod blockchain;
}
use modules::blockchain::BlockChain;

fn print_type_of<T>(_: &T) {
    println!("typeof : {}", std::any::type_name::<T>())
}

fn main() {
    let teste: BlockChain = BlockChain::new();
    let ve1 = teste.blocks;
    print_type_of(&ve1);
}
