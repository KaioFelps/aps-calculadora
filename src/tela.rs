use crate::tecla::Digito;

pub struct Tela;

impl Tela {
    pub fn adicione(digito: Digito) {
        print!("{}", digito.to_char());
    }

    pub fn limpe() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}
