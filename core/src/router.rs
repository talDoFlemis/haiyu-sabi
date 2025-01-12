use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};

use anyhow::Result;

use crate::{actor::Actor, error::RouterError};

/// Router defines a base actor that distributes incomming data through a given task for it's
/// assigned actor or supervisor
#[derive(Default)]
pub struct Router<A = ()> {
    inner: PathRouter<A>,
}

impl<A> Router<A>
where
    A: Actor,
{
    pub fn new() -> Self {
        Self {
            inner: PathRouter::new(),
        }
    }

    pub fn route(&mut self, task: &str, actor: A) -> Self {
        self.inner.route(task, actor).expect("failed to route");

        Self {
            inner: self.inner.clone(),
        }
    }
}

#[derive(Clone, Default)]
pub struct PathRouter<ActorType> {
    routes: HashMap<String, ActorType>,
}

impl<A> PathRouter<A>
where
    A: Actor,
{
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn route(&mut self, task: &str, actor: A) -> Result<()> {
        let task = task.to_string();

        match self.routes.entry(task.clone()) {
            Occupied(_) => {
                anyhow::bail!(RouterError::TaskAlreadyRegistered(
                    task,
                    "here change after".to_string(),
                ))
            }
            Vacant(vacant_entry) => vacant_entry.insert(actor),
        };

        Ok(())
    }
}
