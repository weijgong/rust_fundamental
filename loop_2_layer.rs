#[allow(unreachable_code)]

fn main(){
    'outer: loop{
        println!("Enter the outer loop");
        'inner: loop{
            println!("Enter the inner loop");
            break 'outer;
        }
        println!("if Outer not be break, I will be reached");
    }
    println!("the outer loop break!");
}