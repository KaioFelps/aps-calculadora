use std::{rc::Rc, sync::Mutex};

use crate::traits::{IntoRcMutex, Tecla, Teclado, Ucp};

pub struct TecladoKaio {
    ucp: Option<Rc<Mutex<dyn Ucp>>>,
    teclas: Vec<Box<dyn Tecla>>,
}

impl TecladoKaio {
    pub fn new() -> Self {
        Self {
            ucp: None,
            teclas: Vec::new(),
        }
    }
}

impl Teclado for TecladoKaio {
    fn adicione_tecla(&mut self, tecla: Box<dyn Tecla>) {
        self.teclas.push(tecla);
    }

    fn defina_ucp(&mut self, ucp: Rc<Mutex<dyn Ucp>>) {
        self.ucp = Some(ucp);
    }

    fn obtenha_ucp(&self) -> Option<Rc<Mutex<dyn Ucp>>> {
        self.ucp.clone()
    }
}

impl IntoRcMutex<TecladoKaio> for TecladoKaio {
    fn into_rc_mutex(self) -> Rc<Mutex<TecladoKaio>> {
        Rc::new(Mutex::new(self))
    }
}
