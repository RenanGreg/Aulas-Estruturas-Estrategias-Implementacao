fn main() {
    let precos = vec![
        vec![15.00, 12.50],
        vec![13.00, 7.50], 
        vec![100.00, 97.00],
    ];
    
    let mut medias = Vec::new();
  
    for material in &precos {
        let media = (material[0] + material[1]) / 2.0;
        medias.push(media);
    }
    
    println!("Médias de preços por produto:");
    for (indice, media) in medias.iter().enumerate() {
        println!("MATERIAL[{}]: {:.2}", indice, media);
    }
}
