use std::collections::HashMap;
fn main() {
    let mut list = vec![20,45,53,50,20];
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

