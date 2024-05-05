use hello_macro::HelloMacro;

#[allow(unused)]
fn add_one(x:i64)->i64{
    x+1
}


#[allow(unused)]
fn make_it_twice(f:fn(i64)->i64,arg:i64)->i64{
    f(arg)+f(arg)
}

#[allow(unused)]
fn closure_or_function(){

    let some_numbers = vec![1,2,3];
    //using closures
    let some_chains: Vec<String> =
    some_numbers.iter().map(|i| i.to_string()).collect();
    //using function
    let some_chains:Vec<String> =
    some_numbers.iter().map(ToString::to_string).collect();
}

#[allow(unused)]
fn return_closure()-> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x+1)
}

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hey macro, my name is pancakes")
    }
}

fn main() {
    Pancakes::hello_macro();
}
