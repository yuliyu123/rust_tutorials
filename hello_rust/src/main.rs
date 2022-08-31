use std::time::{Duration, Instant};

fn main() {
    println!("0x0000_0000_0000_0203 & 0x0000_0000_0000_00ff) {:?}", (0x0000_0000_0000_0203 & 0x0000_0000_0000_00ff));
    println!("0x0000_0000_0000_0303 & 0x0000_0000_0000_00ff) {:?}", (0x0000_0000_0000_0303 & 0x0000_0000_0000_00ff));
}

// Time elapsed in expensive_function() is: 99.851526583s
