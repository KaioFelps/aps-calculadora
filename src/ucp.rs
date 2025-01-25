use std::{rc::Rc, sync::RwLock};

use crate::{
    enums::{Digito, Operação},
    pilha_de_digitos::PilhaDeDigitos,
    traits::{DynamicMutable, IntoDynamicMutable, Recebedor, Tela, Ucp},
};

pub struct UcpKaio {
    tela: Option<DynamicMutable<Box<dyn Tela>>>,
    digitos_operando_1: PilhaDeDigitos,
    digitos_operando_2: PilhaDeDigitos,
    operação: Option<Operação>,
}

impl UcpKaio {
    pub fn new() -> Self {
        Self {
            tela: None,
            digitos_operando_1: PilhaDeDigitos::default(),
            digitos_operando_2: PilhaDeDigitos::default(),
            operação: None,
        }
    }
}

impl Ucp for UcpKaio {
    fn defina_tela(&mut self, tela: DynamicMutable<Box<dyn Tela>>) {
        self.tela = Some(tela)
    }

    fn obtenha_tela(&self) -> &Option<DynamicMutable<Box<dyn Tela>>> {
        &self.tela
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

impl IntoDynamicMutable<Box<dyn Ucp>> for UcpKaio {
    fn into_dynamic_mutable(self) -> DynamicMutable<Box<dyn Ucp>> {
        Rc::new(RwLock::new(Box::new(self)))
    }
}
