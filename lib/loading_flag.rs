#[derive(Clone,Copy)]
pub enum ThreadState{
    Running,
    Finished,
    Panicked,
}

pub struct LoadingFlag{
    state:Box<ThreadState>,
}

impl LoadingFlag{
    pub fn new()->LoadingFlag{
        Self{
            state:Box::new(ThreadState::Running),
        }
    }

    pub fn set_state(&mut self,state:ThreadState){
        *self.state.as_mut()=state;
    }

    pub fn get_state(&self)->ThreadState{
        self.state.as_ref().clone()
    }

    pub fn ptr(&mut self)->LoadingFlagSmartPtr{
        LoadingFlagSmartPtr{
            ptr:self.state.as_mut() as *mut ThreadState
        }
    }
}

pub struct LoadingFlagSmartPtr{
    ptr:*mut ThreadState,
}

impl LoadingFlagSmartPtr{
    pub fn get_state(&self)->ThreadState{
        unsafe{
            self.ptr.read()
        }
    }
}

impl Drop for LoadingFlagSmartPtr{
    fn drop(&mut self){
        unsafe{
            if std::thread::panicking(){
                *self.ptr=ThreadState::Panicked
            }
            else{
                *self.ptr=ThreadState::Finished
            }
        }
    }
}

unsafe impl std::marker::Send for LoadingFlagSmartPtr{}
unsafe impl std::marker::Sync for LoadingFlagSmartPtr{}