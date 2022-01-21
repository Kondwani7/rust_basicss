pub fn run(){
    greeting("hey","Kondwani");
    //bind function values to variables
    let get_sum = add(21, 5);
    println!("Sum: {}", get_sum);
    //closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2+ n3;
    println!("C Sum:{}", add_nums(4,4));
    //greeting function
    fn greeting(greet: &str, name: &str){
        println!("{} {}, nice to meet you", greet, name);
    }
    fn add(n1: i32, n2: i32) -> i32{
        n1 + n2
    }
}