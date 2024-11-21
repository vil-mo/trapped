use bevy::{
    app::Update,
    prelude::{Res, ResMut, Resource},
    time::{Time, Timer, TimerMode},
};
use std::time::Duration;

pub struct ExecutingActionTimerPlugin;

impl bevy::app::Plugin for ExecutingActionTimerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<ExecutingActionTimer>();
        app.add_systems(Update, update_executing_action_timer);
    }
}

#[derive(Resource)]
pub struct ExecutingActionTimer(Timer);

impl Default for ExecutingActionTimer {
    fn default() -> Self {
        let mut timer = Timer::new(Duration::ZERO, TimerMode::Once);
        timer.tick(Duration::ZERO);
        ExecutingActionTimer(timer)
    }
}

impl ExecutingActionTimer {
    pub fn start(&mut self, duration: Duration) {
        self.0 = Timer::new(duration, TimerMode::Once);
    }

    /// True if the action is currently executing
    pub fn executing(&self) -> bool {
        !self.0.finished()
    }
}

fn update_executing_action_timer(mut action: ResMut<ExecutingActionTimer>, time: Res<Time>) {
    action.0.tick(time.delta());
}
