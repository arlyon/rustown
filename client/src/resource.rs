//! # Resource System
//!
//! Resources are units of matter that exist
//! in the world, whether that be electricity,
//! or water, or fuel. They differ from items
//! in that they are fungible. Two units of
//! water are "the same" whereas items are not.

use std::cmp;
use std::collections::HashMap;
use std::default;
use std::ops;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Resource {
    name: String,
}

/// A resource bank is a set of resources and
/// amounts,grouped together with a set of
/// operations over it for easier usability.
#[derive(Eq, PartialEq)]
pub struct ResourceBank {
    amount: HashMap<Resource, i64>,
}

impl ResourceBank {
    pub fn merge(&mut self, other: Self) -> &mut Self {
        self.amount.extend(other.amount);
        self
    }
}

impl default::Default for ResourceBank {
    fn default() -> Self {
        Self {
            amount: HashMap::new(),
        }
    }
}

impl<'a, 'b> ops::Add<&'b ResourceBank> for &'a ResourceBank {
    type Output = ResourceBank;

    fn add(self, rhs: &'b ResourceBank) -> ResourceBank {
        ResourceBank {
            amount: self
                .amount
                .clone()
                .into_iter()
                .chain(rhs.amount.clone())
                .collect(),
        }
    }
}

impl<'a, 'b> ops::Sub<&'b ResourceBank> for &'a ResourceBank {
    type Output = ResourceBank;

    fn sub(self, rhs: &'b ResourceBank) -> ResourceBank {
        ResourceBank {
            amount: self
                .amount
                .clone()
                .into_iter()
                .chain((-rhs).amount)
                .collect(),
        }
    }
}

impl<'a> ops::Neg for &'a ResourceBank {
    type Output = ResourceBank;

    fn neg(self) -> ResourceBank {
        ResourceBank {
            amount: self
                .amount
                .clone()
                .into_iter()
                .map(|(r, i)| (r, -i))
                .collect(),
        }
    }
}

impl ops::Index<Resource> for ResourceBank {
    type Output = i64;

    fn index(&self, index: Resource) -> &Self::Output {
        &self.amount[&index]
    }
}

/// ResourceBanks can only be partially ordered
/// because there exist situations (ie when the
/// set of resources in A and the set of resources
/// in B are disjoint) when they are neither
/// greater than, less than, or equal–only unequal.
impl cmp::PartialOrd for ResourceBank {
    fn partial_cmp(&self, other: &ResourceBank) -> Option<cmp::Ordering> {
        let mut resources = (self - other).amount.into_iter().map(|(_, i)| i);
        let first = resources.next().partial_cmp(&Some(0));
        resources.fold(first, |state, next| {
            if next.partial_cmp(&0) == state {
                state
            } else {
                None
            }
        })
    }
}
