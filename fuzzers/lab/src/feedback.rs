use std::borrow::Cow;

use libafl::inputs::HasBytesVec;
use libafl::prelude::Feedback;
use libafl::prelude::UsesInput;
use libafl::state::State;
use libafl_bolts::Named;

pub struct CustomFeedback {
    name: Cow<'static, str>,
}

impl CustomFeedback {
    pub fn new(name: &'static str) -> Self {
        Self {
            name: std::borrow::Cow::Borrowed(name),
        }
    }
}

impl Named for CustomFeedback {
    fn name(&self) -> &Cow<'static, str> {
        &self.name
    }
}

impl<S> Feedback<S> for CustomFeedback
where
    S: UsesInput + State,
    S::Input: HasBytesVec,
{
    fn init_state(&mut self, _state: &mut S) -> Result<(), libafl_bolts::prelude::Error> {
        Ok(())
    }

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
        Ok(true)
    }

    fn append_metadata<EM, OT>(
        &mut self,
        _state: &mut S,
        _manager: &mut EM,
        _observers: &OT,
        _testcase: &mut libafl::prelude::Testcase<<S>::Input>,
    ) -> Result<(), libafl_bolts::prelude::Error>
    where
        OT: libafl::prelude::ObserversTuple<S>,
        EM: libafl::prelude::EventFirer<State = S>,
    {
        Ok(())
    }

    fn discard_metadata(
        &mut self,
        _state: &mut S,
        _input: &<S>::Input,
    ) -> Result<(), libafl_bolts::prelude::Error> {
        Ok(())
    }
}
