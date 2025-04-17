use std::collections::HashMap;

fn main() {
    let mut capitais = HashMap::new();

    capitais.insert("Brasil", "Brasília");
    capitais.insert("França", "Paris");
    capitais.insert("Japão", "Tóquio");


    if let Some(capital) = capitais.get("França") {
    println!("Capital da França: {}", capital);
    }

    for (pais, capital) in &capitais {
    println!("{} → {}", pais, capital);
    }

    if let Some(capital) = capitais.get("França") {
        println!("A capital da França é {}", capital);
        } else {
        println!("França não encontrada.");
    }
        capitais.remove("Japão");
        println!("Japão removido!");

    if capitais.contains_key("Brasil") {
        println!("Brasil está no mapa.");
    } else {
        println!("Brasil não está no mapa.");
        }
    
println!("\nCapitais restantes:");
    for (pais, capital) in &capitais {
        println!("{} → {}", pais, capital);
    }

}
