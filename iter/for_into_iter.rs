fn main(){
    let names = vec!["Bob","Frank","Ferris"];
    for name in names.into_iter(){
        match name{
            "Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello {}",name),
        }
    }
    // 在使用了into_iter后，数组内部的数据会被删除，调用下面的函数会报错 
    // println!("{:?}",names);
}