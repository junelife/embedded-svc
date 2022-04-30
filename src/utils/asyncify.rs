#[cfg(feature = "alloc")]
pub mod event_bus;
#[cfg(feature = "alloc")]
pub mod mqtt;
#[cfg(feature = "alloc")]
pub mod timer;
#[cfg(feature = "alloc")]
pub mod ws;

pub trait AsyncWrapper<S> {
    fn new(sync: S) -> Self;
}

pub trait Asyncify {
    type AsyncWrapper<S>: AsyncWrapper<S>;

    fn into_async(self) -> Self::AsyncWrapper<Self>
    where
        Self: Sized,
    {
        Self::AsyncWrapper::new(self)
    }

    fn as_async(&mut self) -> Self::AsyncWrapper<&mut Self> {
        Self::AsyncWrapper::new(self)
    }
}

pub trait UnblockingAsyncWrapper<U, S> {
    fn new(unblocker: U, sync: S) -> Self;
}

pub trait UnblockingAsyncify {
    type AsyncWrapper<U, S>: UnblockingAsyncWrapper<U, S>;

    fn into_async_with_unblocker<U>(self, unblocker: U) -> Self::AsyncWrapper<U, Self>
    where
        Self: Sized,
    {
        Self::AsyncWrapper::new(unblocker, self)
    }

    fn as_async_with_unblocker<U>(&mut self, unblocker: U) -> Self::AsyncWrapper<U, &mut Self> {
        Self::AsyncWrapper::new(unblocker, self)
    }
}
