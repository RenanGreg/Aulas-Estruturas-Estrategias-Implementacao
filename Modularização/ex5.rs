use std::collections::LinkedList; 

fn main(){
    let mut lista: LinkedList<i32> = LinkedList::new(); 
    
    lista.push_back(10); 
    lista.push_back(20); 
    lista.push_back(30); 
    
    println!("{:?}", lista); 
} 
