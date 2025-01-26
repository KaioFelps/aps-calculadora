use crate::{helpers::montar_calculadora, traits::Calculadora};

#[test]
fn soma() {
    let calculadora = montar_calculadora();
    let teclado = calculadora
        .obtenha_teclado()
        .as_ref()
        .unwrap()
        .read()
        .unwrap();

    teclado.procure_tecla("-x").unwrap().pressione();
    teclado.procure_tecla("1").unwrap().pressione();
    teclado.procure_tecla("2").unwrap().pressione();
    teclado.procure_tecla("3").unwrap().pressione();
    teclado.procure_tecla("+").unwrap().pressione();
    teclado.procure_tecla("5").unwrap().pressione();
    teclado.procure_tecla("0").unwrap().pressione();
    teclado.procure_tecla("=").unwrap().pressione();
}
