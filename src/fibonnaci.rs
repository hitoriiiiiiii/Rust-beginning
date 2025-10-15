fn main(){
    printIn!("{}", fib(10));
}

fn fib(num: u32){
    let mut first = 0;
    let mut second = 1;

    if (num == 0){
        return first;
    } else if (num == 1){
        return second;
    }
    for _ in 0 ..(num-2){
        let temp = second;
        second = first + second;
        first = temp;
    }
    return second;
}