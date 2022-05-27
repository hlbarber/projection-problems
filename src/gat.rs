use crate::{Dog, Field};

trait Project<Index> {
    type Value<'a>
    where
        Self: 'a;

    fn proj(&self) -> Self::Value<'_>;
}

trait ProjectMut<Index>: Project<Index> {
    // TYPE PROBLEM:
    // - This is decoupled from Self::Value
    type ValueMut<'a>
    where
        Self: 'a;

    fn proj_mut(&mut self) -> Self::ValueMut<'_>;
}

////
// Concrete Implementations

impl Project<Field<"height">> for Dog {
    type Value<'a> = &'a f32;

    fn proj(&self) -> Self::Value<'_> {
        &self.height
    }
}

impl ProjectMut<Field<"height">> for Dog {
    type ValueMut<'a> = &'a mut f32;

    fn proj_mut(&mut self) -> Self::ValueMut<'_> {
        &mut self.height
    }
}

impl Project<Field<"age">> for Dog {
    type Value<'a> = &'a u32;

    fn proj(&self) -> Self::Value<'_> {
        &self.age
    }
}

impl ProjectMut<Field<"age">> for Dog {
    type ValueMut<'a> = &'a mut u32;

    fn proj_mut(&mut self) -> Self::ValueMut<'_> {
        &mut self.age
    }
}

////
// Blanket implementations

impl<A, B, T> Project<(A, B)> for T
where
    T: Project<A>,
    T: Project<B>,
{
    type Value<'a> = (<T as Project<A>>::Value<'a>, <T as Project<B>>::Value<'a>) where T: 'a;

    fn proj(&self) -> Self::Value<'_> {
        (
            <Self as Project<A>>::proj(self),
            <Self as Project<B>>::proj(self),
        )
    }
}

impl<A, B, T> ProjectMut<(A, B)> for T
where
    T: ProjectMut<A>,
    T: ProjectMut<B>,
{
    type ValueMut<'a> = (<T as ProjectMut<A>>::ValueMut<'a>, <T as ProjectMut<B>>::ValueMut<'a>) where T: 'a;

    fn proj_mut(&mut self) -> Self::ValueMut<'_> {
        // TYPE PROBLEM:
        // - Lack of view types
        todo!()
    }
}
