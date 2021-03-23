pub enum Class {
    RGW,
    DRIL,
    BSP,
    MAUTO,
}

pub struct Gistre {
    motivation: i32,
    happy: bool,
}

impl Gistre {
    pub fn sad() -> Gistre {
        Gistre {
            motivation: -1,
            happy: false,
        }
    }

    pub fn happy() -> Gistre {
        Gistre {
            motivation: 2,
            happy: true,
        }
    }

    pub fn attend_class(&mut self, class: Class) {
        match class {
            Class::RGW | Class::DRIL => {
                self.motivation += 1;
                self.happy = true
            }
            Class::BSP => {}
            Class::MAUTO => self.motivation -= 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgw_makes_people_happy() {
        let mut ape = Gistre::happy();

        ape.attend_class(Class::RGW);

        assert!(ape.happy);
        assert_eq!(ape.motivation, 3);
    }
}
