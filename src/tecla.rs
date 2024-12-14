use std::{rc::Rc, sync::Mutex};

use crate::traits::{IntoRcMutex, Tecla, Teclado};

pub struct TeclaKaio {
    teclado: Option<Rc<Mutex<dyn Teclado>>>,
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

    fn defina_teclado(&mut self, teclado: Rc<Mutex<dyn Teclado>>) {
        self.teclado = Some(teclado);
    }

    fn obtenha_teclado(&self) -> Option<Rc<Mutex<dyn Teclado>>> {
        self.teclado.clone()
    }
}

impl IntoRcMutex<TeclaKaio> for TeclaKaio {
    fn into_rc_mutex(self) -> Rc<Mutex<TeclaKaio>> {
        Rc::new(Mutex::new(self))
    }
}
