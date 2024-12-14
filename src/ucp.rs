use std::{rc::Rc, sync::Mutex};

use crate::traits::{IntoRcMutex, Tela, Ucp};

pub struct UcpKaio {
    tela: Option<Rc<Mutex<dyn Tela>>>,
}

impl UcpKaio {
    pub fn new() -> Self {
        Self { tela: None }
    }
}

impl Ucp for UcpKaio {
    fn defina_tela(&mut self, tela: Rc<Mutex<dyn Tela>>) {
        self.tela = Some(tela)
    }

    fn obtenha_tela(&self) -> Option<Rc<Mutex<dyn Tela>>> {
        self.tela.clone()
    }
}

impl IntoRcMutex<UcpKaio> for UcpKaio {
    fn into_rc_mutex(self) -> Rc<Mutex<UcpKaio>> {
        Rc::new(Mutex::new(self))
    }
}
