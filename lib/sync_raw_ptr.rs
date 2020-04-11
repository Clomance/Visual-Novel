use core::convert::*;
use core::borrow::Borrow;
use core::ops::Deref;

// Сырой указатель, который можно свободно передавать между потоками
pub struct SyncRawPtr<T>{
    ptr:*mut T,
}

impl<T> SyncRawPtr<T>{
    pub fn new(item:*mut T)->SyncRawPtr<T>{
        Self{
            ptr:item,
        }
    }
}


unsafe impl<T:Send> Send for SyncRawPtr<T>{}    // Типажи для передачи
unsafe impl<T:Sync> Sync for SyncRawPtr<T>{}    // между потоками

impl<T> AsRef<T> for SyncRawPtr<T>{
    fn as_ref(&self)->&T{
        unsafe{
            &*self.ptr
        }
    }
}

impl<T> AsMut<T> for SyncRawPtr<T>{
    fn as_mut(&mut self)->&mut T{
        unsafe{
            &mut *self.ptr
        }
    }
}

impl<T> Borrow<T> for SyncRawPtr<T>{
    fn borrow(&self)->&T{
        unsafe{
            &*self.ptr
        }
    }
}

impl<T> Deref for SyncRawPtr<T>{
    type Target=T;
    fn deref(&self)->&T{
        unsafe{
            &*self.ptr
        }
    }
}