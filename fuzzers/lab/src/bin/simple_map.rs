fn main() {}
// use libafl::{
//     corpus::{InMemoryCorpus, OnDiskCorpus},
//     events::SimpleEventManager,
//     executors::{CommandExecutor, ExitKind, InProcessExecutor},
//     feedbacks::CrashFeedback,
//     inputs::{BytesInput, HasTargetBytes},
//     monitors::SimpleMonitor,
//     mutators::{havoc_mutations, StdScheduledMutator},
//     schedulers::QueueScheduler,
//     stages::StdMutationalStage,
//     state::StdState,
//     Fuzzer, StdFuzzer,
// };
// use libafl_bolts::{rands::StdRand, tuples::tuple_list, AsSlice};
// use max_map_feedback::CustomMaxMapFeedback;
// use std::{path::PathBuf, ptr::write};
//
// use std::time::Duration;
// use std_map_observer::CustomStdMapObserver;
//
// // static OBS_NAME: &str = "hello";
// // static FEEDBACK_NAME: &str = "hello_feed";
//
// static mut SIGNALS: [u8; 16] = [0; 16];
// static mut SIGNALS_PTR: *mut u8 = unsafe { SIGNALS.as_mut_ptr() };
//
// fn signals_set(idx: usize) {
//     unsafe { write(SIGNALS_PTR.add(idx), 1) };
// }
//
// fn harness_fn(input: &BytesInput) -> ExitKind {
//     let target = input.target_bytes();
//     let buf = target.as_slice();
//     signals_set(0);
//     if !buf.is_empty() && buf[0] == b'a' {
//         signals_set(1);
//         if buf.len() > 1 && buf[1] == b'b' {
//             signals_set(2);
//             if buf.len() > 2 && buf[2] == b'c' {
//                 signals_set(3);
//                 if buf.len() > 2 && buf[2] == b'd' {
//                     signals_set(4);
//                     panic!("panic 1");
//                 }
//             }
//         }
//
//         if buf.len() > 1 && buf[1] == b'e' {
//             signals_set(5);
//         }
//     }
//     ExitKind::Ok
// }
//
// fn main() {
//     // let observer = unsafe { StdMapObserver::from_mut_ptr("signals", SIGNALS_PTR, SIGNALS.len()) };
//     // let mut feedback = MaxMapFeedback::new(&observer);
//
//     let observer =
//         unsafe { CustomStdMapObserver::from_mut_ptr("signals", SIGNALS_PTR, SIGNALS.len()) };
//     let mut feedback = CustomMaxMapFeedback::new(&observer);
//
//     // let observer = CustomObserver::new(OBS_NAME);
//     // let mut feedback = CustomFeedback::new(FEEDBACK_NAME);
//
//     let monitor = SimpleMonitor::new(|f| println!("{f}"));
//     let mut objective = CrashFeedback::new();
//
//     let mut state = StdState::new(
//         StdRand::new(),
//         InMemoryCorpus::<BytesInput>::new(),
//         OnDiskCorpus::new(PathBuf::from("./crashes")).unwrap(),
//         &mut feedback,
//         &mut objective,
//     )
//     .unwrap();
//
//     // let mut executor = CommandExecutor::builder()
//     //     .program("./lab")
//     //     .timeout(Duration::from_millis(1000))
//     //     .build(tuple_list!(observer))
//     //     .unwrap();
//
//     let mut manager = SimpleEventManager::new(monitor);
//     let mut stages = tuple_list!(StdMutationalStage::new(StdScheduledMutator::new(
//         havoc_mutations()
//     )));
//
//     let scheduler = QueueScheduler::new();
//     let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);
//
//     let mut harness = harness_fn;
//
//     let mut executor = InProcessExecutor::new(
//         &mut harness,
//         tuple_list!(observer),
//         &mut fuzzer,
//         &mut state,
//         &mut manager,
//     )
//     .unwrap();
//
//     state
//         .load_initial_inputs(
//             &mut fuzzer,
//             &mut executor,
//             &mut manager,
//             &[PathBuf::from("./input")],
//         )
//         .unwrap();
//
//     fuzzer
//         .fuzz_loop(&mut stages, &mut executor, &mut state, &mut manager)
//         .unwrap();
// }
