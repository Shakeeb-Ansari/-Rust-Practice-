struct Gpu {            // Traditional Struct
    vendor: String,
    vram: String,
    bits: u32,
}

struct Processor (String, f32);      // Tuple Struct

pub fn run() {
    let a = Gpu {                   // For Traditional Strcut
        vendor: "Asus".to_string(),
        vram: "4gb".to_string(),
        bits: 256,
    };

    let b = Processor(" Intel i5 9400f".to_string(), 2.9);     // For Tuple Struct

    println!("My Gpu has {} vram , {} bits bus width and vendor is {}", a.vram,a.bits,a.vendor);
    println!("My processor's model is {} and it's base clock speed is {} GHz", b.0, b.1);
    
}