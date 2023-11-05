fn main(){
    let mut names = vec!["Bob","Frank","Ferris"];
    for name in names.iter_mut(){
        // 在这里会被直接修改
        *name = match name{
            &mut "Ferris" => "There is a rustacean among us",
            _ => "Hello",
        }
    }
    println!("{:?}",names);
}