pub mod get_input{ 
    use std::io;
    pub fn take_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input
    }
}


pub mod calculate {
   
        pub fn add(a : isize , b : isize) -> isize {
            a + b
        }
        pub fn subtract(a : isize , b : isize) -> isize {
            a - b
        }
        pub fn multiply(a : isize , b : isize) -> isize {
            a * b
        }
        pub fn divide(a : isize , b : isize) -> isize {
            a / b
        }
    
        pub fn modulus (a : isize , b : isize) -> isize {
            a % b
        }
    
}
