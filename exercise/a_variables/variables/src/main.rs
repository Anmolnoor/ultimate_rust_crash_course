

const MISSILES : i32 = 8;
const READY : i32 = 2; 

fn main() {
    let (missiles, ready) = (MISSILES, READY);
    let _unused:u32;
    // READY = 4; //   cannot assign to this expression
    println!("Firing {} of {} missiles...", ready, missiles);
    // missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}
