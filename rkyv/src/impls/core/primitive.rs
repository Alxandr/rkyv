use core::{
    marker::{PhantomData, PhantomPinned},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8,
        NonZeroIsize, NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64,
        NonZeroU8, NonZeroUsize,
    },
};

use rancor::Fallible;

use crate::{
    primitive::{
        ArchivedChar, ArchivedF32, ArchivedF64, ArchivedI128, ArchivedI16,
        ArchivedI32, ArchivedI64, ArchivedIsize, ArchivedNonZeroI128,
        ArchivedNonZeroI16, ArchivedNonZeroI32, ArchivedNonZeroI64,
        ArchivedNonZeroIsize, ArchivedNonZeroU128, ArchivedNonZeroU16,
        ArchivedNonZeroU32, ArchivedNonZeroU64, ArchivedNonZeroUsize,
        ArchivedU128, ArchivedU16, ArchivedU32, ArchivedU64, ArchivedUsize,
    },
    Archive, Archived, Deserialize, Portable, Serialize,
};

macro_rules! unsafe_impl_portable {
    ($($ty:ty),* $(,)?) => {
        $(unsafe impl Portable for $ty {})*
    };
}

unsafe_impl_portable! {
    (),
    bool,
    i8,
    u8,
    NonZeroI8,
    NonZeroU8,
    rend::NonZeroI16_be,
    rend::NonZeroI16_le,
    rend::NonZeroI32_be,
    rend::NonZeroI32_le,
    rend::NonZeroI64_be,
    rend::NonZeroI64_le,
    rend::NonZeroI128_be,
    rend::NonZeroI128_le,
    rend::NonZeroU16_be,
    rend::NonZeroU16_le,
    rend::NonZeroU32_be,
    rend::NonZeroU32_le,
    rend::NonZeroU64_be,
    rend::NonZeroU64_le,
    rend::NonZeroU128_be,
    rend::NonZeroU128_le,
    rend::char_be,
    rend::char_le,
    rend::f32_be,
    rend::f32_le,
    rend::f64_be,
    rend::f64_le,
    rend::i16_be,
    rend::i16_le,
    rend::i32_be,
    rend::i32_le,
    rend::i64_be,
    rend::i64_le,
    rend::i128_be,
    rend::i128_le,
    rend::u16_be,
    rend::u16_le,
    rend::u32_be,
    rend::u32_le,
    rend::u64_be,
    rend::u64_le,
    rend::u128_be,
    rend::u128_le,
}

unsafe impl<T: Portable, const N: usize> Portable for [T; N] {}
unsafe impl<T: Portable> Portable for [T] {}

macro_rules! impl_serialize_noop {
    ($type:ty) => {
        impl<S: Fallible + ?Sized> Serialize<S> for $type {
            #[inline]
            fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
                Ok(())
            }
        }
    };
}

macro_rules! impl_portable_primitive {
    ($type:ty) => {
        impl Archive for $type {
            type Archived = Self;
            type Resolver = ();

            #[inline]
            unsafe fn resolve(
                &self,
                _: usize,
                _: Self::Resolver,
                out: *mut Self::Archived,
            ) {
                out.write(*self);
            }
        }

        impl_serialize_noop!($type);

        impl<D: Fallible + ?Sized> Deserialize<$type, D> for Archived<$type> {
            #[inline]
            fn deserialize(&self, _: &mut D) -> Result<$type, D::Error> {
                Ok(*self)
            }
        }
    };
}

macro_rules! impl_portable_primitives {
    ($($type:ty;)*) => {
        $(
            impl_portable_primitive!($type);
        )*
    }
}

impl_portable_primitives! {
    ();
    bool;
    i8;
    u8;
    NonZeroI8;
    NonZeroU8;
}

macro_rules! impl_multibyte_primitive {
    ($archived:ident: $type:ty) => {
        impl Archive for $type {
            type Archived = $archived;
            type Resolver = ();

            #[inline]
            unsafe fn resolve(
                &self,
                _: usize,
                _: Self::Resolver,
                out: *mut Self::Archived,
            ) {
                out.write(<$archived>::from_native(*self));
            }
        }

        impl_serialize_noop!($type);

        impl<D: Fallible + ?Sized> Deserialize<$type, D> for $archived {
            #[inline]
            fn deserialize(&self, _: &mut D) -> Result<$type, D::Error> {
                Ok(self.to_native())
            }
        }
    };
}

macro_rules! impl_multibyte_primitives {
    ($($archived:ident: $type:ty),* $(,)?) => {
        $(
            impl_multibyte_primitive!($archived: $type);
        )*
    };
}

impl_multibyte_primitives! {
    ArchivedI16: i16,
    ArchivedI32: i32,
    ArchivedI64: i64,
    ArchivedI128: i128,
    ArchivedU16: u16,
    ArchivedU32: u32,
    ArchivedU64: u64,
    ArchivedU128: u128,
    ArchivedF32: f32,
    ArchivedF64: f64,
    ArchivedChar: char,
    ArchivedNonZeroI16: NonZeroI16,
    ArchivedNonZeroI32: NonZeroI32,
    ArchivedNonZeroI64: NonZeroI64,
    ArchivedNonZeroI128: NonZeroI128,
    ArchivedNonZeroU16: NonZeroU16,
    ArchivedNonZeroU32: NonZeroU32,
    ArchivedNonZeroU64: NonZeroU64,
    ArchivedNonZeroU128: NonZeroU128,
}

// PhantomData

unsafe impl<T: ?Sized> Portable for PhantomData<T> {}

impl<T: ?Sized> Archive for PhantomData<T> {
    type Archived = PhantomData<T>;
    type Resolver = ();

    #[inline]
    unsafe fn resolve(
        &self,
        _: usize,
        _: Self::Resolver,
        _: *mut Self::Archived,
    ) {
    }
}

impl<T: ?Sized, S: Fallible + ?Sized> Serialize<S> for PhantomData<T> {
    #[inline]
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<T: ?Sized, D: Fallible + ?Sized> Deserialize<PhantomData<T>, D>
    for PhantomData<T>
{
    #[inline]
    fn deserialize(&self, _: &mut D) -> Result<PhantomData<T>, D::Error> {
        Ok(PhantomData)
    }
}

// PhantomPinned

unsafe_impl_portable!(PhantomPinned);

impl Archive for PhantomPinned {
    type Archived = PhantomPinned;
    type Resolver = ();

    #[inline]
    unsafe fn resolve(
        &self,
        _: usize,
        _: Self::Resolver,
        _: *mut Self::Archived,
    ) {
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for PhantomPinned {
    #[inline]
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<D: Fallible + ?Sized> Deserialize<PhantomPinned, D> for PhantomPinned {
    #[inline]
    fn deserialize(&self, _: &mut D) -> Result<PhantomPinned, D::Error> {
        Ok(PhantomPinned)
    }
}

// usize

impl Archive for usize {
    type Archived = ArchivedUsize;
    type Resolver = ();

    #[inline]
    unsafe fn resolve(
        &self,
        _: usize,
        _: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        out.write(ArchivedUsize::from_native(*self as _));
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for usize {
    #[inline]
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<D: Fallible + ?Sized> Deserialize<usize, D> for ArchivedUsize {
    #[inline]
    fn deserialize(&self, _: &mut D) -> Result<usize, D::Error> {
        Ok(self.to_native() as usize)
    }
}

// isize

impl Archive for isize {
    type Archived = ArchivedIsize;
    type Resolver = ();

    #[inline]
    unsafe fn resolve(
        &self,
        _: usize,
        _: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        out.write(ArchivedIsize::from_native(*self as _));
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for isize {
    #[inline]
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<D: Fallible + ?Sized> Deserialize<isize, D> for Archived<isize> {
    #[inline]
    fn deserialize(&self, _: &mut D) -> Result<isize, D::Error> {
        Ok(self.to_native() as isize)
    }
}

// NonZeroUsize

impl Archive for NonZeroUsize {
    type Archived = ArchivedNonZeroUsize;
    type Resolver = ();

    #[inline]
    unsafe fn resolve(
        &self,
        _: usize,
        _: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        out.write(ArchivedNonZeroUsize::new_unchecked(self.get() as _));
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for NonZeroUsize {
    #[inline]
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<D: Fallible + ?Sized> Deserialize<NonZeroUsize, D>
    for Archived<NonZeroUsize>
{
    #[inline]
    fn deserialize(&self, _: &mut D) -> Result<NonZeroUsize, D::Error> {
        Ok(unsafe { NonZeroUsize::new_unchecked(self.get() as usize) })
    }
}

// NonZeroIsize

impl Archive for NonZeroIsize {
    type Archived = ArchivedNonZeroIsize;
    type Resolver = ();

    #[inline]
    unsafe fn resolve(
        &self,
        _: usize,
        _: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        out.write(ArchivedNonZeroIsize::new_unchecked(self.get() as _));
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for NonZeroIsize {
    #[inline]
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<D: Fallible + ?Sized> Deserialize<NonZeroIsize, D>
    for Archived<NonZeroIsize>
{
    #[inline]
    fn deserialize(&self, _: &mut D) -> Result<NonZeroIsize, D::Error> {
        Ok(unsafe { NonZeroIsize::new_unchecked(self.get() as isize) })
    }
}

// Atomics

#[cfg(target_has_atomic = "8")]
unsafe_impl_portable!(
    core::sync::atomic::AtomicBool,
    core::sync::atomic::AtomicI8,
    core::sync::atomic::AtomicU8,
);

#[cfg(target_has_atomic = "16")]
unsafe_impl_portable!(
    rend::AtomicI16_be,
    rend::AtomicI16_le,
    rend::AtomicU16_be,
    rend::AtomicU16_le,
);

#[cfg(target_has_atomic = "32")]
unsafe_impl_portable!(
    rend::AtomicI32_be,
    rend::AtomicI32_le,
    rend::AtomicU32_be,
    rend::AtomicU32_le,
);

#[cfg(target_has_atomic = "64")]
unsafe_impl_portable!(
    rend::AtomicI64_be,
    rend::AtomicI64_le,
    rend::AtomicU64_be,
    rend::AtomicU64_le,
);
