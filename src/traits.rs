use std::{rc::Rc, sync::Mutex};

use crate::enums::{Digito, Sinal};

pub trait Calculadora {
    fn defina_tela(&mut self, tela: Rc<Mutex<dyn Tela>>);
    fn obtenha_tela(&self) -> Option<Rc<Mutex<dyn Tela>>>;

    fn defina_ucp(&mut self, ucp: Rc<Mutex<dyn Ucp>>);
    fn obtenha_ucp(&self) -> Option<Rc<Mutex<dyn Ucp>>>;

    fn defina_teclado(&mut self, teclado: Rc<Mutex<dyn Teclado>>);
    fn obtenha_teclado(&self) -> Option<Rc<Mutex<dyn Teclado>>>;
}

pub trait Tela {
    fn adicione(&self, digito: Digito);
    fn limpe(&self);
    fn defina_sinal(&self, sinal: Sinal);
    fn defina_separador_decimal(&self);
}

pub trait Ucp {
    fn defina_tela(&mut self, tela: Rc<Mutex<dyn Tela>>);
    fn obtenha_tela(&self) -> Option<Rc<Mutex<dyn Tela>>>;
}

pub trait Tecla {
    fn pressione(&self);
    fn defina_teclado(&mut self, teclado: Rc<Mutex<dyn Teclado>>);
    fn obtenha_teclado(&self) -> Option<Rc<Mutex<dyn Teclado>>>;
}

pub trait Teclado {
    fn adicione_tecla(&mut self, tecla: Box<dyn Tecla>);
    fn defina_ucp(&mut self, ucp: Rc<Mutex<dyn Ucp>>);
    fn obtenha_ucp(&self) -> Option<Rc<Mutex<dyn Ucp>>>;
}

pub trait IntoRcMutex<T> {
    fn into_rc_mutex(self) -> Rc<Mutex<T>>;
}
