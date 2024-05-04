use libafl::{inputs::UsesInput, prelude::Feedback, state::State};
use libafl_bolts::Named;
use std::borrow::Cow;

pub struct CustomMapFeedback {
    name: Cow<'static, str>,
}

impl<S> Feedback<S> for CustomMapFeedback
where
    S: State + UsesInput,
{
    fn is_interesting<EM, OT>(
        &mut self,
        _state: &mut S,
        _manager: &mut EM,
        _input: &<S>::Input,
        _observers: &OT,
        _exit_kind: &libafl::prelude::ExitKind,
    ) -> Result<bool, libafl_bolts::prelude::Error>
    where
        EM: libafl::prelude::EventFirer<State = S>,
        OT: libafl::prelude::ObserversTuple<S>,
    {
        todo!()
    }
}

impl Named for CustomMapFeedback {
    fn name(&self) -> &Cow<'static, str> {
        &self.name
    }
}
