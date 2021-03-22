pub struct Ape {
    // Talk about public members and public functions and how we'll change that
    // We'll encapsulate everything later
    pub banana_counter: u32,
    pub happy: bool,
}

impl Ape {
    pub fn population() -> u32 {
        36 // sad :(
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
        let mut ape = Ape {
            banana_counter: 15,
            happy: true,
        };
        ape.eat_banana();

        assert_eq!(ape.banana_counter, 14);
    }

    #[test]
    fn zero_banana_means_ape_unhappy() {
        let mut ape = Ape {
            banana_counter: 1,
            happy: true,
        };
        ape.eat_banana();
        ape.eat_banana();

        assert_eq!(ape.happy, false);
    }
}
