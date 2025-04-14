fn main(){

    let mut pilha: Vec<&str> = Vec::new(); 
    
    pilha.push("Joao"); 
    pilha.push("Carlos"); 
    pilha.push("Ana"); 
    
    println!("Pilha inicial: {:?}", pilha); 
    
    let topo = pilha.pop();
    
    println!("Topo da pinha: {:?}", topo); 
    println!("Pilha apos remoção: {:?}", pilha); 
    
    pilha.push("Ana"); 
    pilha.push("Lucas"); 
    
    println!("Pilha apos mais inserções: {:?}", pilha); 
    
    let topo_2 = pilha.pop(); 
    
    println!("Novo topo da linha: {:?}", topo_2); 
    println!("Pilha final: {:?}", pilha);
}
