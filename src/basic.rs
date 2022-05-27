use crate::{Dog, Field};

trait Project<Index> {
    type Value;

    fn proj(&self) -> &Self::Value;
}

trait ProjectMut<Index>: Project<Index> {
    fn proj_mut(&mut self) -> &mut Self::Value;
}

////
// Concrete Implementations

impl Project<Field<"height">> for Dog {
    type Value = f32;

    fn proj(&self) -> &Self::Value {
        &self.height
    }
}

impl ProjectMut<Field<"height">> for Dog {
    fn proj_mut(&mut self) -> &mut Self::Value {
        &mut self.height
    }
}

impl Project<Field<"age">> for Dog {
    type Value = u32;

    fn proj(&self) -> &Self::Value {
        &self.age
    }
}

impl ProjectMut<Field<"age">> for Dog {
    fn proj_mut(&mut self) -> &mut Self::Value {
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
    type Value = (A, B);

    fn proj(&self) -> &Self::Value {
        // INTERFACE PROBLEM:
        // - Impossible to implement
        panic!()
    }
}

impl<A, B, T> ProjectMut<(A, B)> for T
where
    T: Project<A>,
    T: Project<B>,
{
    fn proj_mut(&mut self) -> &mut Self::Value {
        // INTERFACE PROBLEM:
        // - Impossible to implement
        panic!()
    }
}
