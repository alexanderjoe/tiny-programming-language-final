use std::rc::Rc;
use crate::analyzer::Analyzer;
use crate::executor::Executor;
use crate::tree::ProgramNode;

pub struct Machine {
    rc_program: Rc<ProgramNode>,
}

impl Machine {

    pub fn new(rc_program: Rc<ProgramNode>) -> Machine {
        Machine {
            rc_program
        }
    }

    pub fn run(&self) {

        let analyzer = Analyzer::new(self.rc_program.clone());
        analyzer.analyze();

        let executor = Executor::new(self.rc_program.clone());
        executor.execute();

    }
}