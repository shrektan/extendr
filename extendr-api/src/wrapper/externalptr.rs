use super::*;
use std::any::Any;
use std::fmt::Debug;

/// Wrapper for creating R objects containing any Rust object.
///
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let extptr = ExternalPtr::new(1);
///     assert_eq!(*extptr, 1);
///     let robj : Robj = extptr.into();
///     let extptr2 : ExternalPtr<i32> = robj.try_into().unwrap();
///     assert_eq!(*extptr2, 1);
/// }
/// ```
///
#[derive(PartialEq, Clone)]
pub struct ExternalPtr<T: Debug + 'static> {
    /// This is the contained Robj.
    pub(crate) robj: Robj,

    /// This is a zero-length object that holds the type of the object.
    marker: std::marker::PhantomData<T>,
}

impl<T: Debug + 'static> Deref for ExternalPtr<T> {
    type Target = T;

    /// This allows us to treat the Robj as if it is the type T.
    fn deref(&self) -> &Self::Target {
        self.addr()
    }
}

impl<T: Debug + 'static> DerefMut for ExternalPtr<T> {
    /// This allows us to treat the Robj as if it is the mutable type T.
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.addr_mut()
    }
}

impl<T: Any + Debug> ExternalPtr<T> {
    /// Construct an external pointer object from any type T.
    /// In this case, the R object owns the data and will drop the Rust object
    /// when the last reference is removed via register_c_finalizer.
    ///
    /// An ExternalPtr behaves like a Box except that the information is
    /// tracked by a R object.
    pub fn new(val: T) -> Self {
        unsafe {
            // This gets the type name of T as a string. eg. "i32".
            let type_name = std::any::type_name::<T>();

            // This allocates some memory for our object and moves the object into it.
            let boxed = Box::new(val);

            // This constructs an external pointer to our boxed data.
            // into_raw() converts the box to a malloced pointer.
            let robj = Robj::make_external_ptr(Box::into_raw(boxed), r!(type_name), r!(()));

            extern "C" fn finalizer<T>(x: SEXP) {
                unsafe {
                    let ptr = R_ExternalPtrAddr(x) as *mut T;

                    // Convert the pointer to a box and drop it implictly.
                    // This frees up the memory we have used and calls the "T::drop" method if there is one.
                    Box::from_raw(ptr);
                }
            }

            // Tell R about our finalizer
            robj.register_c_finalizer(Some(finalizer::<T>));

            // Return an object in a wrapper.
            Self {
                robj,
                marker: std::marker::PhantomData,
            }
        }
    }

    // TODO: make a constructor for references?

    /// Get the "tag" of an external pointer. This is the type name in the common case.
    pub fn tag(&self) -> Robj {
        unsafe { new_owned(R_ExternalPtrTag(self.robj.get())) }
    }

    /// Get the "protected" field of an external pointer. This is NULL in the common case.
    pub fn protected(&self) -> Robj {
        unsafe { new_owned(R_ExternalPtrProtected(self.robj.get())) }
    }

    /// Get the "address" field of an external pointer.
    /// Normally, we will use Deref to do this.
    pub fn addr<'a>(&self) -> &'a T {
        unsafe {
            let ptr = R_ExternalPtrAddr(self.robj.get()) as *const T;
            &*ptr as &'a T
        }
    }

    /// Get the "address" field of an external pointer as a mutable reference.
    /// Normally, we will use DerefMut to do this.
    pub fn addr_mut(&mut self) -> &mut T {
        unsafe {
            let ptr = R_ExternalPtrAddr(self.robj.get()) as *mut T;
            &mut *ptr as &mut T
        }
    }
}

impl<T: Any + Debug> TryFrom<Robj> for ExternalPtr<T> {
    type Error = Error;

    fn try_from(robj: Robj) -> Result<Self> {
        if robj.rtype() != Rtype::ExternalPtr {
            return Err(Error::ExpectedExternalPtr(robj));
        }

        let res = ExternalPtr::<T> {
            robj,
            marker: std::marker::PhantomData,
        };

        // Check the type name.
        let type_name = std::any::type_name::<T>();
        if res.tag().as_str() != Some(type_name) {
            return Err(Error::ExpectedExternalPtrType(res.robj, type_name.into()));
        }

        Ok(res)
    }
}

impl<T: Any + Debug> From<ExternalPtr<T>> for Robj {
    fn from(val: ExternalPtr<T>) -> Self {
        val.robj
    }
}

impl<T: Debug + 'static> std::fmt::Debug for ExternalPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (&*self as &T).fmt(f)
    }
}