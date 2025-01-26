use std::{rc::Rc, sync::RwLock};

use crate::enums::Ação;
use crate::traits::{DynamicMutable, IntoDynamicMutable, Tecla, Teclado, UcpRecebedor};

pub struct TecladoKaio {
    recebedor: Option<DynamicMutable<Box<dyn UcpRecebedor>>>,
    teclas: Vec<Box<dyn Tecla>>,
}

impl TecladoKaio {
    pub fn new() -> Self {
        Self {
            recebedor: None,
            teclas: Vec::new(),
        }
    }
}

impl Teclado for TecladoKaio {
    fn adicione_tecla(&mut self, tecla: Box<dyn Tecla>) {
        self.teclas.push(tecla);
    }

    fn defina_recebedor(&mut self, recebedor: DynamicMutable<Box<dyn UcpRecebedor>>) {
        self.recebedor = Some(recebedor);
    }

    fn obtenha_recebedor(&self) -> &Option<DynamicMutable<Box<dyn UcpRecebedor>>> {
        &self.recebedor
    }

    fn receba_ação(&self, ação: &Ação) {
        println!("{:#?}", ação);
        self.recebedor
            .as_ref()
            .unwrap()
            .write()
            .unwrap()
            .interprete_ação(ação);
    }

    fn procure_tecla(&self, chave: &str) -> Option<&dyn Tecla> {
        self.teclas
            .iter()
            .find(|tecla| (*tecla).é(chave))
            .map(|tecla| &(**tecla))
    }
}

impl IntoDynamicMutable<Box<dyn Teclado>> for TecladoKaio {
    fn into_dynamic_mutable(self) -> DynamicMutable<Box<dyn Teclado>> {
        Rc::new(RwLock::new(Box::new(self)))
    }
}
