use libafl::inputs::UsesInput;
use libafl::observers::Observer;
use libafl_bolts::Named;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomObserver {
    name: Cow<'static, str>,
}

impl CustomObserver {
    pub fn new(name: &'static str) -> Self {
        Self {
            name: Cow::from(name),
        }
    }
}

impl<S> Observer<S> for CustomObserver
where
    S: UsesInput,
{
    fn flush(&mut self) -> Result<(), libafl_bolts::prelude::Error> {
        Ok(())
    }

    fn pre_exec(
        &mut self,
        _state: &mut S,
        _input: &<S as UsesInput>::Input,
    ) -> Result<(), libafl_bolts::prelude::Error> {
        Ok(())
    }

    fn post_exec(
        &mut self,
        _state: &mut S,
        _input: &<S as UsesInput>::Input,
        _exit_kind: &libafl::prelude::ExitKind,
    ) -> Result<(), libafl_bolts::prelude::Error> {
        Ok(())
    }

    fn pre_exec_child(
        &mut self,
        _state: &mut S,
        _input: &<S as UsesInput>::Input,
    ) -> Result<(), libafl_bolts::prelude::Error> {
        Ok(())
    }

    fn post_exec_child(
        &mut self,
        _state: &mut S,
        _input: &<S as UsesInput>::Input,
        _exit_kind: &libafl::prelude::ExitKind,
    ) -> Result<(), libafl_bolts::prelude::Error> {
        Ok(())
    }

    fn observes_stdout(&self) -> bool {
        false
    }

    fn observes_stderr(&self) -> bool {
        false
    }

    fn observe_stdout(&mut self, _stdout: &[u8]) {}

    fn observe_stderr(&mut self, _stderr: &[u8]) {}
}

impl Named for CustomObserver {
    fn name(&self) -> &Cow<'static, str> {
        &self.name
    }
}
