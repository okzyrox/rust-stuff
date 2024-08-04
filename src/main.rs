//mod modstuff;
//mod tasks;
mod serialstuff;

fn main() {
    //modstuff::testmod::all_test();
    /*
    let mut inp1 = String::new();
    let mut inp2 = String::new();
    let mut op = String::new();

    while running {
        println!("Number one: ");
        std::io::stdin().read_line(&mut inp1).expect("Failed to read line");
        println!("Operation (+,-,*,/,%,^): ");
        std::io::stdin().read_line(&mut op).expect("Failed to read line");
        println!("Number two: ");
        std::io::stdin().read_line(&mut inp2).expect("Failed to read line");

        let num1 = inp1.trim().parse::<i32>().unwrap();
        let num2 = inp2.trim().parse::<i32>().unwrap();
        let result = match op.trim() {
            "+" => modstuff::mathstuff::add(num1, num2),
            "-" => modstuff::mathstuff::sub(num1, num2),
            "*" => modstuff::mathstuff::mul(num1, num2),
            "/" => modstuff::mathstuff::div(num1, num2),
            "%" => modstuff::mathstuff::rem(num1, num2),
            "^" => modstuff::mathstuff::pow(num1, num2.try_into().unwrap()),
            _ => 0
        };

        println!("Result: {}", result);
        println!("");
    } */
   //let mut entity_manager = modstuff::entity::EntityManager::new();
    /*
    let mut input_name = String::new();
    let mut input_id = String::new();

    println!("Enter the entity name: ");
    std::io::stdin().read_line(&mut input_name).expect("Failed to read line");

    println!("Enter the entity id: ");
    std::io::stdin().read_line(&mut input_id).expect("Failed to read line");
    let id = input_id.trim().parse::<i32>().unwrap();
    
    let entity = entity_manager.new_entity(id, input_name.trim().to_string(), 0, 0).expect("Failed to create entity");

    entity.print();
    println!("Hello!: {}", entity.name);
    
     */
    
    //tasks::run();

    serialstuff::run();
    print!("Goodbye!");
}