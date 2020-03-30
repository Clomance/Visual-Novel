use core::convert::*;
use core::ptr;
use core::borrow::Borrow;
use core::ops::Deref;
use core::ops::DerefMut;

// Структура для свободного использования ссылка на объект между потоками
// Только для некопирующихся типов - с ними возможны проблемы
pub struct SyncRawPtr<T>{
    ptr:*mut T,
}

impl<T> SyncRawPtr<T>{
    pub const fn null()->SyncRawPtr<T>{
        Self{
            ptr:ptr::null_mut(),
        }
    }
    pub fn new(item:&mut T)->SyncRawPtr<T>{
        Self{
            ptr:item as *mut T,
        }
    }
    pub fn set(&mut self,item:&mut T){
        self.ptr=item as *mut T;
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
            &mut*self.ptr
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

impl<T> DerefMut for SyncRawPtr<T>{
    fn deref_mut(&mut self)->&mut T{
        unsafe{
            &mut*self.ptr
        }
    }
}