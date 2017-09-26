

use immutable_map::TreeMap;
use typed_arena::Arena;

use std::rc::Rc;

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
    Pgm(&'a List<&'a str>, &'a KItem<'a>),
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
    match c {
        Cfg {k: Cons(&AVar(i), rest), state, allocator}=> {
            if state.contains_key(&i) {
                let x = allocator.alloc(ACon(*state.get(&i).unwrap()));
                Ok(Cfg{k: Cons(x, rest), state: state, allocator})
            } else {
                let x = allocator.alloc(ACon(-1));
                Err(Cfg{k: Cons(x, rest), state, allocator})
            }
        },  
        Cfg {k: Cons(&Div(&ACon(i), &ACon(j)), rest), state, allocator}=> {
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
        Cfg {k: Cons(&Add(&ACon(i), &ACon(j)), rest), state, allocator}=> {
            let x = allocator.alloc(ACon(i + j));
            Ok(Cfg{k: Cons(x, rest), state, allocator})
        },
        Cfg {k: Cons(&Le(&ACon(i), &ACon(j)), rest), state, allocator}=> {
            let x = allocator.alloc(BCon(i <= j));
            Ok(Cfg{k: Cons(x, rest), state, allocator})
        },
        Cfg {k: Cons(&Not(&BCon(b)), rest), state, allocator}=> {
            let x = allocator.alloc(BCon(!b));
            Ok(Cfg{k: Cons(x, rest), state, allocator})
        },
        Cfg {k: Cons(&And(&BCon(true), b), rest), state, allocator}=> {
            Ok(Cfg{k: Cons(b, rest), state, allocator})
        },
        Cfg {k: Cons(&And(&BCon(false), _), rest), state, allocator}=> {
            let x = allocator.alloc(BCon(false));
            Ok(Cfg{k: Cons(x, rest), state, allocator})
        },
        Cfg {k: Cons(&Assign(i, &ACon(j)), rest), state, allocator}=> {
            Ok(Cfg{k: *rest, state: state.insert(i, j), allocator})
        },
        Cfg {k: Cons(&Seq(s1, s2), rest), state, allocator}=> {
            Ok(Cfg{k: Cons(s1, Box::new(Cons(s2, rest))), state, allocator})
        },
        Cfg {k: Cons(&Skip, rest), state, allocator}=> {
            Ok(Cfg{k: *rest, state, allocator})
        },
        Cfg {k: Cons(&If(&BCon(true), s, _), rest), state, allocator}=> {
            Ok(Cfg{k: Cons(s, rest), state, allocator})
        },
        Cfg {k: Cons(&If(&BCon(false), _, s), rest), state, allocator}=> {
            Ok(Cfg{k: Cons(s, rest), state, allocator})
        },
        Cfg {k: Cons(&While(b, s), rest), state, allocator}=> {
            let while_ = allocator.alloc(While(b, s));
            let skip_ = allocator.alloc(Skip);
            let seq_ = allocator.alloc(Seq(s, while_));
            let if_ = allocator.alloc(If(b, seq_, skip_));
            Ok(Cfg{k: Cons(if_, rest), state, allocator})            
        },
        /*
        Cfg {k: Cons(&Pgm(args, s), box Nil), state, allocator}=> {
            // let pgm = allocator.alloc(Pgm(&xs, s));
            // Ok(Cfg{k: Cons(pgm, Box::new(Nil)), state: state.insert(i, 0), allocator})
            match args {
                &Cons(i, box xs)=> {
                    let pgm = allocator.alloc(Pgm(&List::Nil, s));
                    Ok(Cfg{k: Cons(pgm, Box::new(Nil)), state: state.insert(i, 0), allocator})
                },
                _ => Ok(Cfg{k: Cons(s, Box::new(Nil)), state, allocator})
            }
        },

        Cfg {k: Cons(&Pgm(Nil, s), box Nil), state, allocator}=> {
            Ok(Cfg{k: Cons(s, Box::new(Nil)), state, allocator})
        },
        */
        // Heading/cooling rules 
        // Heating 
        Cfg {k: Cons(&Div(e1, e2), rest), state, allocator}=> {
            if !a_result(e1) {
                Ok(Cfg{k: Cons(e1, Box::new(Cons(allocator.alloc(DivL(e2)), rest))) , state, allocator})
            } else if !a_result(e2) {
                Ok(Cfg{k: Cons(e2, Box::new(Cons(allocator.alloc(DivR(e1)), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(allocator.alloc(Div(e1, e2)), rest), state, allocator})
            }
        },
        Cfg {k: Cons(Add(&e1, &e2), rest), state, allocator}=> {
            if !a_result(&e1) {
                Ok(Cfg{k: Cons(e1, Box::new(Cons(AddL(&e2), rest))) , state, allocator})
            } else if !a_result(&e2) {
                Ok(Cfg{k: Cons(e2, Box::new(Cons(AddR(&e1), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(Add(&e1, &e2), rest), state, allocator})
            }
        },
        /*
        Cfg {k: Cons(Le(&e1, &e2), rest), state, allocator}=> {
            if !a_result(&e1) {
                Ok(Cfg{k: Cons(e1, Box::new(Cons(LeL(&e2), rest))) , state, allocator})
            } else if !a_result(&e2) {
                Ok(Cfg{k: Cons(e2, Box::new(Cons(LeR(&e1), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(Le(&e1, &e2), rest), state, allocator})
            }
        },
        Cfg {k: Cons(Not(&b), rest), state, allocator}=> {
            if !b_result(&b) {
                Ok(Cfg{k: Cons(b, Box::new(Cons(NotF, rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(Not(&b), rest), state, allocator})
            }
        },
        Cfg {k: Cons(And(&b1, &b2), rest), state, allocator}=> {
            if !b_result(&b1) {
                Ok(Cfg{k: Cons(b1, Box::new(Cons(AndL(&b2), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(And(&b1, &b2), rest), state, allocator})
            }
        },
        Cfg {k: Cons(Assign(i, &e), rest), state, allocator}=> {
            if !a_result(&e) {
                Ok(Cfg{k: Cons(e, Box::new(Cons(AssignR(i), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(Assign(i, &e), rest), state, allocator})
            }
        },
        Cfg {k: Cons(If(&b, s1, s2), rest), state, allocator}=> {
            if !b_result(&b) {
                Ok(Cfg{k: Cons(b, Box::new(Cons(IfC(s1, s2), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(If(&b, s1, s2), rest), state, allocator})
            }
        },
        // Cooling
        // https://stackoverflow.com/questions/28638757/use-of-collaterally-moved-value-error-on-a-recursive-enum#28639004 
        //   
        //Cfg {k: Cons(ACon(e), &Cons(DivL(e2), rest)), state, allocator}=> {
        //    Ok(Cfg{k: Cons(Div(ACon(e), e2), rest), state, allocator})
        //},
        // 
        Cfg {k: Cons(ACon(e), box more), state, allocator}=> {
            match more {
                Cons(DivL(e2), rest)=> {
                    Ok(Cfg{k: Cons(Div(&ACon(e), e2), rest), state, allocator})
                },
                Cons(DivR(e1), rest)=> {
                    Ok(Cfg{k: Cons(Div(e1, &ACon(e)), rest), state, allocator})
                },
                Cons(AddL(e2), rest)=> {
                    Ok(Cfg{k: Cons(Add(&ACon(e), e2), rest), state, allocator})
                },
                Cons(AddR(e1), rest)=> {
                    Ok(Cfg{k: Cons(Add(e1, &ACon(e)), rest), state, allocator})
                },
                Cons(LeL(e2), rest)=> {
                    Ok(Cfg{k: Cons(Le(&ACon(e), e2), rest), state, allocator})
                },
                Cons(LeR(e1), rest)=> {
                    Ok(Cfg{k: Cons(Le(e1, &ACon(e)), rest), state, allocator})
                },
                Cons(AssignR(i), rest)=> {
                    Ok(Cfg{k: Cons(Assign(i, &ACon(e)), rest), state, allocator})
                },
                _ => Err(Cfg{k: Cons(ACon(e), Box::new(more)), state, allocator})
            }
        },
        Cfg {k: Cons(BCon(e), box more), state, allocator}=> {
            match more {
                Cons(NotF, rest) => {
                    Ok(Cfg{k: Cons(Not(&BCon(e)), rest), state, allocator})
                },
                Cons(AndL(e2), rest) => {
                    Ok(Cfg{k: Cons(And(&BCon(e), e2), rest), state, allocator})
                },
                Cons(IfC(s1, s2), rest) => {
                    Ok(Cfg{k: Cons(If(&BCon(e), s1, s2), rest), state, allocator})
                },
                _ => Err(Cfg{k: Cons(BCon(e), Box::new(more)), state, allocator})

            }
        },
        */
        _ => Err(Cfg{k: c.k, state: c.state, allocator: c.allocator})
    }
    /*
    match c {
        Cfg {k: Cons(AVar(i), rest), state, allocator}=> {
            if state.contains_key(&i) {
                Ok(Cfg{k: Cons(ACon(*state.get(&i).unwrap()), rest), state: state, allocator})
            } else {
                Err(Cfg{k: Cons(ACon(*state.get(&i).unwrap()), rest), state, allocator})
            }
        },  
        Cfg {k: Cons(Div(&ACon(i), &ACon(j)), rest), state, allocator}=> {
            if j == 0 {
                let x = allocator.alloc(ACon(i));
                let y = allocator.alloc(ACon(j));
                Err(Cfg{k: Cons(Div(x, y), rest), state, allocator})
            } else {
                Ok(Cfg{k: Cons(ACon(i / j), rest), state, allocator})
            }
        },
        Cfg {k: Cons(Add(&ACon(i), &ACon(j)), rest), state, allocator}=> {
            Ok(Cfg{k: Cons(ACon(i + j), rest), state, allocator})
        },
        Cfg {k: Cons(Le(&ACon(i), &ACon(j)), rest), state, allocator}=> {
            Ok(Cfg{k: Cons(BCon(i <= j), rest), state, allocator})
        },
        Cfg {k: Cons(Not(&BCon(b)), rest), state, allocator}=> {
            Ok(Cfg{k: Cons(BCon(!b), rest), state, allocator})
        },
        Cfg {k: Cons(And(&BCon(true), &b), rest), state, allocator}=> {
            Ok(Cfg{k: Cons(b, rest), state, allocator})
        },
        Cfg {k: Cons(And(&BCon(false), _), rest), state, allocator}=> {
            Ok(Cfg{k: Cons(BCon(false), rest), state, allocator})
        },
        Cfg {k: Cons(Assign(i, &ACon(j)), rest), state, allocator}=> {
            Ok(Cfg{k: *rest, state: state.insert(i, j), allocator})
        },
        Cfg {k: Cons(Seq(&s1, &s2), rest), state, allocator}=> {
            Ok(Cfg{k: Cons(s1, Box::new(Cons(s2, rest))), state, allocator})
        },
        Cfg {k: Cons(Skip, rest), state, allocator}=> {
            Ok(Cfg{k: *rest, state, allocator})
        },
        Cfg {k: Cons(If(&BCon(true), &s, _), rest), state, allocator}=> {
            Ok(Cfg{k: Cons(s, rest), state, allocator})
        },
        Cfg {k: Cons(If(&BCon(false), _, &s), rest), state, allocator}=> {
            Ok(Cfg{k: Cons(s, rest), state, allocator})
        },
        Cfg {k: Cons(While(b, s), rest), state, allocator}=> {
            Ok(Cfg{k: Cons(If(b, &Seq(s, &While(&b, &s)), &Skip), rest), state, allocator})            
        },
        Cfg {k: Cons(Pgm(Cons(i, xs), s), box Nil), state, allocator}=> {
            Ok(Cfg{k: Cons(Pgm(*xs, s), Box::new(Nil)), state: state.insert(i, 0), allocator})
        },
        Cfg {k: Cons(Pgm(Nil, &s), box Nil), state, allocator}=> {
            Ok(Cfg{k: Cons(s, Box::new(Nil)), state, allocator})
        },
        // Heading/cooling rules 
        // Heating 
        Cfg {k: Cons(Div(&e1, &e2), rest), state, allocator}=> {
            if !a_result(&e1) {
                Ok(Cfg{k: Cons(e1, Box::new(Cons(DivL(&e2), rest))) , state, allocator})
            } else if !a_result(&e2) {
                Ok(Cfg{k: Cons(e2, Box::new(Cons(DivR(&e1), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(Div(&e1, &e2), rest), state, allocator})
            }
        },
        Cfg {k: Cons(Add(&e1, &e2), rest), state, allocator}=> {
            if !a_result(&e1) {
                Ok(Cfg{k: Cons(e1, Box::new(Cons(AddL(&e2), rest))) , state, allocator})
            } else if !a_result(&e2) {
                Ok(Cfg{k: Cons(e2, Box::new(Cons(AddR(&e1), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(Add(&e1, &e2), rest), state, allocator})
            }
        },
        Cfg {k: Cons(Le(&e1, &e2), rest), state, allocator}=> {
            if !a_result(&e1) {
                Ok(Cfg{k: Cons(e1, Box::new(Cons(LeL(&e2), rest))) , state, allocator})
            } else if !a_result(&e2) {
                Ok(Cfg{k: Cons(e2, Box::new(Cons(LeR(&e1), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(Le(&e1, &e2), rest), state, allocator})
            }
        },
        Cfg {k: Cons(Not(&b), rest), state, allocator}=> {
            if !b_result(&b) {
                Ok(Cfg{k: Cons(b, Box::new(Cons(NotF, rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(Not(&b), rest), state, allocator})
            }
        },
        Cfg {k: Cons(And(&b1, &b2), rest), state, allocator}=> {
            if !b_result(&b1) {
                Ok(Cfg{k: Cons(b1, Box::new(Cons(AndL(&b2), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(And(&b1, &b2), rest), state, allocator})
            }
        },
        Cfg {k: Cons(Assign(i, &e), rest), state, allocator}=> {
            if !a_result(&e) {
                Ok(Cfg{k: Cons(e, Box::new(Cons(AssignR(i), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(Assign(i, &e), rest), state, allocator})
            }
        },
        Cfg {k: Cons(If(&b, s1, s2), rest), state, allocator}=> {
            if !b_result(&b) {
                Ok(Cfg{k: Cons(b, Box::new(Cons(IfC(s1, s2), rest))) , state, allocator})
            } else {
                Err(Cfg{k: Cons(If(&b, s1, s2), rest), state, allocator})
            }
        },
        // Cooling
        // https://stackoverflow.com/questions/28638757/use-of-collaterally-moved-value-error-on-a-recursive-enum#28639004 
        /*   
        Cfg {k: Cons(ACon(e), &Cons(DivL(e2), rest)), state, allocator}=> {
            Ok(Cfg{k: Cons(Div(ACon(e), e2), rest), state, allocator})
        },
        */
        Cfg {k: Cons(ACon(e), box more), state, allocator}=> {
            match more {
                Cons(DivL(e2), rest)=> {
                    Ok(Cfg{k: Cons(Div(&ACon(e), e2), rest), state, allocator})
                },
                Cons(DivR(e1), rest)=> {
                    Ok(Cfg{k: Cons(Div(e1, &ACon(e)), rest), state, allocator})
                },
                Cons(AddL(e2), rest)=> {
                    Ok(Cfg{k: Cons(Add(&ACon(e), e2), rest), state, allocator})
                },
                Cons(AddR(e1), rest)=> {
                    Ok(Cfg{k: Cons(Add(e1, &ACon(e)), rest), state, allocator})
                },
                Cons(LeL(e2), rest)=> {
                    Ok(Cfg{k: Cons(Le(&ACon(e), e2), rest), state, allocator})
                },
                Cons(LeR(e1), rest)=> {
                    Ok(Cfg{k: Cons(Le(e1, &ACon(e)), rest), state, allocator})
                },
                Cons(AssignR(i), rest)=> {
                    Ok(Cfg{k: Cons(Assign(i, &ACon(e)), rest), state, allocator})
                },
                _ => Err(Cfg{k: Cons(ACon(e), Box::new(more)), state, allocator})
            }
        },
        Cfg {k: Cons(BCon(e), box more), state, allocator}=> {
            match more {
                Cons(NotF, rest) => {
                    Ok(Cfg{k: Cons(Not(&BCon(e)), rest), state, allocator})
                },
                Cons(AndL(e2), rest) => {
                    Ok(Cfg{k: Cons(And(&BCon(e), e2), rest), state, allocator})
                },
                Cons(IfC(s1, s2), rest) => {
                    Ok(Cfg{k: Cons(If(&BCon(e), s1, s2), rest), state, allocator})
                },
                _ => Err(Cfg{k: Cons(BCon(e), Box::new(more)), state, allocator})

            }
        }
        _ => Err(Cfg{k: c.k, state: c.state, allocator: c.allocator})
    }
    */
}

pub fn sum_pgm<'a>(size: i64)-> KItem<'a> {
    let n = "n";
    let sum = "sum";
    let args = Cons(n, Box::new(Cons(sum, Box::new(Nil))));
    
    let allocator: Arena<KItem> = Arena::new();
    let a = allocator.alloc(ACon(1));
    let b = Box::new(ACon(1));

    /*
    Pgm(args, 
        Box::new(Seq(
            Box::new(Assign(n, Box::new(ACon(size)))),
        Box::new(Seq(
            Box::new(Assign(sum, Box::new(ACon(0)))),
            Box::new(While(Box::new(Not(Box::new(Le(Box::new(AVar(n)), Box::new(ACon(0)))))),
                        Box::new(Seq(
                            Box::new(Assign(sum, Box::new(Add(Box::new(AVar(sum)), Box::new(AVar(n)))))),
                            Box::new(Assign(n, Box::new(Add(Box::new(AVar(n)), Box::new(ACon(-1))))))
                        )))))))))
                        */
    ACon(1)
}

/*
pub fn start(p: KItem)->Cfg {
    Cfg {
        k: Cons(p, Box::new(Nil)),
        state: TreeMap::new()
    }
}
*/

pub fn run(mut c:Cfg) {
    loop {
        let r = step(c);
        match r {
            Ok(c_)=> {
                c = c_;
                continue;
            },
            Err(c)=> {
                // println!("Done {:?}", c);
                break;
            }
        }
    }
}