pub struct Ape {
    // Talk about public members and public functions and how we'll change that
    // We'll encapsulate everything later
    banana_counter: u32,
    happy: bool,
}

impl Ape {
    pub fn population() -> u32 {
        36 // sad :(
    }

    pub fn new() -> Ape {
        Ape {
            banana_counter: 0,
            happy: false,
        }
    }

    pub fn with_bananas(banana_counter: u32) -> Ape {
        Ape {
            // Tell them that if your field and variable have the same name, you don't
            // need to use the <field>: <variable> syntax
            banana_counter,
            happy: match banana_counter {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn get_bananas(&self) -> u32 {
        // Now, if you change the internal working of the Ape structure, your API will
        // still stay the same
        self.banana_counter
    }

    pub fn is_happy(&self) -> bool {
        self.happy
    }

    pub fn scream(&self) {
        match self.happy {
            false => println!("OUHA OUHA OUHAAAA HAOUAH"),
            true => println!("Ouh ouh ouh :)"),
        }
    }

    pub fn eat_banana(&mut self) {
        match self.banana_counter {
            0 => self.happy = false,
            _ => self.banana_counter -= 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eat_banana_decreases_banana_counter() {
        let mut ape = Ape::with_bananas(15);
        ape.eat_banana();

        assert_eq!(ape.get_bananas(), 14);
    }

    #[test]
    fn zero_banana_means_ape_unhappy() {
        let mut ape = Ape::with_bananas(1);
        ape.eat_banana();
        ape.eat_banana();

        assert_eq!(ape.is_happy(), false);
    }
}
