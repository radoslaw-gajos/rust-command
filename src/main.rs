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
    addend: i32,
}

impl Command for Add {
    fn execute(&mut self) {
        let augent = *self.augent.borrow();
        let addend = self.addend;
        let sum = augent + addend;
        println!("executing addition: {} + {} = {}", augent, addend, sum);
        *self.augent.borrow_mut() = sum;
    }

    fn undo(&mut self) {
        let subtrahend = *self.augent.borrow();
        let minuend = self.addend;
        let difference = subtrahend - minuend;
        println!("Undoing addition: {} - {} = {}", subtrahend, minuend, difference);
        *self.augent.borrow_mut() = difference;
    }

    fn redo(&mut self) {
        let augent = *self.augent.borrow();
        let addend = self.addend;
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
}

struct SimpleButton {
    command: Box<dyn Command>,
}

impl Button for SimpleButton {
    fn click(&mut self) {
        println!("*click*");
        self.command.execute();
    }
}

fn main() {
    let x = Rc::new(RefCell::new(2));
    let mut addition = Add {
        augent: x,
        addend: 3,
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
    };

    button.click();

    // Todo: history of commands + changeable addend
}
