use std::{rc::Rc, sync::RwLock};

use crate::traits::{Calculadora, DynamicMutable, IntoDynamicMutable, Teclado, Tela, UcpRecebedor};

pub struct CalculadoraKaio {
    teclado: Option<DynamicMutable<Box<dyn Teclado>>>,
    ucp: Option<DynamicMutable<Box<dyn UcpRecebedor>>>,
    tela: Option<DynamicMutable<Box<dyn Tela>>>,
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
    fn defina_tela(&mut self, tela: DynamicMutable<Box<dyn Tela>>) {
        self.tela = Some(tela);
    }

    fn obtenha_tela(&self) -> &Option<DynamicMutable<Box<dyn Tela>>> {
        &self.tela
    }

    fn defina_ucp(&mut self, ucp: DynamicMutable<Box<dyn UcpRecebedor>>) {
        self.ucp = Some(ucp);
    }

    fn obtenha_ucp(&self) -> &Option<DynamicMutable<Box<dyn UcpRecebedor>>> {
        &self.ucp
    }

    fn defina_teclado(&mut self, teclado: DynamicMutable<Box<dyn Teclado>>) {
        self.teclado = Some(teclado);
    }

    fn obtenha_teclado(&self) -> &Option<DynamicMutable<Box<dyn Teclado>>> {
        &self.teclado
    }
}

impl IntoDynamicMutable<Box<dyn Calculadora>> for CalculadoraKaio {
    fn into_dynamic_mutable(self) -> DynamicMutable<Box<dyn Calculadora>> {
        Rc::new(RwLock::new(Box::new(self)))
    }
}
