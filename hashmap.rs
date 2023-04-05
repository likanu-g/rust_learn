use std::collections::HashMap;
fn main() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    let zhangsan1 = map.insert("zhangsan",97);
    map.insert("lisi",64);
    println!("{:?}",zhangsan1);
    println!("{:?}",map);
    map.insert("world",1);
    println!("{:?}",map);
}