//! A scheduler library.
//!
//! This library provides the traits and structures necessary
//! to implement a process scheduler.
//!

use std::collections::VecDeque;
use std::num::NonZeroUsize;

mod scheduler;

use schedulers::{PriorityQueueRoundRobin, RoundRobin, CFS};

pub use crate::scheduler::{
    Pid, Process, ProcessState, Scheduler, SchedulingDecision, StopReason, Syscall, SyscallResult,
};

mod schedulers;

// TODO import your scheduler here
// This example imports the Empty scheduler

/// Returns a structure that implements the `Scheduler` trait with a round robin scheduler policy
///
/// * `timeslice` - the time quanta that a process can run before it is preempted
/// * `minimum_remaining_timeslice` - when a process makes a system call, the scheduler
///                                 has to decode whether to schedule it again for the
///                                 remaining time of its quanta, or to schedule a new
///                                 process. The scheduler will schedule the process
///                                 again of the remaining quanta is greater or equal to
///                                 the `minimum_remaining_timeslice` value.

pub fn round_robin(timeslice: NonZeroUsize, minimum_remaining_timeslice: usize) -> impl Scheduler {
    RoundRobin {
        ready_processes: VecDeque::new(),
        waiting_processes: VecDeque::new(),
       timeslice,
       minimum_remaining_timeslice,
        remaining_timeslice: 0,
        next_pid: 1,
        sleep: false,
        sleep_time: 0,
    }
}

/// Returns a structure that implements the `Scheduler` trait with a priority queue scheduler policy
/// * `timeslice` - the time quanta that a process can run before it is preempted
/// * `minimum_remaining_timeslice` - when a process makes a system call, the scheduler
///                                 has to decode whether to schedule it again for the
///                                 remaining time of its quanta, or to schedule a new
///                                 process. The scheduler will schedule the process
///                                 again of the remaining quanta is greater or equal to
///                                 the `minimum_remaining_timeslice` value.
pub fn priority_queue(
    timeslice: NonZeroUsize,
    minimum_remaining_timeslice: usize,
) -> impl Scheduler {
    PriorityQueueRoundRobin {
        ready_processes: VecDeque::new(),
        waiting_processes: VecDeque::new(),
        timeslice,
        minimum_remaining_timeslice,
        remaining_timeslice: 0,
        next_pid: 1,
        sleep: false,
        sleep_time: 0,
    }
}

/// Returns a structure that implements the `Scheduler` trait with a simplified [cfs](https://opensource.com/article/19/2/fair-scheduling-linux) scheduler policy
/// * `cpu_time` - the total time units that the cpu has for an iteration, this is used to compute
///                    the `timeslice` of each process.
/// * `minimum_remaining_timeslice` - when a process makes a system call, the scheduler
///                                 has to decode whether to schedule it again for the
///                                 remaining time of its quanta, or to schedule a new
///                                 process. The scheduler will schedule the process
///                                 again of the remaining quanta is greater or equal to
///                                 the `minimum_remaining_timeslice` value.
pub fn cfs(cpu_time: NonZeroUsize, minimum_remaining_timeslice: usize) -> impl Scheduler {
    CFS {
        ready_processes: VecDeque::new(),
        waiting_processes: VecDeque::new(),
        cpu_time,
        minimum_remaining_timeslice,
        remaining_timeslice: 0,
        next_pid: 1,
        sleep: false,
        sleep_time: 0,
    }
}
