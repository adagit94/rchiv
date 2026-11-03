/// Common trait to raw pointers readable via immutable get() and writable via set(value) method that dereferences raw pointer internally and writes value directly.
pub trait RawPtrTrait<T> {
    fn get(&self) -> *const T;
    fn set(&self, value: T);
}

macro_rules! impl_raw_ptr_trait {
    ($struct_name: ident) => {
        impl<T> RawPtrTrait<T> for $struct_name<T> {
            fn get(&self) -> *const T {
                self.0
            }

            fn set(&self, value: T) {
                unsafe {
                    *self.0 = value;
                }
            }
        }
    };
}

/// Clonable wrapper for a raw pointer (*mut) that implements Send & Sync. E.g. for a shared read/write access by multiple threads without usage of atomics. 
pub struct RawPtr<T>(*mut T);
impl_raw_ptr_trait!(RawPtr);
unsafe impl<T> Send for RawPtr<T> {}
unsafe impl<T> Sync for RawPtr<T> {}
impl<T> Clone for RawPtr<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

/// Auxilliary abstraction to wrap incoming value into heap allocated Box and save it's mutable raw pointer (*mut) for further read/write operations.
/// 
/// Beware that when Raw get's dropped, underlaying value will be deallocated and same pointer passed into RawPtr via into_ptr() becomes basically invalid and potentially dangling at places where RawPtr is used.
/// 
/// Abstraction is intended rather for edges cases as producer of clonable RawPtr structs that implement Send & Sync for a shared read/write access (e.g. by multiple threads) without usage of atomics. 
pub struct Raw<T>(*mut T);
impl<T> Raw<T> {
  /// Wrap incoming value into heap allocated Box and save it's mutable raw pointer (*mut) for further read/write operations.
  pub fn new(value: T) -> Self {
    Self(Box::into_raw(Box::new(value)))
  }
  
  /// Producer of clonable RawPtr structs that implement Send & Sync. Beware that when Raw get's dropped, underlaying value will be deallocated and same pointer passed into RawPtr becomes basically invalid and potentially dangling at places where RawPtr is used. 
  pub fn into_ptr(&self) -> RawPtr<T> {
    RawPtr(self.0)
  }
}

impl_raw_ptr_trait!(Raw);
impl<T> Drop for Raw<T> {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.0));
        };
    }
}
