use std::sync::{RwLock, Arc, RwLockReadGuard, RwLockWriteGuard};

pub fn runwrap<'a, T>(x: &'a Arc<RwLock<T>>) -> RwLockReadGuard<'a, T> {
    x.read().unwrap()
}

pub fn wunwrap<'a, T>(x: &'a Arc<RwLock<T>>) -> RwLockWriteGuard<'a, T> {
    x.write().unwrap()
}
