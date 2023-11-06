#[macro_export]
macro_rules! common {
    ($ty:ident, $($name:ident, $pretty:expr);+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, specta::Type)]
        pub enum $ty {
            $($name),+
        }


        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                match self {
                    $(
                        Self::$name => write!(f, "{}", $pretty),
                    )+
                }
            }
        }

        impl $ty {
            pub const fn variants() -> &'static [Self] {
                &[$(Self::$name),+]
            }
        }
    };
}

#[macro_export]
macro_rules! impl_absorber {
    ($($name:ident, $pretty:expr, $per_m2:expr, $absorp:expr);+) => {
        $crate::common!(Absorber, $($name, $pretty);+);

        impl Absorber {
            pub fn absoptivity(&self) -> f64 {
                match self {
                    $(
                        Self::$name => $absorp,
                    )+
                }
            }

            pub fn cost_per_m2(&self, m2: f64) -> f64 {
                match self {
                    $(
                        Self::$name => $per_m2 * m2,
                    )+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_reflective {
    ($($name:ident, $pretty:expr, $per_m2:expr, $reflectivity:expr);+) => {
        $crate::common!(ReflectiveMaterial, $($name, $pretty);+);

        impl ReflectiveMaterial {
            pub fn reflectivity(&self) -> f64 {
                match self {
                    $(
                        Self::$name => $reflectivity,
                    )+
                }
            }

            pub fn cost_per_m2(&self, m2: f64) -> f64 {
                match self {
                    $(
                        Self::$name => $per_m2 * m2,
                    )+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_insulator {
    ($($name:ident, $pretty:expr, $per_m2:expr, $conductivity:expr);+) => {
        $crate::common!(Insulator, $($name, $pretty);+);

        impl Insulator {
            pub fn conductivity(&self) -> f64 {
                match self {
                    $(
                        Self::$name => $conductivity,
                    )+
                }
            }

            pub fn cost_per_m3(&self, vol: f64) -> f64 {
                match self {
                    $(
                        Self::$name => $per_m2 * vol,
                    )+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_body {
    ($($name:ident, $pretty:expr, $per_m2:expr, $conductivity:expr, $thick:expr);+) => {
        $crate::common!(BodyMaterial, $($name, $pretty);+);

        impl BodyMaterial {
            pub fn conductivity(&self) -> f64 {
                match self {
                    $(
                        Self::$name => $conductivity,
                    )+
                }
            }

            pub fn cost_per_m2(&self, m2: f64) -> f64 {
                match self {
                    $(
                        Self::$name => $per_m2 * m2,
                    )+
                }
            }

            pub fn thickness(&self) -> f64 {
                match self {
                    $(
                        Self::$name => $thick,
                    )+
                }
            }
        }
    };
}
