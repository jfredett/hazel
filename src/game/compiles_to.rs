
/*


Sketchy, but here's the idea.

Every `CompilesTo` type has some implementation of this (perhaps generic in the target rep, instead of a trait object)

A log contains a list of items which compile to the same target rep, and then can cache that compilation and seek through it dynamically.
The assumption is always that every action can be 'undone' -- either trivially by rerunning, or incremeentally via inverses.

Then I can layer caches per representation level.

CompilesTo is a kind of 'Context-Embedding' operation. Whatever context is necessary must be provided, then the target representation should stand without further context.

*/

pub trait CompilesTo<R> {
    type Context = ();

    fn compile(&self, context: &Self::Context) -> R;
}




/*
* In principle this is transitive, but I think Context makes it tricky.
*
* Say you have some #compile/T (with T as the Context Type) to take A -> B
* and similarly you have a B -> C with S as the context type, then in theory:
*
* A.compile(context: &T).compile(context: &S) -> Vec<C>
*
* should always exist, so the implementation would be:

impl<A, B, C> CompilesTo<C> for A where B: CompilesTo<C>, A: CompilesTo<B>, A != B != C {
    // I'm sure this is wrong, I need to know about the sum of each implementation's context. So if
    //
    // A --> B, under context T
    // B --> C, under context S
    //
    // Then A --> C under context (T, S)
    type Context = (A::Context, B::Context);

    fn compile(&self, context: &Self::Context) -> C {
        let (a, b) = context;
        self.compile(a).compile(b)
    }
}

* but this doesn't work because there is no sense in which I can declare `A,B,C` distinct.
* It's possible there is a way to express this, but I'm not sure how to do it.
*
* Instead, there is this function:
*/
pub fn transitive_compile<A, B, C, T, S>(a: &A, context: &(T, S)) -> C where A: CompilesTo<B, Context=T>, B: CompilesTo<C, Context=S> {
    let (t, s) = context;
    a.compile(t).compile(s)
}
/* 
* This unfortunately requires you to specify everything, including the intermediate type and the
* context types; which would otherwise be calculated for you. But it does allow you to transitively
* compile along a type chain.
*
* The ideal generic implementation version would mean that an `fn foo(bar: &impl CompilesTo<C>)` would
* be able to take any type which can compile to C, regardless of the intermediate types (I think),
* meaning that the context would be inferred from the type signature. There is one wrinkle, easily
* solved with another theoretical generic impl like "If A -> B under T, then Vec<A> -> B under T.
* by compiling each A, collecting, and flattening"
* This constrains the context to be the same across all of the types in the vec, which means that
* if the moves trigger state change that will effect the subsequent compilation, it will not be
* able to compile because of the hidden gamestate change between compilations. So this generic impl
* is more like a fold with an additional operation to update the metadata. For Hazel, I don't want
* to get that generic (yet).
*
* The transitive compile should be enough to get the nested language thing working, so that's what
* I'm going for.
*/

