// Serial checker

use json;
use std::fs;

const SERIALS_LIST: [&str; 6] = [ "XAW1", "XAW4", "XAW7", "XAJ1", "XAJ4", "XAJ7"];

const EXTENDED_SERIALS_LISTS: [&str; 6] = ["XKW1", "XKJ1", "XJW1", "XWW1", "XAW9", "XAK"];



pub fn run() {
    let serial_ranges_f = fs::read_to_string("resources/ranges.json").expect("Failed to read file");
    let serial_ranges = json::parse(&serial_ranges_f).expect("Failed to parse JSON");

    let mut serial_full = String::new();
    let mut serial_code = String::new();

    println!("Enter serial code (first 4 characters): ");
    std::io::stdin().read_line(&mut serial_code).expect("Failed to read line");
    if serial_code.trim() == "" {
        println!("You must enter a serial code");
        return;
    }
    if SERIALS_LIST.contains(&serial_code.trim()) {
        println!("Valid serial code entered");
    } else if EXTENDED_SERIALS_LISTS.contains(&serial_code.trim()) {
        println!("Extended serial code entered");
        return;

    } else {
        println!("Invalid serial code entered");
        return;
    }
    
    println!("Enter the full serial code: ");
    std::io::stdin().read_line(&mut serial_full).expect("Failed to read line");
    let serial_full = &serial_full[4..14];
    // i am assuming everyone who uses this is smart enough
    // to enter the right character amount
    // otherwise we panic

    let code_ranges = &serial_ranges[serial_code.trim()];

    let code_ranges_u = &code_ranges["unpatched"];
    //let code_ranges_pp = &code_ranges["possibly_patched"];
    //let code_ranges_p = &code_ranges["patched"];

    //let mut assembled_pp: [i64; 3];
    //let mut assembled_p: [i64; 2];
    let code = serial_full.to_string();

    let mut assembled_u = [
        code_ranges_u["min"].to_string(), 
        code.clone(),
        code_ranges_u["max"].to_string(),
    ];
    assembled_u.sort(); 
    // we have to use a cloned 'code' because of the sorting
    // kinda weird but i guess it makes sense

    println!("{} {} {}", assembled_u[0], assembled_u[1], assembled_u[2]);

    if assembled_u[1] == code { 
        // if middle is the same as the code
        // its in the valid ranges
        // and is valid
        println!("Unpatched!");
    } else {
        println!("Patched!");
    }
    
}
