#[derive(Debug,PartialEq)]
pub enum CoreEv {
    Key(u16, Movement),
    On(Mode),
    Off(Mode),
    Tick
}

#[derive(Debug,PartialEq)]
pub enum MachineEv {
    Now(Mode),
    MaskOn(u16),
    MaskOff(u16),
}

#[derive(Debug,PartialEq)]
pub enum RunnerEv {
    Spawn(String),
    Die
}


#[derive(Debug,PartialEq)]
pub enum Out {
    Core(CoreEv),
    Machine(MachineEv),
    Runner(RunnerEv)
}

#[derive(Debug,PartialEq)]
pub enum MachineOut {
    Core(CoreEv),
    Runner(RunnerEv)
}

#[derive(Debug,PartialEq)]
pub enum RunnerOut {
    Core(CoreEv),
    Runner(RunnerEv)
}



#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Movement {
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    Root,
    Mode(&'static str),
}

#[derive(Debug, Copy, Clone)]
pub enum Act {
    Drop,
    Mask(u16),
    Map(u16, u16),
    Emit(u16, Movement),
    Then(Mode),
    Launch(&'static str),
}

pub type NextDue = u64;
