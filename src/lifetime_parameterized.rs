use crate::{Dog, Field};

trait Project<'a, Index> {
    type Value;

    fn proj(&'a self) -> Self::Value;
}

trait ProjectMut<'a, Index>: Project<'a, Index> {
    // TYPE PROBLEM:
    // - This is decoupled from Self::Value
    type ValueMut;

    fn proj_mut(&'a mut self) -> Self::ValueMut;
}

////
// Concrete Implementations

impl<'a> Project<'a, Field<"height">> for Dog {
    type Value = &'a f32;

    fn proj(&'a self) -> Self::Value {
        &self.height
    }
}

impl<'a> ProjectMut<'a, Field<"height">> for Dog {
    type ValueMut = &'a mut f32;

    fn proj_mut(&'a mut self) -> Self::ValueMut {
        &mut self.height
    }
}

impl<'a> Project<'a, Field<"age">> for Dog {
    type Value = &'a u32;

    fn proj(&'a self) -> Self::Value {
        &self.age
    }
}

impl<'a> ProjectMut<'a, Field<"age">> for Dog {
    type ValueMut = &'a mut u32;

    fn proj_mut(&'a mut self) -> Self::ValueMut {
        &mut self.age
    }
}

////
// Blanket implementations

impl<'a, A, B, T> Project<'a, (A, B)> for T
where
    T: Project<'a, A>,
    T: Project<'a, B>,
{
    type Value = (<T as Project<'a, A>>::Value, <T as Project<'a, B>>::Value);

    fn proj(&'a self) -> Self::Value {
        (
            <Self as Project<'a, A>>::proj(self),
            <Self as Project<'a, B>>::proj(self),
        )
    }
}

impl<'a, A, B, T> ProjectMut<'a, (A, B)> for T
where
    T: ProjectMut<'a, A>,
    T: ProjectMut<'a, B>,
{
    type ValueMut = (
        <T as ProjectMut<'a, A>>::ValueMut,
        <T as ProjectMut<'a, B>>::ValueMut,
    );

    fn proj_mut(&mut self) -> Self::ValueMut {
        // TYPE PROBLEM:
        // - Lack of view types
        todo!()
    }
}
