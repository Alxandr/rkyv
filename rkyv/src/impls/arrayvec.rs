use arrayvec::ArrayVec;
use rancor::Fallible;

use crate::{
    ser::{Allocator, Writer},
    vec::{ArchivedVec, VecResolver},
    Archive, Archived, Deserialize, Serialize,
};

impl<T, const CAP: usize> Archive for ArrayVec<T, CAP>
where
    T: Archive,
{
    type Archived = ArchivedVec<Archived<T>>;
    type Resolver = VecResolver;

    #[inline]
    unsafe fn resolve(
        &self,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedVec::resolve_from_slice(self.as_slice(), pos, resolver, out);
    }
}

impl<T, S, const CAP: usize> Serialize<S> for ArrayVec<T, CAP>
where
    T: Serialize<S>,
    S: Fallible + Allocator + Writer + ?Sized,
{
    #[inline]
    fn serialize(
        &self,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_slice(self.as_slice(), serializer)
    }
}

impl<T, D, const CAP: usize> Deserialize<ArrayVec<T, CAP>, D>
    for ArchivedVec<Archived<T>>
where
    T: Archive,
    Archived<T>: Deserialize<T, D>,
    D: Fallible + ?Sized,
{
    #[inline]
    fn deserialize(
        &self,
        deserializer: &mut D,
    ) -> Result<ArrayVec<T, CAP>, D::Error> {
        let mut result = ArrayVec::new();
        for item in self.as_slice() {
            result.push(item.deserialize(deserializer)?);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use arrayvec::ArrayVec;
    use rancor::{Failure, Infallible};

    use crate::{
        access_unchecked, deserialize, ser::Positional as _, vec::ArchivedVec,
        Archived,
    };

    #[test]
    fn array_vec() {
        use crate::ser::CoreSerializer;

        let value: ArrayVec<i32, 4> = ArrayVec::from([10, 20, 40, 80]);

        let serializer = crate::util::serialize_into::<_, _, Failure>(
            &value,
            CoreSerializer::<256, 256>::default(),
        )
        .unwrap();
        let end = serializer.pos();
        let result = serializer.into_writer().into_inner();
        let archived = unsafe {
            access_unchecked::<ArchivedVec<Archived<i32>>>(&result[0..end])
        };
        assert_eq!(archived.as_slice(), &[10, 20, 40, 80]);

        let deserialized =
            deserialize::<ArrayVec<i32, 4>, _, Infallible>(archived, &mut ())
                .unwrap();
        assert_eq!(value, deserialized);
    }
}
