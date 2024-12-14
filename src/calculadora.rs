use std::{rc::Rc, sync::Mutex};

use crate::traits::{Calculadora, IntoRcMutex, Teclado, Tela, Ucp};

pub struct CalculadoraKaio {
    teclado: Option<Rc<Mutex<dyn Teclado>>>,
    ucp: Option<Rc<Mutex<dyn Ucp>>>,
    tela: Option<Rc<Mutex<dyn Tela>>>,
}

impl CalculadoraKaio {
    pub fn new() -> Self {
        Self {
            teclado: None,
            tela: None,
            ucp: None,
        }
    }
}

impl Calculadora for CalculadoraKaio {
    fn defina_tela(&mut self, tela: Rc<Mutex<dyn Tela>>) {
        self.tela = Some(tela);
    }

    fn obtenha_tela(&self) -> Option<Rc<Mutex<dyn Tela>>> {
        self.tela.clone()
    }

    fn defina_ucp(&mut self, ucp: Rc<Mutex<dyn Ucp>>) {
        self.ucp = Some(ucp);
    }

    fn obtenha_ucp(&self) -> Option<Rc<Mutex<dyn Ucp>>> {
        self.ucp.clone()
    }

    fn defina_teclado(&mut self, teclado: Rc<Mutex<dyn Teclado>>) {
        self.teclado = Some(teclado);
    }

    fn obtenha_teclado(&self) -> Option<Rc<Mutex<dyn Teclado>>> {
        self.teclado.clone()
    }
}

impl IntoRcMutex<CalculadoraKaio> for CalculadoraKaio {
    fn into_rc_mutex(self) -> Rc<Mutex<CalculadoraKaio>> {
        Rc::new(Mutex::new(self))
    }
}
