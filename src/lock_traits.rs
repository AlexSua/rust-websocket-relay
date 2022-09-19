use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub trait ReadLockTrait<T> {
    fn lock_and_write(&self, callback: impl FnOnce(&mut RwLockWriteGuard<T>) -> ());
    fn lock_and_read(&self, callback: impl FnOnce(&mut RwLockReadGuard<T>) -> ());
}

pub trait MutexTrait<T> {
    fn lock_and_go(&self, callback: impl FnOnce(&mut MutexGuard<T>));
}

impl<T> ReadLockTrait<T> for RwLock<T> {
    fn lock_and_write(&self, callback: impl FnOnce(&mut RwLockWriteGuard<T>) -> ()) {
        let mut rwlock = self.write().expect("Write lock poisoned");
        callback(&mut rwlock);
    }

    fn lock_and_read(&self, callback: impl FnOnce(&mut RwLockReadGuard<T>) -> ()) {
        let mut rwlock = self.try_read().expect("Read lock poisoned");
        callback(&mut rwlock);
    }
}

impl<T> MutexTrait<T> for Mutex<T> {
    fn lock_and_go(&self, callback: impl FnOnce(&mut MutexGuard<T>)) {
        let mut mutex = self.lock().expect("Mutex poisoned");
        callback(&mut mutex);
    }
}
