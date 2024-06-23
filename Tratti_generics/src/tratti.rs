use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Cilindro {
    pub raggio: f64,
    pub altezza: f64,
}

pub struct Cubo {
    pub lato: f64,
}

pub trait Figura3D:{
    type AssociatedType;
    fn volume(&self) -> f64;
    fn prova(self: Box<Self>);
    fn associated(arg:Self::AssociatedType);
}

impl Figura3D for Cilindro {
    type AssociatedType = f64;
    fn volume(&self) -> f64 {
        std::f64::consts::PI * self.raggio * self.raggio * self.altezza
    }
    fn prova(self: Box<Self>) {
        println!("Prova");
    }
    fn associated(arg:Self::AssociatedType) {
        println!("{}", arg)
    }
}

impl Figura3D for Cubo{
    type AssociatedType = f64;
    fn volume(&self) -> f64 {
        self.lato * self.lato * self.lato
    }
    fn prova(self: Box<Self>) {
        println!("Prova");
    }
    fn associated(_: Self::AssociatedType) {
        todo!()
    }
}

impl PartialEq for Cilindro {
    fn eq(&self, other: &Self) -> bool {
        self.volume() == other.volume()
    }
}

//TRATTI GENERICI
pub struct CilindroGeneric<T> { //lo voglio o con gli interi o con i float
    pub raggio: T,
    pub altezza: T,
}

impl<T> CilindroGeneric<T> where T: Mul<Output=T> + Copy + std::fmt::Debug {
    pub fn volume(&self) -> T {
        self.raggio * self.raggio * self.altezza
    }
}


trait Figura3DGeneric{
    fn volume<T>(&self) -> T;
}
