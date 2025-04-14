use std::io;

fn main() {
    // Solicita o nome do usuário
    println!("Qual é o seu nome?");
    let nome = ler_entrada();

    // Solicita a idade do usuário
    println!("Quantos anos você tem?");
    let idade: i32 = ler_entrada().trim().parse().expect("Por favor, insira um número válido para a idade.");

    // Solicita a cidade do usuário
    println!("De qual cidade você é?");
    let cidade = ler_entrada();

    // Exibe as informações
    println!("\nInformações Cadastradas:");
    println!("Nome: {}", nome);
    println!("Idade: {}", idade);
    println!("Cidade: {}", cidade);
}

// Função para ler entrada do usuário
fn ler_entrada() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler a entrada.");
    entrada.trim().to_string() // Remove espaços extras e retorna a string
}
