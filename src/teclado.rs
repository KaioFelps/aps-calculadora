use std::{rc::Rc, sync::RwLock};

use crate::traits::{DynamicMutable, IntoRcMutex, Recebedor, Tecla, Teclado};

pub struct TecladoKaio {
    recebedor: Option<DynamicMutable<Box<dyn Recebedor>>>,
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

    fn defina_recebedor(&mut self, recebedor: DynamicMutable<Box<dyn Recebedor>>) {
        self.recebedor = Some(recebedor);
    }

    fn obtenha_recebedor(&self) -> Option<DynamicMutable<Box<dyn Recebedor>>> {
        self.recebedor.clone()
    }
}

impl IntoRcMutex<Box<dyn Teclado>> for TecladoKaio {
    fn into_rc_mutex(self) -> DynamicMutable<Box<dyn Teclado>> {
        Rc::new(RwLock::new(Box::new(self)))
    }
}
