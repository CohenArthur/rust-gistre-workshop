pub enum Class {
    RGW,
    DRIL,
    BSP,
    MAUTO,
}

pub struct Gistre {
    pub motivation: i32,
    pub happy: bool,
}

impl Gistre {
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
        let mut ape = Gistre {
            motivation: 0,
            happy: false,
        };

        ape.attend_class(Class::RGW);

        assert!(ape.happy);
        assert_eq!(ape.motivation, 1);
    }
}
