use std::{cell::RefCell, io::Write, rc::Rc};

pub type OutStream = Rc<RefCell<dyn Write>>;

pub fn write(to: OutStream, contents: &impl ToString) {
    (*to)
        .borrow_mut()
        .write_all(contents.to_string().as_bytes())
        .unwrap();
}
