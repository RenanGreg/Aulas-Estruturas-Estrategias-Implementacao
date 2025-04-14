use std:: collections:: VecDeque; 

fn main (){
	
	let mut fila: VecDeque<i32> = VecDeque:: new (); 
	
	fila.push_back(1); 
	fila.push_back(2); 
	fila.push_back(3); 
	
	println!("{:?"}, fila); 
	
	println!("{:?}", fila.pop_front()); 
	
	println!("{:?}", fila); 
} 
