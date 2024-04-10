/*
    Appellation: vs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Data {
    type Elem;
}

pub trait Field<S>
where
    S: Data,
{
    type Dim;
}

pub trait VectorSpace<S>
where
    S: Data,
{
    type Field: Field<S>;
}

pub trait Subspace<S>
where
    S: Data,
{
    type Subspace: VectorSpace<S>;
}
