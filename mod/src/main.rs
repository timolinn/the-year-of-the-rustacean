mod country {
    pub mod state {
        pub mod town {
            pub fn population() -> () {
                let pop = super::lagos_population();
                println!("The population of Lagos is = {}", pop);
            }
        }

        fn lagos_population() -> u64 {
            20_000_000
        }
    }
}

mod cities {
    pub struct City {
        id: &'static str,
        pub name: String
    }

    impl City {
        pub fn get_city_id(&self) -> &str {
            self.id
        }

        pub fn new(id: &'static str, name: String) -> City {
            City {
                id,
                name
            }
        }
    }
}

mod menu {
    #[derive(Debug)]
    pub enum FavSoup {
        Egusi,
        Efo,
    }
}

use menu::FavSoup;
use self::country::state;

fn main() {
    // using absolute path
    crate::country::state::town::population();

    // using relative path
    country::state::town::population();
    // state::town::population(); // works too

    let onitsha = cities::City::new("046", String::from("Onitsha"));
    println!("The area code of the wonderful city of {} is {}", onitsha.name, onitsha.get_city_id());

    let order1 = FavSoup::Egusi;
    let order2 = FavSoup::Efo;
    println!("Bia madam! tinyere m one plate of eba na ofe {:?}, mixye ya na {:?}", order1, order2); // a lil bit of native langs for ordering food 😀
}
