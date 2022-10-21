// We don't actually use the structures below, just document them, which makes the compiler a bit
// unhappy.
#![allow(dead_code)]
use serde_derive::Deserialize;
use structdoc::{Documentation, StructDoc};

#[derive(StructDoc)]
struct A<T: Default = ()>(T);

#[derive(StructDoc, Deserialize)]
#[structdoc(rename_all = "SCREAMING-KEBAB-CASE")]
struct Stuff<T>
where
    T: Clone,
{
    /// How many times to say hello
    #[structdoc(rename = "hello")]
    #[serde(default)]
    a: i32,

    /// Some extra chatter.
    ///
    /// Appended to the stuff.
    #[serde(default)]
    b: Vec<String>,

    #[structdoc(skip, leaf)]
    und: Undocumented,

    /// Select one of these, please
    selection: Selection,

    #[serde(flatten)]
    sub: T,

    #[structdoc(with = "gen_doc")]
    flag: bool,
}

#[derive(Deserialize)]
enum Undocumented {
    A,
}

#[derive(Deserialize, StructDoc)]
#[serde(tag = "type", rename_all = "camelCase")]
enum Selection {
    A {
        a: i32,
        b: i32,
    },
    X {
        x: String,
    },
    #[structdoc(leaf)]
    AnotherThing(Undocumented),
    FooBar(String),
    #[serde(skip)]
    Other,
}

fn gen_doc() -> Documentation {
    Documentation::leaf("Manual implementation")
}

#[derive(Clone, StructDoc, Deserialize)]
struct Inner {
    /// A bool
    c: bool,
}

#[derive(Clone, StructDoc, Deserialize)]
struct Simple {
    /// An inner field
    inner: Inner,
    /// An integer
    integer: usize,
    /// An optional type
    r#type: Option<SimpleExternalEnum>,
    /// A vec of untagged
    untagged: Vec<SimpleUntaggedEnum>,
    /// A box of untagged
    untaggeds: Box<UntaggedEnum>,
}

#[derive(Clone, StructDoc, Deserialize)]
enum SimpleExternalEnum {
    One,
    Two,
    Three,
}

#[derive(Clone, StructDoc, Deserialize)]
#[serde(untagged)]
enum SimpleUntaggedEnum {
    /// One
    #[serde(rename = "one")]
    One,
    /// Two
    Two,
    /// Three
    Three,
}

#[derive(Clone, StructDoc, Deserialize)]
#[serde(untagged)]
enum UntaggedEnum {
    /// An inner value
    One(Inner),
    /// A bool
    Two(bool),
    /// A recursive thing
    Three(Simple),
    /// A tuple
    Four((bool, usize, String, SimpleUntaggedEnum)),
}

#[derive(Clone, StructDoc, Deserialize)]
struct Rec {
    v: Box<Rec>,
}

fn main() {
    let documentation = Stuff::<Inner>::document();
    println!("{:?}", documentation);
    println!("{}", documentation);

    //println!("{}", Rec::document());
    //println!("{}", Simple::document());
    println!("{}", Simple::document().markdown());
}
