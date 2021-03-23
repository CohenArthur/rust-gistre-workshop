use crate::animal::Ape;

pub struct ApeWrangler;

impl ApeWrangler {
    fn feed(&self, ape: &mut Ape) {
        ape.set_bananas(ape.get_bananas() + 2)
    }

    fn make_happy(&self, ape: &mut Ape) {
        match ape.is_happy() {
            false => ape.set_happiness(true),
            true => {}
        }
    }

    pub fn take_care(&self, apes: &mut [&mut Ape]) {
        apes.iter_mut().for_each(|ape| {
            self.feed(ape);
            self.make_happy(ape)
        })
    }
}
