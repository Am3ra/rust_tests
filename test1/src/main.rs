fn main() {
    convert_to_celcius(32);
    convert_to_celcius(-40);
    nth_fibonacci(3);
    nth_fibonacci(5);
}


fn convert_to_celcius(x:i32){
    println!("{}",(x-32)*5/9);
}

fn nth_fibonacci(n : i32) {
    let (mut x,mut y)=(1,1);
    for _number in 1..n{
        x = x+y;
        y = x-y;
    }
    println!("{}", x);
}