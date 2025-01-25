use std::{rc::Rc, sync::RwLock};

use crate::{
    enums::{Digito, Operação},
    traits::{DynamicMutable, IntoRcMutex, Recebedor, Tela, Ucp},
};

const MAXIMO_DIGITOS_POR_OPERANDO: usize = 8;

#[derive(Default)]
pub struct PilhaDeOperandos {
    memoria: [Option<Digito>; MAXIMO_DIGITOS_POR_OPERANDO],
    topo: u8,
}

impl PilhaDeOperandos {
    pub fn receba(&mut self, digito: Digito) {
        if self.topo > MAXIMO_DIGITOS_POR_OPERANDO as u8 {
            self.memoria[self.topo as usize] = Some(digito);
            self.topo.saturating_add(1);
        }
    }
}

pub struct UcpKaio {
    tela: Option<DynamicMutable<Box<dyn Tela>>>,
    digitos_operando_1: PilhaDeOperandos,
    digitos_operando_2: PilhaDeOperandos,
    operação: Option<Operação>,
}

impl UcpKaio {
    pub fn new() -> Self {
        Self {
            tela: None,
            digitos_operando_1: PilhaDeOperandos::default(),
            digitos_operando_2: PilhaDeOperandos::default(),
            operação: None,
        }
    }
}

impl Ucp for UcpKaio {
    fn defina_tela(&mut self, tela: DynamicMutable<Box<dyn Tela>>) {
        self.tela = Some(tela)
    }

    fn obtenha_tela(&self) -> Option<DynamicMutable<Box<dyn Tela>>> {
        self.tela.clone()
    }

    fn armazene_digito(&mut self, digito: Digito) {
        match self.operação {
            Some(_) => self.digitos_operando_2.receba(digito),
            None => self.digitos_operando_1.receba(digito),
        };
    }
}

impl Recebedor for UcpKaio {
    fn receba_digito(&mut self, digito: crate::enums::Digito) {
        self.tela
            .as_ref()
            .unwrap()
            .write()
            .unwrap()
            .adicione(digito);
    }

    fn receba_operação(&mut self, operação: crate::enums::Operação) {
        todo!()
    }

    fn receba_controle(&mut self, sinal: crate::enums::Controle) {
        todo!()
    }
}

impl IntoRcMutex<Box<dyn Ucp>> for UcpKaio {
    fn into_rc_mutex(self) -> DynamicMutable<Box<dyn Ucp>> {
        Rc::new(RwLock::new(Box::new(self)))
    }
}
