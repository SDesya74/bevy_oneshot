use bevy::prelude::*;

/// A trait for oheshot systems
pub trait CommandRunOnce<'w, 's> {
    /// Run system once
    ///
    /// For version with In arguments look at run_once_with
    fn run_once<S, Marker: 'static>(&mut self, command: S)
    where
        S: SystemParamFunction<Marker, In = ()>;

    fn run_once_with<S, Marker: 'static, Args: Send + 'static>(&mut self, command: S, args: Args)
    where
        S: SystemParamFunction<Marker, In = Args>;
}

impl<'w, 's> CommandRunOnce<'w, 's> for Commands<'w, 's> {
    fn run_once<S, Marker: 'static>(&mut self, command: S)
    where
        S: SystemParamFunction<Marker, In = ()>,
    {
        self.run_once_with(command, ());
    }

    fn run_once_with<S, Marker: 'static, Args: Send + 'static>(&mut self, command: S, args: Args)
    where
        S: SystemParamFunction<Marker, In = Args>,
    {
        self.add(move |world: &mut World| {
            let mut system = IntoSystem::into_system(command);
            system.initialize(world);
            system.run(args, world);
            system.apply_buffers(world);
        });
    }
}
