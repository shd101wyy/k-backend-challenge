

use immutable_map::TreeMap;
use typed_arena::Arena;
use std::time::{SystemTime};


// ================================
// ========  Unoptimized  =========
// ================================
#[derive(Clone, Debug)]
pub enum KItem<'a> {
    ACon(i64),
    AVar(&'a str),
    Div(&'a KItem<'a>, &'a KItem<'a>),
    Add(&'a KItem<'a>, &'a KItem<'a>),
    BCon(bool),
    Le(&'a KItem<'a>, &'a KItem<'a>),
    Not(&'a KItem<'a>),
    And(&'a KItem<'a>, &'a KItem<'a>),
    Assign(&'a str, &'a KItem<'a>),
    If(&'a KItem<'a>, &'a KItem<'a>, &'a KItem<'a>),
    While(&'a KItem<'a>, &'a KItem<'a>),
    Seq(&'a KItem<'a>, &'a KItem<'a>),
    Skip,
    Pgm(List<&'a str>, &'a KItem<'a>),
    DivL(&'a KItem<'a>),
    DivR(&'a KItem<'a>),
    AddL(&'a KItem<'a>),
    AddR(&'a KItem<'a>),
    LeL(&'a KItem<'a>),
    LeR(&'a KItem<'a>),
    NotF,
    AndL(&'a KItem<'a>),
    AssignR(&'a str),
    IfC(&'a KItem<'a>, &'a KItem<'a>)
}
use self::KItem::*;

#[derive(Clone, Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}
use self::List::*;

// #[derive(Clone, Debug)]
pub struct Cfg<'a> {
    k: List<&'a KItem<'a>>,
    state: TreeMap<&'a str, i64>,
    allocator: &'a Arena<KItem<'a>>
}

fn a_result(a: &KItem)->bool {
    match *a {
        KItem::ACon(_) => true,
        _ => false
    }
}

fn b_result(a: &KItem)->bool {
    match *a {
        KItem::BCon(_) => true,
        _ => false
    }
}

#[allow(dead_code)]
fn step<'a>(c: Cfg<'a>) -> Result<Cfg<'a>, Cfg<'a>> {
    let k = c.k;
    let state = c.state;
    let allocator = c.allocator;
    match k {
        Cons(&AVar(i), rest)=> {
            if state.contains_key(&i) {
                let x = allocator.alloc(ACon(*state.get(&i).unwrap()));
                Ok(Cfg{k: Cons(x, rest), state: state, allocator})
            } else {
                let x = allocator.alloc(ACon(-1));
                Err(Cfg{k: Cons(x, rest), state, allocator})
            }
        },  
        Cons(&Div(&ACon(i), &ACon(j)), rest)=> {
            if j == 0 {
                let x = allocator.alloc(ACon(i));
                let y = allocator.alloc(ACon(j));
                let div = allocator.alloc(Div(x, y));
                Err(Cfg{k: Cons(div, rest), state, allocator})
            } else {
                let x = allocator.alloc(ACon(i / j));
                Ok(Cfg{k: Cons(x, rest), state, allocator})
            }
        },
        Cons(&Add(&ACon(i), &ACon(j)), rest)=> {
            let x = allocator.alloc(ACon(i + j));
            Ok(Cfg{k: Cons(x, rest), state, allocator})
        },
        Cons(&Le(&ACon(i), &ACon(j)), rest)=> {
            let x = allocator.alloc(BCon(i <= j));
            Ok(Cfg{k: Cons(x, rest), state, allocator})
        },
        Cons(&Not(&BCon(b)), rest)=> {
            let x = allocator.alloc(BCon(!b));
            Ok(Cfg{k: Cons(x, rest), state, allocator})
        },
        Cons(&And(&BCon(true), b), rest)=> {
            Ok(Cfg{k: Cons(b, rest), state, allocator})
        },
        Cons(&And(&BCon(false), _), rest)=> {
            let x = allocator.alloc(BCon(false));
            Ok(Cfg{k: Cons(x, rest), state, allocator})
        },
        Cons(&Assign(i, &ACon(j)), rest)=> {
            Ok(Cfg{k: *rest, state: state.insert(i, j), allocator})
        },
        Cons(&Seq(s1, s2), rest)=> {
            Ok(Cfg{k: Cons(s1, Box::new(Cons(s2, rest))), state, allocator})
        },
        Cons(&Skip, rest)=> {
            Ok(Cfg{k: *rest, state, allocator})
        },
        Cons(&If(&BCon(true), s, _), rest)=> {
            Ok(Cfg{k: Cons(s, rest), state, allocator})
        },
        Cons(&If(&BCon(false), _, s), rest)=> {
            Ok(Cfg{k: Cons(s, rest), state, allocator})
        },
        Cons(&While(b, s), rest)=> {
            let while_ = allocator.alloc(While(b, s));
            let skip_ = allocator.alloc(Skip);
            let seq_ = allocator.alloc(Seq(s, while_));
            let if_ = allocator.alloc(If(b, seq_, skip_));
            Ok(Cfg{k: Cons(if_, rest), state, allocator})            
        },
        Cons(&Pgm(ref args, s), box Nil)=> {
            let args_ = args.clone();
            match args_ {
                Cons(i, box xs)=> {
                    let pgm = allocator.alloc(Pgm(xs, s));
                    Ok(Cfg{k: Cons(pgm, Box::new(Nil)), state: state.insert(i, 0), allocator})
                },
                _ => Ok(Cfg{k: Cons(s, Box::new(Nil)), state, allocator})
            }
        },
        // Heading/cooling rules 
        // Heating 
        Cons(&Div(e1, e2), rest)=> {
            if !a_result(e1) {
                Ok(Cfg{k: Cons(e1, Box::new(Cons(allocator.alloc(DivL(e2)), rest))) , state, allocator})
            } else if !a_result(e2) {
                Ok(Cfg{k: Cons(e2, Box::new(Cons(allocator.alloc(DivR(e1)), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(allocator.alloc(Div(e1, e2)), rest), state, allocator})
            }
        },
        Cons(&Add(e1, e2), rest)=> {
            if !a_result(&e1) {
                Ok(Cfg{k: Cons(e1, Box::new(Cons(allocator.alloc(AddL(e2)), rest))) , state, allocator})
            } else if !a_result(&e2) {
                Ok(Cfg{k: Cons(e2, Box::new(Cons(allocator.alloc(AddR(e1)), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(allocator.alloc(Add(e1, e2)), rest), state, allocator})
            }
        },
        Cons(&Le(e1, e2), rest)=> {
            if !a_result(e1) {
                Ok(Cfg{k: Cons(e1, Box::new(Cons(allocator.alloc(LeL(e2)), rest))) , state, allocator})
            } else if !a_result(&e2) {
                Ok(Cfg{k: Cons(e2, Box::new(Cons(allocator.alloc(LeR(e1)), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(allocator.alloc(Le(e1, e2)), rest), state, allocator})
            }
        },
        Cons(&Not(b), rest)=> {
            if !b_result(b) {
                Ok(Cfg{k: Cons(b, Box::new(Cons(allocator.alloc(NotF), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(allocator.alloc(Not(b)), rest), state, allocator})
            }
        },
        Cons(&And(b1, b2), rest)=> {
            if !b_result(&b1) {
                Ok(Cfg{k: Cons(b1, Box::new(Cons(allocator.alloc(AndL(b2)), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(allocator.alloc(And(b1, b2)), rest), state, allocator})
            }
        },
        Cons(&Assign(i, e), rest)=> {
            if !a_result(e) {
                Ok(Cfg{k: Cons(e, Box::new(Cons(allocator.alloc(AssignR(i)), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(allocator.alloc(Assign(i, e)), rest), state, allocator})
            }
        },
        Cons(&If(b, s1, s2), rest)=> {
            if !b_result(b) {
                Ok(Cfg{k: Cons(b, Box::new(Cons(allocator.alloc(IfC(s1, s2)), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(allocator.alloc(If(b, s1, s2)), rest), state, allocator})
            }
        },
        // Cooling
        // https://stackoverflow.com/questions/28638757/use-of-collaterally-moved-value-error-on-a-recursive-enum#28639004 
        //   
        //Cfg {k: Cons(ACon(e), &Cons(DivL(e2), rest)), state, allocator}=> {
        //    Ok(Cfg{k: Cons(Div(ACon(e), e2), rest), state, allocator})
        //},
        // 
        Cons(&ACon(e), box more)=> {
            match more {
                Cons(&DivL(e2), rest)=> {
                    Ok(Cfg{k: Cons(allocator.alloc(Div(allocator.alloc(ACon(e)), e2)), rest), state, allocator})
                },
                Cons(&DivR(e1), rest)=> {
                    Ok(Cfg{k: Cons(allocator.alloc(Div(e1, allocator.alloc(ACon(e)))), rest), state, allocator})
                },
                Cons(&AddL(e2), rest)=> {
                    Ok(Cfg{k: Cons(allocator.alloc(Add(allocator.alloc(ACon(e)), e2)), rest), state, allocator})
                },
                Cons(&AddR(e1), rest)=> {
                    Ok(Cfg{k: Cons(allocator.alloc(Add(e1, allocator.alloc(ACon(e)))), rest), state, allocator})
                },
                Cons(&LeL(e2), rest)=> {
                    Ok(Cfg{k: Cons(allocator.alloc(Le(allocator.alloc(ACon(e)), e2)), rest), state, allocator})
                },
                Cons(&LeR(e1), rest)=> {
                    Ok(Cfg{k: Cons(allocator.alloc(Le(e1, allocator.alloc(ACon(e)))), rest), state, allocator})
                },
                Cons(&AssignR(i), rest)=> {
                    Ok(Cfg{k: Cons(allocator.alloc(Assign(i, allocator.alloc(ACon(e)))), rest), state, allocator})
                },
                _ => Err(Cfg{k: Cons(allocator.alloc(ACon(e)), Box::new(more)), state, allocator})
            }
        },
        Cons(&BCon(e), box more)=> {
            match more {
                Cons(&NotF, rest) => {
                    Ok(Cfg{k: Cons(allocator.alloc(Not(allocator.alloc(BCon(e)))), rest), state, allocator})
                },
                Cons(&AndL(e2), rest) => {
                    Ok(Cfg{k: Cons(allocator.alloc(And(allocator.alloc(BCon(e)), e2)), rest), state, allocator})
                },
                Cons(&IfC(s1, s2), rest) => {
                    Ok(Cfg{k: Cons(allocator.alloc(If(allocator.alloc(BCon(e)), s1, s2)), rest), state, allocator})
                },
                _ => Err(Cfg{k: Cons(allocator.alloc(BCon(e)), Box::new(more)), state, allocator})

            }
        },
        _ => Err(Cfg{k, state, allocator})
    }
}

#[allow(dead_code)]
pub fn run_pgm<'a>(size: i64) {
    let n = "n";
    let sum = "sum";
    let args = Cons(n, Box::new(Cons(sum, Box::new(Nil))));
    
    let allocator: Arena<KItem> = Arena::new();

    let pgm = allocator.alloc(Pgm(args, 
        allocator.alloc(Seq(
            allocator.alloc(Assign(n, allocator.alloc(ACon(size)))),
        allocator.alloc(Seq(
            allocator.alloc(Assign(sum, allocator.alloc(ACon(0)))),
            allocator.alloc(While(allocator.alloc(Not(allocator.alloc(Le(allocator.alloc(AVar(n)), allocator.alloc(ACon(0)))))),
                        allocator.alloc(Seq(
                            allocator.alloc(Assign(sum, allocator.alloc(Add(allocator.alloc(AVar(sum)), allocator.alloc(AVar(n)))))),
                            allocator.alloc(Assign(n, allocator.alloc(Add(allocator.alloc(AVar(n)), allocator.alloc(ACon(-1))))))
                        ))))))))));
    let mut c = Cfg {
        k: Cons(pgm, Box::new(Nil)),
        state: TreeMap::new(),
        allocator: &allocator
    };

    let timer = SystemTime::now();

    loop {
        let r = step(c);
        match r {
            Ok(c_)=> {
                c = c_;
                continue;
            },
            Err(c)=> {
                println!("Done {:?} {:?}", c.k, c.state);

                let elapsed = timer.elapsed().unwrap();
                let ms = elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000;
                println!("arena (Before freeing memory): execution time  {}ms", ms);

                break;
            }
        }
    }
}
