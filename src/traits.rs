use std::{
    rc::Rc,
    sync::{RwLock, RwLockWriteGuard},
};

use crate::enums::{Ação, Controle, Digito, Operação, Sinal};

pub trait Calculadora {
    fn defina_tela(&mut self, tela: DynamicMutable<Box<dyn Tela>>);
    fn obtenha_tela(&self) -> &Option<DynamicMutable<Box<dyn Tela>>>;

    fn defina_ucp(&mut self, ucp: DynamicMutable<Box<dyn UcpRecebedor>>);
    fn obtenha_ucp(&self) -> &Option<DynamicMutable<Box<dyn UcpRecebedor>>>;

    fn defina_teclado(&mut self, teclado: DynamicMutable<Box<dyn Teclado>>);
    fn obtenha_teclado(&self) -> &Option<DynamicMutable<Box<dyn Teclado>>>;
}

pub trait Tela {
    fn adicione(&mut self, digito: Digito);
    fn limpe(&mut self);
    fn defina_sinal(&mut self, sinal: Sinal);
    fn defina_separador_decimal(&mut self);
}

pub trait Ucp {
    fn defina_tela(&mut self, tela: DynamicMutable<Box<dyn Tela>>);
    fn obtenha_tela(&self) -> &Option<DynamicMutable<Box<dyn Tela>>>;
    fn armazene_digito(&mut self, digito: Digito);
    fn calcule(&mut self);
}

pub trait Tecla {
    fn é(&self, chave: &str) -> bool;
    fn pressione(&self);
    fn defina_teclado(&mut self, teclado: DynamicMutable<Box<dyn Teclado>>);
    fn obtenha_teclado(&self) -> &Option<DynamicMutable<Box<dyn Teclado>>>;
    fn defina_ação(&mut self, ação: Ação);
    fn obtenha_ação(&self) -> &Ação;
}

pub trait Teclado {
    fn adicione_tecla(&mut self, tecla: Box<dyn Tecla>);
    fn defina_recebedor(&mut self, ucp: DynamicMutable<Box<dyn UcpRecebedor>>);
    fn obtenha_recebedor(&self) -> &Option<DynamicMutable<Box<dyn UcpRecebedor>>>;
    fn receba_ação(&self, ação: &Ação);
    fn procure_tecla(&self, chave: &str) -> Option<&dyn Tecla>;
}

pub trait Recebedor {
    fn receba_digito(&mut self, digito: &Digito);
    fn receba_operação(&mut self, operação: &Operação);
    fn receba_controle(&mut self, controle: &Controle);
    fn interprete_ação(&mut self, ação: &Ação);
}

pub trait UcpRecebedor: Recebedor + Ucp {}

// helper traits

pub type DynamicMutable<T> = Rc<RwLock<T>>;

pub trait IntoDynamicMutable<T> {
    fn into_dynamic_mutable(self) -> DynamicMutable<T>;
}

pub trait UnwrapDynamicMutableRef<T> {
    fn unwrap_write(&self) -> RwLockWriteGuard<'_, T>;
}

impl<T> UnwrapDynamicMutableRef<T> for DynamicMutable<T> {
    fn unwrap_write(&self) -> RwLockWriteGuard<'_, T> {
        self.write().unwrap()
    }
}

pub trait UnwrapOptionalAsRef<T> {
    fn unwrap_as_ref(&self) -> &DynamicMutable<T>;
}

impl<T> UnwrapOptionalAsRef<T> for &Option<DynamicMutable<T>> {
    fn unwrap_as_ref(&self) -> &DynamicMutable<T> {
        self.as_ref().unwrap()
    }
}

pub trait UnwrapAndWrite<T> {
    fn get_write_ref(&self) -> RwLockWriteGuard<'_, T>;
}

impl<J, T: UnwrapOptionalAsRef<J>> UnwrapAndWrite<J> for T {
    fn get_write_ref(&self) -> RwLockWriteGuard<'_, J> {
        self.unwrap_as_ref().unwrap_write()
    }
}
