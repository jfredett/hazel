# 11-JAN-2025

## 2322 - movegen

I think roughly the model is:


```
Type:
    name: String
    structure: Struct | Enum | Trait
    api: API*
 
API:
    name: String
    consumes: [Type]
    produces: Type

Value:
    kind: Const | LazyStatic | ...
    name: String
    value: String // the literal setting in code.

```

I'll then generate a UML 'card' that I can use as a graphic in something like `draw.io` to work out how to use them to
represent how the codebase works.

Once I know the diagram I want, I can work backwards to automate bits of it with some other tool (graphviz or
something).

I'm sure I'll eventually chase this yak to DIYtopia.

... I do wonder if I could, instead, get this data from the rust compiler itself. It must parse everything at some
point, and ostensibly I could query the compiler directly since I want to do this statically.

Indeed, docgen must do this, it's certainly using some parsed version of the crate.

Something to investigate. I do want to do some of this with treesitter anyway, but the final version might not need it.

## 2237 - movegen

With utter predictability, what I want is reflection, which doesn't exist in any easy to access form. This is a well
known issue, and it is unsurprising I want it since I have an unmanaged metaprogramming problem. So treesitter is
probably the easiest way to do this for now. Just getting the types w/o any API information dumped to individual files
would be a big step forward.

# 16-JAN-2025

## 1139 - movegen

I've named it `tabitha` for now, and I've got the right design in head for it.

Three parts:

engine:
    - constructs the source tree, responsible to executing queries against it, caching, etc.
    - runs `queries` against the source tree to create models
model:
    - Ruby model of a Rust codebase.
    - runs `queries` against partial parses to allow incremental building of the model.
query:
    - Queries that can be executed against some arbitrary text to produce items from the `model`


The setup will be lazy, you ask for a `struct` by doing `Struct[:NameOfStruct]` and it will automatically parse
across the tree if it hasn't already. If it finds any matches it records them in the instance, so that later it can
refresh them if they change. Eventually the main loop will watch for incoming changes and queue refreshes.

The goal should be to parse as _little_ as possible each step, and cache as much as possible along the way.

Am I reinventing the LSP wheel? Probably, but the upside is I don't have to learn how to LSP, which I think is overkill
most of the time.

For instance, `impl` blocks are a good example; a `struct` has many `impl`s, categorized by whether or not they are:

1. Trait impls
2. Bare impls
3. Generic bare imples (if the trait is generic)
    - possibly _partially_ generic, so e.g., `struct Foo<A,B>` might have an impl like `impl<A> Foo<A, i32>`
4. Generic Trait generic impls
    - possibly partial
5. Generic Trait impls with associated types

Each of these impls contain a bunch of `fn`s, which are themselves categorized by whether they are:

1. Plain
2. Generic
3. async
4. unsafe
5. const

and so on.

I want to record information about file location, content, references, etc -- and ideally do so pretty generically. I
also want to model the module system, but I suspect that won't be entirely tree-sittable.
