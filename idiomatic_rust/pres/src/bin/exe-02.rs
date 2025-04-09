use std::marker::PhantomData;

use rand::Rng;

pub struct Printer3D<S> {
    _marker: PhantomData<S>,
}

/* States */

/// The 3D printer encountered an error and needs resetting
pub enum ErrorState {}
/// The 3D printer is waiting for a job
pub enum IdleState {}
/// The 3D printer is currently printing
pub enum PrintingState {}
/// The 3D printed product is ready
pub enum ProductReadyState {}

impl Printer3D<ErrorState> {
    /// Reset the printer
    pub fn reset(self) -> Printer3D<IdleState> {
        println!("Resetting printer");
        Printer3D {
            _marker: PhantomData,
        }
    }
}

impl Printer3D<IdleState> {
    /// Start printing
    pub fn start(self) -> Printer3D<PrintingState> {
        println!("Starting print");
        Printer3D {
            _marker: PhantomData,
        }
    }
}

/// Check if we're out of filament
fn out_of_filament() -> bool {
    let rand: usize = rand::thread_rng().gen_range(0..100);
    rand > 95
}

impl Printer3D<PrintingState> {
    pub fn out_of_filament(self) -> Printer3D<ErrorState> {
        println!("Out of filament");
        Printer3D {
            _marker: PhantomData,
        }
    }

    pub fn ready(self) -> Printer3D<ProductReadyState> {
        println!("Printed product");
        Printer3D {
            _marker: PhantomData,
        }
    }
}

impl Printer3D<ProductReadyState> {
    pub fn retrieve(self) -> Printer3D<IdleState> {
        println!("Product retrieved");
        Printer3D {
            _marker: PhantomData,
        }
    }
}

fn main() {
    let printer = Printer3D {
        _marker: PhantomData,
    };
    let printer = printer.reset();
    let printer = printer.start();
    if out_of_filament() {
        printer.out_of_filament();
    } else {
        let printer = printer.ready();
        printer.retrieve();
    };
}
