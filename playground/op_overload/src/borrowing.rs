use std::borrow::{Borrow, BorrowMut};

fn main() {
    let mut name = Name(String::from("Hello"));

    use_s(name.borrow());
    use_s_mut(name.borrow_mut());

    use_s_generic(&name);
    use_s_generic_mut(&mut name);

    use_s_generic_as_ref(&name);
    use_s_generic_as_mut(&mut name);
}

fn use_s_generic_as_ref(s: &impl AsRef<String>) {}

fn use_s_generic_as_mut(s: &mut impl AsMut<String>) {}

fn use_s_generic<T: Borrow<String>>(s: &T) {}

fn use_s_generic_mut<T: BorrowMut<String>>(s: &mut T) {}

fn use_s(s: &String) {}

fn use_s_mut(s: &mut String) {}
struct Name(String);

impl Borrow<String> for Name {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl BorrowMut<String> for Name {
    fn borrow_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl AsRef<String> for Name {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsMut<String> for Name {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
