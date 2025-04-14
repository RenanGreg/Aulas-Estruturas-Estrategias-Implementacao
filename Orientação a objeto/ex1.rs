// Estrutura do produto
struct Produto {
    nome: String,
    quantidade: u32,
}

impl Produto {
    // Construtor para criar um novo produto
    fn new(nome: &str, quantidade: u32) -> Produto {
        Produto {
            nome: nome.to_string(),
            quantidade,
        }
    }

    // exibir o produto
    fn exibir(&self) {
        println!("Produto: {}, Quantidade: {}", self.nome, self.quantidade);
    }

    // altera a quantidade do produto
    fn alterar_quantidade(&mut self, nova_quantidade: u32) {
        self.quantidade = nova_quantidade;
    }
}

fn main() {
    // Cria um produto
    let mut produto = Produto::new("Arroz", 50);

    // Exibi o produto
    produto.exibir();

    // Altera a quantidade
    produto.alterar_quantidade(100);
  
    produto.exibir();
}
