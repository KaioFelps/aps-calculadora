use std::{rc::Rc, sync::RwLock};

use crate::enums::{Controle, Digito, Operação, Sinal};

pub trait Calculadora {
    fn defina_tela(&mut self, tela: DynamicMutable<Box<dyn Tela>>);
    fn obtenha_tela(&self) -> Option<DynamicMutable<Box<dyn Tela>>>;

    fn defina_ucp(&mut self, ucp: DynamicMutable<Box<dyn Ucp>>);
    fn obtenha_ucp(&self) -> Option<DynamicMutable<Box<dyn Ucp>>>;

    fn defina_teclado(&mut self, teclado: DynamicMutable<Box<dyn Teclado>>);
    fn obtenha_teclado(&self) -> Option<DynamicMutable<Box<dyn Teclado>>>;
}

pub trait Tela {
    fn adicione(&mut self, digito: Digito);
    fn limpe(&mut self);
    fn defina_sinal(&mut self, sinal: Sinal);
    fn defina_separador_decimal(&mut self);
}

pub trait Ucp {
    fn defina_tela(&mut self, tela: DynamicMutable<Box<dyn Tela>>);
    fn obtenha_tela(&self) -> Option<DynamicMutable<Box<dyn Tela>>>;
    fn armazene_digito(&mut self, digito: Digito);
}

pub trait Tecla {
    fn pressione(&self);
    fn defina_teclado(&mut self, teclado: DynamicMutable<Box<dyn Teclado>>);
    fn obtenha_teclado(&self) -> Option<DynamicMutable<Box<dyn Teclado>>>;
}

pub trait Teclado {
    fn adicione_tecla(&mut self, tecla: Box<dyn Tecla>);
    fn defina_recebedor(&mut self, ucp: DynamicMutable<Box<dyn Recebedor>>);
    fn obtenha_recebedor(&self) -> Option<DynamicMutable<Box<dyn Recebedor>>>;
}

pub type DynamicMutable<T> = Rc<RwLock<T>>;

pub trait IntoDynamicMutable<T> {
    fn into_dynamic_mutable(self) -> DynamicMutable<T>;
}

pub trait Recebedor {
    fn receba_digito(&mut self, digito: Digito);
    fn receba_operação(&mut self, operação: Operação);
    fn receba_controle(&mut self, sinal: Controle);
}
