use std::fmt;

struct Int16Vec(Vec<i16>);

impl fmt::Display for Int16Vec {
    fn fmt(&self, f: &mut
