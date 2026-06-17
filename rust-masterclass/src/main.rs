mod fundamentos;
mod utils;

use std::process::exit;
use utils::terminal::{exibir_menu, limpar_tela};

// use crate::utils::terminal::limpar_tela;

fn main() {
    loop {
        let itens = ["Fundamentos", "Tipos", "Controle", "Funções", "Ownership"];

        let selecionado = exibir_menu("Principal", &itens, true);

        limpar_tela();

        match selecionado {
            1 => fundamentos::executar(),
            2 => println!("2"),
            3 => println!("3"),
            4 => println!("4"),
            5 => println!("5"),
            _ => exit(0),
        }
    }
}
