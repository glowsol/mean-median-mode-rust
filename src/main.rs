use std::collections::HashMap;
use std::io;
fn main() {
    let mut list = get_list();
    let mut total:u16 = 0;
    let mut count:u16 = 0;
    let mut hashm = HashMap::new();
    for i in &list{
        total+=i;
        let point = hashm.entry(*i).or_insert(0);
        *point += 1;
        count+=1;
        
    }
    println!("Mean:{}",total/count);
    list.sort();
    let center = count/2;
    let center = center as usize;
    println!("Median:{},index:{}",list[center],center+1);
    let mut max_v = 0;
    let mut max_k = 0;
    for (k,v) in hashm{
        if v>max_v {
            max_v = v;
            max_k=k;
        }
    }
    println!("Mode:{}, count:{}",max_k,max_v);
}
fn get_list()->Vec<u16>{
    let mut list:Vec<u16> = vec![];
    println!("Please keep adding the list values, enter 'E' when done!");
    loop{
        let mut value = String::new();
        io::stdin().read_line(&mut value).expect("Unexpected value intered!");
        let value = value.trim();
        match value{
            "e"|"E"=>{return list;},
            _=>()
        };       
        let value:u16 = match value.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please enter a valid interger value");
                continue;
            }
        };
        list.push(value);
    }
}