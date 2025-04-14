use std::collections::VecDeque;

fn main(){
    
    let mut fila: VecDeque<&str> = VecDeque:: new(); 
    
    fila.push_back("Joao"); 
    fila.push_back("Maria"); 
    fila.push_back("Renan"); 
    
    println!("Fila inicial{:?}", fila); 
    
    let primeiro = fila.pop_front();
    
    println!("Primeiro da fila é: {:?}", primeiro);
    println!("Removendo da fila: {:?}", fila); 
    
    fila.push_back("Ana"); 
    fila.push_back("Lucas"); 
    
    println!("Fila apos mais inserções: {:?}", fila); 
    
    let segundo = fila.pop_front(); 
    
    println!("Segundo da fila: {:?}", segundo);
    println!("Fila final: {:?}", fila);
}
