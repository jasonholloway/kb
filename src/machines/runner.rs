use std::collections::VecDeque;

use super::{LookupFac, MachineRef, Runnable, Sink};

pub struct Runner<TEv, TLookup>
where
    TLookup: LookupFac<MachineRef<TEv>>,
{
    active: Vec<MachineRef<TEv>>,
    lookup: TLookup,
    buff1: VecDeque<TEv>,
    buff2: VecDeque<TEv>,
}

impl<'a, TEv, TLookup> Runner<TEv, TLookup>
where
    TLookup: LookupFac<MachineRef<TEv>>,
{
    pub fn new<TTags: IntoIterator<Item = &'static str>>(
        lookup: TLookup,
        initial: TTags,
    ) -> Runner<TEv, TLookup> {
        Runner {
            active: initial
                .into_iter()
                .flat_map(|s| lookup.find(s))
                .collect::<Vec<_>>(),
            lookup,
            buff1: VecDeque::new(),
            buff2: VecDeque::new(),
        }
    }
}

impl<TEv, TLookup> Runnable<TEv> for Runner<TEv, TLookup>
where
    TEv: std::fmt::Debug,
    TLookup: LookupFac<MachineRef<TEv>>,
{
    fn run(&mut self, ev: TEv, sink: &mut Sink<TEv>) -> () {
        let mut input = &mut self.buff1;
        let mut output = &mut self.buff2;

        input.push_back(ev);

        for m in self.active.iter_mut() {
            for e in input.drain(0..) {
                m.run(e, output);
            }

            input.extend(output.drain(0..));
        }

        sink.extend(input.drain(0..));
    }
}
