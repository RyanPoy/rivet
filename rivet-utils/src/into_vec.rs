pub trait IntoVec<T> {
    fn to_vec(self) -> Vec<T>;
}

impl<T, I> IntoVec<T> for Vec<I>
where
    I: Into<T>,
{
    fn to_vec(self) -> Vec<T> {
        self.into_iter().map(Into::into).collect()
    }
}

impl<T, I, const N: usize> IntoVec<T> for [I; N]
where
    I: Into<T>,
{
    fn to_vec(self) -> Vec<T> {
        self.into_iter().map(Into::into).collect()
    }
}

#[macro_export]
macro_rules! impl_into_vec_for {
    (
        $(
            $key:ident => [ $($val:ty),* $(,)? ]
        ),* $(,)?
    ) => {
        $(
            $(
                impl rivet_utils::into_vec::IntoVec<$key> for $val
                where
                    $val: Into<$key>,
                {
                    fn to_vec(self) -> Vec<$key> {
                        vec![self.into()]
                    }
                }
            )*
        )*
    };
}
