//! Testing the difference between not using Flurx and using Flurx in a simple countdown.
#![allow(missing_docs)]

use bevy::app::{App, AppExit, Startup};
use bevy::prelude::{Commands, EventReader, EventWriter, Local, ResMut, Resource, Update, World};
use criterion::{Criterion, criterion_group, criterion_main};

use bevy_flurx::{FlurxPlugin, sequence};
use bevy_flurx::extension::ScheduleReactor;
use bevy_flurx::prelude::{Flurx, once, wait};

#[derive(Resource, Default)]
struct Exit(bool);

const LIMIT: usize = 1000;

fn default_version(c: &mut Criterion) {
    c.bench_function("default_version", |b| {
        b.iter(|| {
            let mut app = App::new();
            app
                .init_resource::<Exit>()
                .add_systems(Update, |mut ew: EventWriter<AppExit>, mut count: Local<usize>| {
                    *count += 1;
                    if *count == LIMIT {
                        ew.send(AppExit);
                    }
                })
                .add_systems(Update, |mut exit: ResMut<Exit>, mut er: EventReader<AppExit>| {
                    if er.read().last().is_some() {
                        exit.0 = true;
                    }
                });

            while !app.world.resource::<Exit>().0 {
                app.update();
            }
        });
    });
}

fn flurx_version(c: &mut Criterion) {
    c.bench_function("flurx_version", |b| {
        b.iter(|| {
            let mut app = App::new();
            app
                .add_plugins(FlurxPlugin)
                .init_resource::<Exit>()
                .add_systems(Startup, |mut commands: Commands| {
                    commands.spawn(Flurx::schedule(|task| async move {
                        task.will(Update, wait::until(|mut count: Local<usize>| {
                            *count += 1;
                            *count == LIMIT
                        })).await;
                        task.will(Update, once::run(|mut exit: ResMut<Exit>| {
                            exit.0 = true;
                        })).await;
                    }));
                });

            while !app.world.resource::<Exit>().0 {
                app.update();
            }
        });
    });
}

fn flurx_sequence_version(c: &mut Criterion) {
    c.bench_function("flurx_sequence_version", |b| {
        b.iter(|| {
            let mut app = App::new();
            app
                .add_plugins(FlurxPlugin)
                .init_resource::<Exit>()
                .add_systems(Startup, |mut commands: Commands| {
                    commands.spawn(Flurx::schedule(|task| async move {
                        task.will(Update, sequence!(
                            wait::until(|mut count: Local<usize>| {
                                *count += 1;
                                *count == LIMIT
                            }),
                            once::run(|mut exit: ResMut<Exit>| {
                                exit.0 = true;
                            })
                        )).await;
                    }));
                });

            while !app.world.resource::<Exit>().0 {
                app.update();
            }
        });
    });
}

criterion_group!(benches, default_version, flurx_version, flurx_sequence_version);
criterion_main!(benches);