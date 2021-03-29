use std::collections::VecDeque;



pub trait Sink<TEv> {
    fn emit(&mut self, ev: TEv) -> ();
    fn emit_many<TIter: Iterator<Item=TEv>>(&mut self, evs: TIter) -> ();
}

impl<TEv> Sink<TEv> for VecDeque<TEv> {
    fn emit(&mut self, ev: TEv) -> () {
        self.push_back(ev);
    }

    fn emit_many<TIter: Iterator<Item=TEv>>(&mut self, evs: TIter) -> () {
        for ev in evs {
            self.emit(ev);
        }
    }
}
