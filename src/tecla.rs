use std::{rc::Rc, sync::RwLock};

use crate::traits::{DynamicMutable, IntoRcMutex, Tecla, Teclado};

pub struct TeclaKaio {
    teclado: Option<DynamicMutable<Box<dyn Teclado>>>,
}

impl TeclaKaio {
    pub fn new() -> Self {
        Self { teclado: None }
    }
}

impl Tecla for TeclaKaio {
    fn pressione(&self) {
        todo!()
    }

    fn defina_teclado(&mut self, teclado: DynamicMutable<Box<dyn Teclado>>) {
        self.teclado = Some(teclado);
    }

    fn obtenha_teclado(&self) -> Option<DynamicMutable<Box<dyn Teclado>>> {
        self.teclado.clone()
    }
}

impl IntoRcMutex<Box<dyn Tecla>> for TeclaKaio {
    fn into_rc_mutex(self) -> DynamicMutable<Box<dyn Tecla>> {
        Rc::new(RwLock::new(Box::new(self)))
    }
}
