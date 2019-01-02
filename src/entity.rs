use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Entity<T>(PhantomData<T>);

#[derive(Clone, Default)]
pub struct EntityDef {
    pub table_name: String,
    pub columns: HashMap<String, String>,
}

pub trait HasEntityDef {
    fn entity_def() -> EntityDef;
}

impl<T> Default for Entity<T> {
    fn default() -> Entity<T> {
        Entity(PhantomData)
    }
}

impl<T: HasEntityDef> HasEntityDef for Entity<T> {
    fn entity_def() -> EntityDef {
        T::entity_def()
    }
}

impl<T: HasEntityDef> HasEntityDef for Option<T> {
    fn entity_def() -> EntityDef {
        T::entity_def()
    }
}
