use std::rc::Rc;
use std::cell::RefCell;

trait Command: CommandClone {
    fn execute(&mut self);
    fn undo(&mut self);
    fn redo(&mut self);
}

trait CommandClone {
    fn clone_box(&self) -> Box<dyn Command>;
}

impl<T> CommandClone for T
where T: 'static + Command + Clone {
    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Command> {
    fn clone(&self) -> Box<dyn Command> {
        self.clone_box()
    }
}

#[derive(Clone)]
struct Add {
    augent: Rc<RefCell<i32>>, 
    addend: Rc<RefCell<i32>>,
    addend_copy: Option<i32>,
}

impl Command for Add {
    fn execute(&mut self) {
        let augent = *self.augent.borrow();
        let addend = *self.addend.borrow();
        let sum = augent + addend;
        println!("executing addition: {} + {} = {}", augent, addend, sum);
        *self.augent.borrow_mut() = sum;
        self.addend_copy = Some(addend);
    }

    fn undo(&mut self) {
        let subtrahend = *self.augent.borrow();
        let minuend = self.addend_copy.expect("Undo should not be called before execute");
        let difference = subtrahend - minuend;
        println!("Undoing addition: {} - {} = {}", subtrahend, minuend, difference);
        *self.augent.borrow_mut() = difference;
    }

    fn redo(&mut self) {
        let augent = *self.augent.borrow();
        let addend = self.addend_copy.expect("Redo should not be called before execute");
        let sum = augent + addend;
        println!("redoing addition: {} + {} = {}", augent, addend, sum);
        *self.augent.borrow_mut() = sum;
    }
}

#[derive(Clone)]
struct DollarGiver;

impl Command for DollarGiver {
    fn execute(&mut self) {
        println!("*Gives you a dollar*");
    }
    fn undo(&mut self) {
        println!("*Takes your dollar back :(*");
    }
    fn redo(&mut self) {
        println!("*Gives you a dollar again*");
    }
}

trait Button {
    fn click(&mut self);
    fn unclick(&mut self);
    fn reclick(&mut self);
}

struct SimpleButton {
    command: Box<dyn Command>,
    prev: Vec<Box<dyn Command>>,
    next: Vec<Box<dyn Command>>,
}

impl Button for SimpleButton {
    fn click(&mut self) {
        println!("*click*");
        let mut command = self.command.clone();
        command.execute();
        self.prev.push(command);
    }

    fn unclick(&mut self) {
        println!("*kcilc*");
        let mut command = self.prev.pop().expect("You shouldn't unclick before clicking!");
        command.undo();
        self.next.push(command);
    }

    fn reclick(&mut self) {
        println!("*click?*");
        let mut command = self.next.pop().expect("You shouldn't reclick before unclicking!");
        command.redo();
        self.prev.push(command);
    }
}

fn main() {
    let x = Rc::new(RefCell::new(2));
    let y = Rc::new(RefCell::new(3));
    let mut addition = Add {
        augent: Rc::clone(&x),
        addend: Rc::clone(&y),
        addend_copy: None,
    };
    addition.execute();
    addition.undo();
    addition.redo();

    let mut dollar_giver = DollarGiver;
    dollar_giver.execute();
    dollar_giver.undo();
    dollar_giver.redo();

    let mut button = SimpleButton {
        command: Box::new(addition),
        prev: Vec::new(),
        next: Vec::new(),
    };

    button.click();
    *y.borrow_mut() = 4;
    button.click();
    *y.borrow_mut() = 1;
    button.click();
    button.unclick();
    button.unclick();
    button.reclick();
}
