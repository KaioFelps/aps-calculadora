#![allow(unused)]

use calculadora::CalculadoraKaio;
use enums::{Digito, Sinal};
use helpers::montar_calculadora;
use tecla::TeclaKaio;
use teclado::TecladoKaio;
use tela::TelaKaio;
use traits::{
    Calculadora, IntoDynamicMutable, UnwrapAndWrite, UnwrapDynamicMutableRef, UnwrapOptionalAsRef,
};
use ucp::UcpKaio;

mod calculadora;
mod enums;
mod helpers;
mod pilha_de_digitos;
mod tecla;
mod teclado;
mod tela;
mod traits;
mod ucp;

#[cfg(test)]
mod tests;

fn main() {
    // Montando a calculadora como numa linha de produção
    let calculadora = montar_calculadora();
    // Testando a calculadora
    let tela = calculadora.obtenha_tela();
    let mut tela = tela.get_write_ref();

    tela.adicione(Digito::Um);
    tela.limpe();

    tela.defina_sinal(Sinal::Negativo);

    tela.adicione(Digito::Um);
    tela.adicione(Digito::Zero);
    tela.defina_separador_decimal();
    tela.adicione(Digito::Um);

    println!("All is okay!");
}
