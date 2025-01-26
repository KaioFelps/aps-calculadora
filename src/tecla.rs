use crate::enums::Ação;
use crate::traits::{DynamicMutable, IntoDynamicMutable, Tecla, Teclado, UnwrapDynamicMutableRef};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};
use std::{rc::Rc, sync::RwLock};

pub struct TeclaKaio {
    teclado: Option<DynamicMutable<Box<dyn Teclado>>>,
    ação: Option<Ação>,
    chave: String,
}

impl TeclaKaio {
    pub fn new(chave: &str) -> Self {
        Self {
            teclado: None,
            ação: None,
            chave: chave.to_string(),
        }
    }

    pub fn com_ação(mut self, ação: Ação) -> Self {
        self.ação = Some(ação);
        self
    }
}

impl Tecla for TeclaKaio {
    fn pressione(&self) {
        let ação = self.ação
            .as_ref()
            .expect("Tecla sem ação acionada. Adicione uma ação ou remova a tecla para que o programa funcione corretamente.");

        self.teclado
            .as_ref()
            .unwrap()
            .read()
            .unwrap()
            .receba_ação(ação);
    }

    fn defina_teclado(&mut self, teclado: DynamicMutable<Box<dyn Teclado>>) {
        self.teclado = Some(teclado);
    }

    fn obtenha_teclado(&self) -> &Option<DynamicMutable<Box<dyn Teclado>>> {
        &self.teclado
    }

    fn defina_ação(&mut self, ação: Ação) {
        self.ação = Some(ação);
    }

    fn obtenha_ação(&self) -> &Ação {
        self.ação.as_ref().expect(
            "Tecla sem ação acionada. Adicione uma ação ou remova a tecla para que o programa funcione corretamente.",
        )
    }

    fn é(&self, chave: &str) -> bool {
        self.chave.eq(chave)
    }
}

impl IntoDynamicMutable<Box<dyn Tecla>> for TeclaKaio {
    fn into_dynamic_mutable(self) -> DynamicMutable<Box<dyn Tecla>> {
        Rc::new(RwLock::new(Box::new(self)))
    }
}
