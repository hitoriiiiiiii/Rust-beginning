fn main(){
    let v1 = vec![1, 2, 3, 4, 5];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Value: {}", val);
    }
    prinln!(" {:?}", v1); //borrowing is used here so v1 is still accessible
}

//Mutuable iterators
fn main(){
    let mut nums = vec![1,2,3,4,5];

    let iter = nums.iter_mut();

    for val in iter {
        *val = *val + 10;
    }
    println!("Updated Vector: {:?}", nums); //nums is still accessible here
}

//Iterator using .next()
fn main(){
    let nums = vec![10,20,30];

    let mut iter = nums.iter();

    while let Some(val) = iter.next() {
        println!("Value: {}", val); 
    }
}

//Iterators adaptors are methods that change iterators into different kinds of iterators

fn main(){
    let v1 = vec![1,2,3,4,5];
    let v1_iter = v1.iter();

    let v1_iter2 = v1_iter.map(|x| x + 1);

    for i in v1_iter2 {
        println!("Value: {}", i);
    }
}

//Using filter adaptor
fn main(){
    
}