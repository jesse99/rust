// xfail-fast
// aux-build:cci_class_trait.rs
extern mod cci_class_trait;
use cci_class_trait::animals::*;

struct cat {
  priv mut meows : uint,

  mut how_hungry : int,
  name : ~str,
}

impl cat {
  fn eat() -> bool {
    if self.how_hungry > 0 {
        error!("OM NOM NOM");
        self.how_hungry -= 2;
        return true;
    }
    else {
        error!("Not hungry!");
        return false;
    }
  }
}

impl cat : noisy {

  fn speak() { self.meow(); }

}

priv impl cat {
    fn meow() {
      error!("Meow");
      self.meows += 1u;
      if self.meows % 5u == 0u {
          self.how_hungry += 1;
      }
    }
}

fn cat(in_x : uint, in_y : int, in_name: ~str) -> cat {
    cat {
        meows: in_x,
        how_hungry: in_y,
        name: in_name
    }
}


fn main() {
  let nyan = cat(0u, 2, ~"nyan");
  nyan.eat();
  assert(!nyan.eat());
  for uint::range(1u, 10u) |_i| { nyan.speak(); };
  assert(nyan.eat());
}