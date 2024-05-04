pub mod feedback;
pub mod forkserver_executor;
pub mod map_feedback;
pub mod map_observer;
pub mod max_map_feedback;
pub mod monitor;
pub mod observer;
pub mod std_map_observer;

use forkserver_executor::CustomForkserverExecutor;
use libafl::{
    corpus::{InMemoryCorpus, OnDiskCorpus},
    events::SimpleEventManager,
    executors::ForkserverExecutor,
    feedbacks::CrashFeedback,
    inputs::BytesInput,
    monitors::SimpleMonitor,
    mutators::{havoc_mutations, StdScheduledMutator},
    schedulers::QueueScheduler,
    stages::StdMutationalStage,
    state::StdState,
    Fuzzer, StdFuzzer,
};
use libafl_bolts::{
    rands::StdRand,
    shmem::{ShMem, ShMemProvider, StdShMemProvider},
    tuples::tuple_list,
    AsSliceMut,
};
use max_map_feedback::CustomMaxMapFeedback;
use std::path::PathBuf;

use std_map_observer::CustomStdMapObserver;

static MAP_SIZE: usize = 512;

fn main() {
    let mut shmem_provider = StdShMemProvider::new().unwrap();
    let mut shmem = shmem_provider.new_shmem(MAP_SIZE).unwrap();
    shmem.write_to_env("__AFL_SHM_ID").unwrap();
    let shmem_map = shmem.as_slice_mut();

    let observer = unsafe { CustomStdMapObserver::new("shared_mem", shmem_map) };
    let mut feedback = CustomMaxMapFeedback::new(&observer);

    let monitor = SimpleMonitor::new(|f| println!("{f}"));
    let mut objective = CrashFeedback::new();

    let mut state = StdState::new(
        StdRand::new(),
        InMemoryCorpus::<BytesInput>::new(),
        OnDiskCorpus::new(PathBuf::from("./crashes")).unwrap(),
        &mut feedback,
        &mut objective,
    )
    .unwrap();

    let mut executor = CustomForkserverExecutor::builder()
        .shmem_provider(&mut shmem_provider)
        .arg_input_file_std()
        .coverage_map_size(MAP_SIZE)
        .program("./lab")
        .debug_child(true)
        .is_deferred_frksrv(true)
        .build(tuple_list!(observer))
        .unwrap();

    let mut manager = SimpleEventManager::new(monitor);
    let mut stages = tuple_list!(StdMutationalStage::new(StdScheduledMutator::new(
        havoc_mutations()
    )));

    let scheduler = QueueScheduler::new();
    let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);

    state
        .load_initial_inputs(
            &mut fuzzer,
            &mut executor,
            &mut manager,
            &[PathBuf::from("./input")],
        )
        .unwrap();

    fuzzer
        .fuzz_loop(&mut stages, &mut executor, &mut state, &mut manager)
        .unwrap();
}
