use std::{
    collections::HashMap,
    f32::consts::{FRAC_PI_2, PI},
    ops::Range,
};

use bevy::{
    camera::ScalingMode,
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};
use serde::{Deserialize, Serialize};
