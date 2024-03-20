type PiecePosition = u64;

fn bit_to_position(bit: PiecePosition) -> Result<String, String> {
    if bit == 0 {
        return Err("No piece present!".to_string());
    } else {
        let onebit_index = bit_scan(bit);
    }
}

fn main() {
    
    
    println!("{}", (1 as u64) << 55);
    println!("{}", (1 as u64) << 8);
    println!("{}", (1 as u64) << 10);
    println!("{}", (1 as u64) << 23);

    println!("Hello, world!");
}
