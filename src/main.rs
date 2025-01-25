#![allow(unused)]

use calculadora::CalculadoraKaio;
use enums::{Digito, Sinal};
use tecla::TeclaKaio;
use teclado::TecladoKaio;
use tela::TelaKaio;
use traits::{Calculadora, IntoDynamicMutable};
use ucp::UcpKaio;

mod calculadora;
mod enums;
mod pilha_de_digitos;
mod tecla;
mod teclado;
mod tela;
mod traits;
mod ucp;

fn main() {
    // Montando a calculadora como numa linha de produção
    let tecla1 = TeclaKaio::new();

    let teclado1 = TecladoKaio::new().into_dynamic_mutable();
    teclado1.write().unwrap().adicione_tecla(Box::new(tecla1));

    let tela1 = TelaKaio::new().into_dynamic_mutable();

    let ucp1 = UcpKaio::new().into_dynamic_mutable();
    ucp1.write().unwrap().defina_tela(tela1.clone());

    let mut calculadora1: CalculadoraKaio = CalculadoraKaio::new();
    calculadora1.defina_tela(tela1);
    calculadora1.defina_teclado(teclado1);
    calculadora1.defina_ucp(ucp1);

    // Testando a calculadora
    calculadora1
        .obtenha_tela()
        .unwrap()
        .write()
        .unwrap()
        .adicione(Digito::Um);

    calculadora1
        .obtenha_tela()
        .unwrap()
        .write()
        .unwrap()
        .limpe();

    calculadora1
        .obtenha_tela()
        .unwrap()
        .write()
        .unwrap()
        .defina_sinal(Sinal::Negativo);

    calculadora1
        .obtenha_tela()
        .unwrap()
        .write()
        .unwrap()
        .adicione(Digito::Um);

    calculadora1
        .obtenha_tela()
        .unwrap()
        .write()
        .unwrap()
        .adicione(Digito::Zero);

    calculadora1
        .obtenha_tela()
        .unwrap()
        .write()
        .unwrap()
        .defina_separador_decimal();

    calculadora1
        .obtenha_tela()
        .unwrap()
        .write()
        .unwrap()
        .adicione(Digito::Um);

    println!("All is okay!");
}
