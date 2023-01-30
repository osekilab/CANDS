use std::fmt;



macro_rules! wh_feature { () => { SyntacticFeature::Normal(f!("wh")) }; }
macro_rules! epp_feature { () => { SyntacticFeature::Normal(f!("EPP")) }; }
macro_rules! comp_feature { () => { SyntacticFeature::Normal(f!("C")) } }
macro_rules! strong_light_verb_feature { () => { SyntacticFeature::Normal(f!("v*")) } }

pub(crate) use { wh_feature, epp_feature, comp_feature, strong_light_verb_feature };

pub const CATSEL_FEATURE_PREFIX: &'static str = "=";



/// Features are identified by strings.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Feature(pub String);



impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}



impl Feature {
    pub fn new(s: String) -> Self {
        Feature(s)
    }
}



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SyntacticFeature {
    Normal(Feature),
    Valuable {
        interpretable: bool,
        feature: Feature,
        value: Option<String>,
    },
}



impl fmt::Display for SyntacticFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntacticFeature::Normal(feature) =>
                write!(f, "{}", feature),

            SyntacticFeature::Valuable { interpretable, feature, value } =>
                write!(f, "{}{}:{}",
                    if *interpretable { "" } else { "u" },
                    feature,
                    match value {
                        Some(value) => value,
                        None => "_"
                    }
                ),
        }
    }
}



impl SyntacticFeature {
    pub fn is_interpretable(&self) -> bool {
        match self {
            SyntacticFeature::Valuable { interpretable: true, .. } => true,
            _ => false,
        }
    }

    pub fn is_uninterpretable(&self) -> bool {
        match self {
            SyntacticFeature::Valuable { interpretable: false, .. } => true,
            _ => false,
        }
    }

    pub fn is_person(&self) -> bool {
        match self {
            SyntacticFeature::Valuable { feature, .. } =>
                feature.0 == "Person",
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            SyntacticFeature::Valuable { feature, .. } =>
                feature.0 == "Number",
            _ => false,
        }
    }

    pub fn matches(&self, other: &SyntacticFeature) -> bool {
        match (self, other) {
            (SyntacticFeature::Valuable { feature: feature1, .. },
             SyntacticFeature::Valuable { feature: feature2, .. }) => {
                feature1 == feature2
            },
            _ => false,
        }
    }
}



/// Macro to generate a feature.
///
/// # Example
///
/// ```
/// cands::f!("wh")
/// ```
#[macro_export]
macro_rules! f {
    ($literal:expr) => {
        Feature::new(String::from($literal))
    };
}

pub(crate) use f;



/// Macro to generate a feature set.
/// 
/// # Example
/// 
/// ```
/// fset!("n", "+voiced", "EPP")
/// ```
#[macro_export]
macro_rules! fset {
    ($($literal:expr),*) => {
        {
            set!( $(f!($literal)),* )
        }
    };
}

pub(crate) use fset;



/// Macro to generate a feature vector.
/// 
/// # Example
/// 
/// ```
/// fvec!["n", "+voiced", "EPP"]
/// ```
#[macro_export]
macro_rules! fvec {
    [$($literal:expr),*] => {
        {
            vec![ $(f!($literal)),* ]
        }
    };
}

pub(crate) use fvec;



#[cfg(test)]
mod tests {
    /// Tests for the `fset!` macro.
    mod fset {
        use crate::prelude::*;

        /// Make an empty fset.
        #[test]
        fn empty_fset() {
            let set1: Set<Feature> = fset!();
            let set2: Set<Feature> = Set::new();
            assert_eq!(set1, set2);
        }

        /// Make a fset with three distinct elements.
        #[test]
        fn three_elems() {
            let set1: Set<Feature> = fset!("alpha", "beta", "gamma");
            let mut set2: Set<Feature> = Set::new();

            set2.insert(f!("alpha"));
            set2.insert(f!("beta"));
            set2.insert(f!("gamma"));

            assert_eq!(set1, set2);
        }
        
        /// Make a fset with duplicates. The resulting fset should not contain
        /// duplicates.
        #[test]
        fn dedup() {
            let set1: Set<Feature> = fset!("alpha", "alpha", "beta");
            let mut set2: Set<Feature> = Set::new();

            set2.insert(f!("alpha"));
            set2.insert(f!("beta"));

            assert_eq!(set1, set2);
        }
    }
}