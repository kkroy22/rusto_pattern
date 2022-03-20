use std::marker::PhantomData;

trait Drinks {
    fn serve(&self) -> DrinkNature;
}

enum DrinkNature {
    Dairy,
    Mocktail,
    Cocktail,
    Fruity,
}

struct MangoJuice;

impl Drinks for MangoJuice {
    fn serve(&self)  -> DrinkNature {
        println!("mango juice in tumbler with ice cream");
        DrinkNature::Fruity
    }
}

pub struct Coffee;

impl Drinks for Coffee {
    fn serve(&self) -> DrinkNature {
        println!("coffee latte in white cup");
        DrinkNature::Dairy
    }
}

struct DrinkFactory<T> {
    data: PhantomData<T>
}

impl<T> DrinkFactory<T>
where T: Drinks
{
    fn order_drink(d: T) -> T {
        d
    }
}
