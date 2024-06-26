//! `once` creates a task that only once run system.
//!
//! actions
//!
//! - [`once::res`](crate::prelude::once::res)
//! - [`once::non_send`](crate::prelude::once::res)
//! - [`once::event`](crate::prelude::once::res)
//! - [`once::state`](crate::prelude::once::res)
//! - [`once::switch`](crate::prelude::once::switch)
//! - [`once::audio`](crate::prelude::once::audio) (require feature flag `audio`)


use bevy::prelude::{IntoSystem, System, World};

use crate::action::seed::ActionSeed;
use crate::prelude::Action;
use crate::runner::{CancellationToken, Output, Runner};

pub mod res;
pub mod non_send;
pub mod event;
pub mod state;
pub mod switch;
#[cfg(feature = "audio")]
pub mod audio;


/// Once run a system.
///
/// The return value will be the system return value.
///
/// ## Examples
///
/// ```no_run
/// use bevy::app::AppExit;
/// use bevy::prelude::{World, Update, EventWriter};
/// use bevy_flurx::prelude::*;
///
/// Reactor::schedule(|task| async move{
///     task.will(Update, once::run(|mut ew: EventWriter<AppExit>|{
///         ew.send(AppExit);
///     })).await;
/// });
/// ```
#[inline(always)]
pub fn run<Sys, I, Out, M>(system: Sys) -> ActionSeed<I, Out>
    where
        Sys: IntoSystem<I, Out, M> + 'static,
        I: 'static,
        Out: 'static
{
    ActionSeed::new(move |input, output| {
        OnceRunner{
            input: Some(input),
            output,
            system: IntoSystem::into_system(system)      
        }
    })
}

/// Once run a system with input.
///
/// The return value will be the system return value.
///
/// ## Examples
///
/// ```no_run
/// use bevy::app::AppExit;
/// use bevy::prelude::{World, Update, EventWriter, In};
/// use bevy_flurx::prelude::*;
///
/// Reactor::schedule(|task| async move{
///     task.will(Update, once::run_with(1, |In(num): In<usize>|{
///         num + 1
///     })).await;
/// });
/// ```
#[inline(always)]
pub fn run_with<Sys, Input, Out, Marker>(input: Input, system: Sys) -> Action<Input, Out>
    where
        Sys: IntoSystem<Input, Out, Marker> + Clone + 'static,
        Input: 'static,
        Out: 'static
{
    run(system).with(input)
}

struct OnceRunner<Sys, I, O> {
    system: Sys,
    input: Option<I>,
    output: Output<O>,
}

impl<Sys, I, O> Runner for OnceRunner<Sys, I, O>
    where
        Sys: System<In=I, Out=O>,
        I: 'static,
        O: 'static
{
    fn run(&mut self, world: &mut World, _: &CancellationToken) -> bool {
        self.system.initialize(world);
        let Some(input) = self.input.take() else {
            return true;
        };
        let out = self.system.run(input, world);
        self.system.apply_deferred(world);
        self.output.set(out);
        true
    }
}

