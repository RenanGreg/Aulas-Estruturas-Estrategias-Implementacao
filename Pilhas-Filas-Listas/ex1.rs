fn main() {
    struct Pessoa {
        nome: String,
        altura: f32,
        idade: f32, 
    }

    let pessoa = Pessoa {
        nome: String::from("Ana"),
        idade: 28.0, 
        altura: 1.65, 
    };

    println!("Nome: {}, Idade: {}, Altura: {}," pessoa.nome, pessoa.idade, pessoa.altura);
}
