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
