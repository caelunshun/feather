/// Asserts that a floating-point value is within
/// a certain range of the expected value.
#[cfg(test)]
macro_rules! assert_float_eq {
    ($left:expr, $right:expr) => {
        assert_float_eq!($left, $right, 0.001);
    };
    ($left:expr, $right:expr, $range:expr) => {
        let range = ($left - $range)..($left + $range);
        assert!(range.contains(&$right));
    };
}

/// Generates a setup() implementation for a system
/// which initializes an internal event reader.
macro_rules! setup_impl {
    ($($reader:ident),+) => {
        fn setup(&mut self, world: &mut specs::World) {
            use specs::SystemData;
            use shrev::EventChannel;
            Self::SystemData::setup(world);

            $(self.$reader = Some(world.fetch_mut::<EventChannel<_>>().register_reader());)+
        }
    };
}

macro_rules! flagged_setup_impl {
    ($component:ident, $reader:ident) => {
        fn setup(&mut self, world: &mut specs::World) {
            use specs::{SystemData, WorldExt};
            Self::SystemData::setup(world);

            self.$reader = Some(world.write_component::<$component>().register_reader());
        }
    }
}
