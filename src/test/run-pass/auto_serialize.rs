extern mod std;

// These tests used to be separate files, but I wanted to refactor all
// the common code.

use cmp::Eq;
use std::ebml;
use io::Writer;
use std::serialization::{Serializable, Deserializable, deserialize};
use std::prettyprint;

fn test_prettyprint<A: Serializable<prettyprint::Serializer>>(
    a: &A,
    expected: &~str
) {
    let s = do io::with_str_writer |w| {
        a.serialize(&prettyprint::Serializer(w))
    };
    debug!("s == %?", s);
    assert s == *expected;
}

fn test_ebml<A:
    Eq
    Serializable<ebml::Serializer>
    Deserializable<ebml::Deserializer>
>(a1: &A) {
    let bytes = do io::with_bytes_writer |wr| {
        let ebml_w = &ebml::Serializer(wr);
        a1.serialize(ebml_w)
    };
    let d = ebml::Doc(@bytes);
    let a2: A = deserialize(&ebml::Deserializer(d));
    assert *a1 == a2;
}

#[auto_serialize]
#[auto_deserialize]
enum Expr {
    Val(uint),
    Plus(@Expr, @Expr),
    Minus(@Expr, @Expr)
}

impl Expr : cmp::Eq {
    pure fn eq(other: &Expr) -> bool {
        match self {
            Val(e0a) => {
                match *other {
                    Val(e0b) => e0a == e0b,
                    _ => false
                }
            }
            Plus(e0a, e1a) => {
                match *other {
                    Plus(e0b, e1b) => e0a == e0b && e1a == e1b,
                    _ => false
                }
            }
            Minus(e0a, e1a) => {
                match *other {
                    Minus(e0b, e1b) => e0a == e0b && e1a == e1b,
                    _ => false
                }
            }
        }
    }
    pure fn ne(other: &Expr) -> bool { !self.eq(other) }
}

impl AnEnum : cmp::Eq {
    pure fn eq(other: &AnEnum) -> bool {
        self.v == other.v
    }
    pure fn ne(other: &AnEnum) -> bool { !self.eq(other) }
}

impl Point : cmp::Eq {
    pure fn eq(other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
    pure fn ne(other: &Point) -> bool { !self.eq(other) }
}

impl<T:cmp::Eq> Quark<T> : cmp::Eq {
    pure fn eq(other: &Quark<T>) -> bool {
        match self {
            Top(ref q) => {
                match *other {
                    Top(ref r) => q == r,
                    Bottom(_) => false
                }
            },
            Bottom(ref q) => {
                match *other {
                    Top(_) => false,
                    Bottom(ref r) => q == r
                }
            },
        }
    }
    pure fn ne(other: &Quark<T>) -> bool { !self.eq(other) }
}

impl CLike : cmp::Eq {
    pure fn eq(other: &CLike) -> bool {
        self as int == *other as int
    }
    pure fn ne(other: &CLike) -> bool { !self.eq(other) }
}

#[auto_serialize]
#[auto_deserialize]
type Spanned<T> = {lo: uint, hi: uint, node: T};

impl<T:cmp::Eq> Spanned<T> : cmp::Eq {
    pure fn eq(other: &Spanned<T>) -> bool {
        self.lo == other.lo && self.hi == other.hi && self.node == other.node
    }
    pure fn ne(other: &Spanned<T>) -> bool { !self.eq(other) }
}

#[auto_serialize]
#[auto_deserialize]
type SomeRec = {v: ~[uint]};

#[auto_serialize]
#[auto_deserialize]
enum AnEnum = SomeRec;

#[auto_serialize]
#[auto_deserialize]
struct Point {x: uint, y: uint}

#[auto_serialize]
#[auto_deserialize]
enum Quark<T> {
    Top(T),
    Bottom(T)
}

#[auto_serialize]
#[auto_deserialize]
enum CLike { A, B, C }

fn main() {
    let a = &Plus(@Minus(@Val(3u), @Val(10u)), @Plus(@Val(22u), @Val(5u)));
    test_prettyprint(a, &~"Plus(@Minus(@Val(3u), @Val(10u)), \
                           @Plus(@Val(22u), @Val(5u)))");
    test_ebml(a);

    let a = &{lo: 0u, hi: 5u, node: 22u};
    test_prettyprint(a, &~"{lo: 0u, hi: 5u, node: 22u}");
    test_ebml(a);

    let a = &AnEnum({v: ~[1u, 2u, 3u]});
    test_prettyprint(a, &~"AnEnum({v: ~[1u, 2u, 3u]})");
    test_ebml(a);

    let a = &Point {x: 3u, y: 5u};
    test_prettyprint(a, &~"Point {x: 3u, y: 5u}");
    test_ebml(a);

    let a = &@[1u, 2u, 3u];
    test_prettyprint(a, &~"@[1u, 2u, 3u]");
    test_ebml(a);

    let a = &Top(22u);
    test_prettyprint(a, &~"Top(22u)");
    test_ebml(a);

    let a = &Bottom(222u);
    test_prettyprint(a, &~"Bottom(222u)");
    test_ebml(a);

    let a = &A;
    test_prettyprint(a, &~"A");
    test_ebml(a);

    let a = &B;
    test_prettyprint(a, &~"B");
    test_ebml(a);
}
