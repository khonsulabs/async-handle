use async_rwlock::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{
    fmt::{Debug, Display, Formatter},
    sync::Arc,
};

#[derive(Clone)]
/// Reference-counted async RwLock
pub struct Handle<T> {
    data: Arc<RwLock<T>>,
}

impl<T> Handle<T> {
    /// Creates a new handle wrapping `value`
    pub fn new(value: T) -> Self {
        Self {
            data: Arc::new(RwLock::new(value)),
        }
    }

    /// Lock the contained value for reading
    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
        self.data.read().await
    }

    /// Lock the contained value for writing
    pub async fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.data.write().await
    }

    pub fn try_unwrap(self) -> Result<T, Self> {
        match Arc::try_unwrap(self.data) {
            Ok(lock) => Ok(lock.into_inner()),
            Err(data) => Err(Self { data }),
        }
    }
}

impl<T> Debug for Handle<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        futures_lite::future::block_on(async {
            let data = self.read().await;
            data.fmt(f)
        })
    }
}

impl<T> Display for Handle<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        futures_lite::future::block_on(async {
            let data = self.read().await;
            data.fmt(f)
        })
    }
}

impl<T> Default for Handle<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}
