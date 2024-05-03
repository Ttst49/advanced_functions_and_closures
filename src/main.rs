fn add_one(x:i64)->i64{
    x+1
}

fn make_it_twice(f:fn(i64)->i64,arg:i64)->i64{
    f(arg)+f(arg)
}

fn main() {
    let response = make_it_twice(add_one,5);
    println!("The response is {}",response)
}
