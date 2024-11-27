pub enum Digito {
    Zero,
    Um,
    Dois,
    Três,
    Quatro,
    Cinco,
    Seis,
    Sete,
    Oito,
    Nove,
}

pub enum Operação {
    Soma,
    Subtração,
    Multiplicação,
    Divisão,
    Radiciação,
    Porcentagem,
}

pub enum Controle {
    LigaLimpaErro,
    Desliga,
    MemóriaLeituraEscrita,
    MemóriaSoma,
    MemóriaSubtração,
    Igual,
    SeparadorDecimal,
}

pub struct Tecla;

impl Tecla {
    pub fn pressione() {}
}
