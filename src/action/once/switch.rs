//! [`once::switch`] creates a task that only once run system related to [`Switch`](crate::prelude::Switch).
//!
//! - [`once::switch::on`]
//! - [`once::switch::off`]


use bevy::prelude::World;

use crate::action::once;
use crate::action::seed::ActionSeed;
use crate::action::switch::Switch;
use crate::prelude::seed::Seed;

/// Turns [`Switch`](crate::prelude::Switch) on.
#[inline]
pub fn on<M>() -> impl ActionSeed + Seed
    where M: Send + Sync + 'static
{
    once::run(|world: &mut World| {
        Switch::<M>::setup(world, true);
    })
}

/// Turns [`Switch`](crate::prelude::Switch) off.
#[inline]
pub fn off<M>() -> impl ActionSeed + Seed
    where M: Send + Sync + 'static
{
    once::run(|world: &mut World| {
        Switch::<M>::setup(world, false);
    })
}


#[cfg(test)]
mod tests {
    use bevy::app::Startup;
    use bevy::prelude::{Commands, IntoSystemConfigs, ResMut, Update};
    use bevy_test_helper::resource::bool::{Bool, BoolExtension};

    use crate::action::once;
    use crate::prelude::{switch_just_turned_off, switch_just_turned_on};
    use crate::scheduler::Flurx;
    use crate::tests::test_app;

    struct T;

    #[test]
    fn once_switch_on() {
        let mut app = test_app();
        app
            .add_systems(Startup, |mut commands: Commands| {
                commands.spawn(Flurx::schedule(|task| async move {
                    task.will(Update, once::switch::on::<T>()).await;
                }));
            })
            .add_systems(Update, (|mut b: ResMut<Bool>| {
                **b = true;
            }).run_if(switch_just_turned_on::<T>));

        app.update();
        assert!(app.is_bool_true());
    }

    #[test]
    fn once_switch_on_after_1frame() {
        let mut app = test_app();
        app
            .add_systems(Startup, |mut commands: Commands| {
                commands.spawn(Flurx::schedule(|task| async move {
                    task.will(Update, once::run(|| {})).await;
                    task.will(Update, once::switch::on::<T>()).await;
                }));
            })
            .add_systems(Update, (|mut b: ResMut<Bool>| {
                **b = true;
            }).run_if(switch_just_turned_on::<T>));

        app.update();
        assert!(app.is_bool_false());
        app.update();
        assert!(app.is_bool_true());
    }

    #[test]
    fn once_switch_off() {
        let mut app = test_app();
        app
            .add_systems(Startup, |mut commands: Commands| {
                commands.spawn(Flurx::schedule(|task| async move {
                    task.will(Update, once::switch::off::<T>()).await;
                }));
            })
            .add_systems(Update, (|mut b: ResMut<Bool>| {
                **b = true;
            }).run_if(switch_just_turned_off::<T>));

        app.update();
        assert!(app.is_bool_true());
    }

    #[test]
    fn once_switch_off_after_1frame() {
        let mut app = test_app();
        app
            .add_systems(Startup, |mut commands: Commands| {
                commands.spawn(Flurx::schedule(|task| async move {
                    task.will(Update, once::run(|| {})).await;
                    task.will(Update, once::switch::off::<T>()).await;
                }));
            })
            .add_systems(Update, (|mut b: ResMut<Bool>| {
                **b = true;
            }).run_if(switch_just_turned_off::<T>));

        app.update();
        assert!(app.is_bool_false());
        app.update();
        assert!(app.is_bool_true());
    }
}