use std::{
    rc::Rc,
    sync::{RwLock, RwLockWriteGuard},
};

use crate::{
    calculadora::CalculadoraKaio,
    enums::{Ação, Controle, Digito, Operação, Sinal},
    tecla::TeclaKaio,
    teclado::TecladoKaio,
    tela::TelaKaio,
    traits::{
        Calculadora, DynamicMutable, IntoDynamicMutable, Recebedor, Tecla, Teclado, Ucp,
        UnwrapAndWrite, UnwrapDynamicMutableRef,
    },
    ucp::UcpKaio,
};

pub fn montar_calculadora() -> impl Calculadora {
    let teclado = TecladoKaio::new().into_dynamic_mutable();
    montar_teclado(&teclado);

    let tela = TelaKaio::new().into_dynamic_mutable();

    let ucp = UcpKaio::new().into_dynamic_mutable();
    ucp.write().unwrap().defina_tela(tela.clone());

    teclado.write().unwrap().defina_recebedor(ucp.clone());

    let mut calculadora = CalculadoraKaio::new();
    calculadora.defina_tela(tela);
    calculadora.defina_teclado(teclado);
    calculadora.defina_ucp(ucp.clone());

    calculadora
}

fn montar_teclado(teclado: &DynamicMutable<Box<dyn Teclado>>) {
    // region: --- Digitos
    let mut zero = TeclaKaio::new("0").com_ação(Ação::Di(Digito::Zero));
    let mut um = TeclaKaio::new("1").com_ação(Ação::Di(Digito::Um));
    let mut dois = TeclaKaio::new("2").com_ação(Ação::Di(Digito::Dois));
    let mut três = TeclaKaio::new("3").com_ação(Ação::Di(Digito::Três));
    let mut quatro = TeclaKaio::new("4").com_ação(Ação::Di(Digito::Quatro));
    let mut cinco = TeclaKaio::new("5").com_ação(Ação::Di(Digito::Cinco));
    let mut seis = TeclaKaio::new("6").com_ação(Ação::Di(Digito::Seis));
    let mut sete = TeclaKaio::new("7").com_ação(Ação::Di(Digito::Sete));
    let mut oito = TeclaKaio::new("8").com_ação(Ação::Di(Digito::Oito));
    let mut nove = TeclaKaio::new("9").com_ação(Ação::Di(Digito::Nove));

    zero.defina_teclado(teclado.clone());
    um.defina_teclado(teclado.clone());
    dois.defina_teclado(teclado.clone());
    três.defina_teclado(teclado.clone());
    quatro.defina_teclado(teclado.clone());
    cinco.defina_teclado(teclado.clone());
    seis.defina_teclado(teclado.clone());
    sete.defina_teclado(teclado.clone());
    oito.defina_teclado(teclado.clone());
    nove.defina_teclado(teclado.clone());
    // endregion: --- Digitos

    // region: --- Controles
    let mut desliga = TeclaKaio::new("OFF").com_ação(Ação::Ctrl(Controle::Desliga));
    let mut igual = TeclaKaio::new("=").com_ação(Ação::Ctrl(Controle::Igual));
    let mut limpa_erro = TeclaKaio::new("CE").com_ação(Ação::Ctrl(Controle::LigaLimpaErro));
    let mut memoria_leitura_escrita =
        TeclaKaio::new("MR").com_ação(Ação::Ctrl(Controle::MemóriaLeituraEscrita));

    let mut memoria_soma = TeclaKaio::new("M+").com_ação(Ação::Ctrl(Controle::MemóriaSoma));
    let mut memoria_subtração =
        TeclaKaio::new("M-").com_ação(Ação::Ctrl(Controle::MemóriaSubtração));
    let mut separador_decimal =
        TeclaKaio::new("•").com_ação(Ação::Ctrl(Controle::SeparadorDecimal));

    desliga.defina_teclado(teclado.clone());
    igual.defina_teclado(teclado.clone());
    limpa_erro.defina_teclado(teclado.clone());
    memoria_leitura_escrita.defina_teclado(teclado.clone());
    memoria_soma.defina_teclado(teclado.clone());
    memoria_subtração.defina_teclado(teclado.clone());
    separador_decimal.defina_teclado(teclado.clone());
    // endregion: --- Controle

    // region: --- Sinais
    let mut positivo = TeclaKaio::new("+x").com_ação(Ação::Si(Sinal::Positivo));
    let mut negativo = TeclaKaio::new("-x").com_ação(Ação::Si(Sinal::Negativo));

    positivo.defina_teclado(teclado.clone());
    negativo.defina_teclado(teclado.clone());
    // endregion: --- Sinais

    // region: --- Operações
    let mut divisão = TeclaKaio::new("/").com_ação(Ação::Op(Operação::Divisão));
    let mut multiplicação = TeclaKaio::new("*").com_ação(Ação::Op(Operação::Multiplicação));
    let mut porcentagem = TeclaKaio::new("%").com_ação(Ação::Op(Operação::Porcentagem));
    let mut radiciação = TeclaKaio::new("^").com_ação(Ação::Op(Operação::Radiciação));
    let mut soma = TeclaKaio::new("+").com_ação(Ação::Op(Operação::Soma));
    let mut subtração = TeclaKaio::new("-").com_ação(Ação::Op(Operação::Subtração));

    divisão.defina_teclado(teclado.clone());
    multiplicação.defina_teclado(teclado.clone());
    porcentagem.defina_teclado(teclado.clone());
    radiciação.defina_teclado(teclado.clone());
    soma.defina_teclado(teclado.clone());
    subtração.defina_teclado(teclado.clone());
    // endregion: --- Operações

    let mut teclado_lock = teclado.unwrap_write();
    teclado_lock.adicione_tecla(Box::new(zero));
    teclado_lock.adicione_tecla(Box::new(um));
    teclado_lock.adicione_tecla(Box::new(dois));
    teclado_lock.adicione_tecla(Box::new(três));
    teclado_lock.adicione_tecla(Box::new(quatro));
    teclado_lock.adicione_tecla(Box::new(cinco));
    teclado_lock.adicione_tecla(Box::new(seis));
    teclado_lock.adicione_tecla(Box::new(sete));
    teclado_lock.adicione_tecla(Box::new(oito));
    teclado_lock.adicione_tecla(Box::new(nove));

    teclado_lock.adicione_tecla(Box::new(desliga));
    teclado_lock.adicione_tecla(Box::new(igual));
    teclado_lock.adicione_tecla(Box::new(limpa_erro));
    teclado_lock.adicione_tecla(Box::new(memoria_leitura_escrita));
    teclado_lock.adicione_tecla(Box::new(memoria_soma));
    teclado_lock.adicione_tecla(Box::new(memoria_subtração));
    teclado_lock.adicione_tecla(Box::new(separador_decimal));

    teclado_lock.adicione_tecla(Box::new(divisão));
    teclado_lock.adicione_tecla(Box::new(multiplicação));
    teclado_lock.adicione_tecla(Box::new(porcentagem));
    teclado_lock.adicione_tecla(Box::new(radiciação));
    teclado_lock.adicione_tecla(Box::new(soma));
    teclado_lock.adicione_tecla(Box::new(subtração));

    teclado_lock.adicione_tecla(Box::new(positivo));
    teclado_lock.adicione_tecla(Box::new(negativo));
}
