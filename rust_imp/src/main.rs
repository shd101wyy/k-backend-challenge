#![feature(box_patterns)]
extern crate immutable_map;

use std::time::{SystemTime};
use std::rc::Rc;
// use std::collections::HashMap;

use immutable_map::TreeMap;

#[derive(Clone, Debug)]
enum KItem<'a> {
    ACon(i64),
    AVar(&'a str),
    Div(Box<KItem<'a>>, Box<KItem<'a>>),
    Add(Box<KItem<'a>>, Box<KItem<'a>>),
    BCon(bool),
    Le(Box<KItem<'a>>, Box<KItem<'a>>),
    Not(Box<KItem<'a>>),
    And(Box<KItem<'a>>, Box<KItem<'a>>),
    Assign(&'a str, Box<KItem<'a>>),
    If(Box<KItem<'a>>, Box<KItem<'a>>, Box<KItem<'a>>),
    While(Box<KItem<'a>>, Box<KItem<'a>>),
    Seq(Box<KItem<'a>>, Box<KItem<'a>>),
    Skip,
    Pgm(List<&'a str>, Box<KItem<'a>>),
    DivL(Box<KItem<'a>>),
    DivR(Box<KItem<'a>>),
    AddL(Box<KItem<'a>>),
    AddR(Box<KItem<'a>>),
    LeL(Box<KItem<'a>>),
    LeR(Box<KItem<'a>>),
    NotF,
    AndL(Box<KItem<'a>>),
    AssignR(&'a str),
    IfC(Box<KItem<'a>>, Box<KItem<'a>>)
}
use KItem::*;

#[derive(Clone, Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}
use List::*;

#[derive(Clone, Debug)]
struct Cfg<'a> {
    k: List<KItem<'a>>,
    state: TreeMap<&'a str, i64>
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

/*
#[derive(Clone, Debug)]
enum KState {
    Continue,
    Stuck
}

#[derive(Clone, Debug)]
struct Cfg_mut<'a> {
    k: List<KItem<'a>>,
    state: TreeMap<&'a str, i64>,
    k_state: KState
}
*/

// #[allow(dead_code)]
//fn step_mut(mut c: Cfg_mut) -> Cfg_mut {
    /*
    match c {
        Cfg_mut {k: Cons(AVar(i), rest), state, k_state:_}=> {
            if state.contains_key(&i) {
                c.k = Cons(ACon(*state.get(&i).unwrap()), rest.clone());
                c
            } else {
                c.k_state = KState::Stuck;
                c
            }
        },
        _ => c
    }
    */
    /*
    match c {
        Cfg_mut {k: Cons(AVar(i), rest), state, k_state:_}=> {
            if state.contains_key(&i) {
                c.k = Cons(ACon(*state.get(&i).unwrap()), rest);
            } else {
                c.k_state = KState::Stuck;
            }
        },
        Cfg_mut {k: Cons(Div(box ACon(i), box ACon(j)), rest), state:_, k_state:_}=> {
            if j == 0 {
                c.k_state = KState::Stuck;
            } else {
                c.k = Cons(ACon(i / j), rest);
            }
        },
        Cfg_mut {k: Cons(Add(box ACon(i), box ACon(j)), rest), state:_, k_state:_}=> {
            c.k = Cons(ACon(i + j), rest);
        },
        Cfg_mut {k: Cons(Le(box ACon(i), box ACon(j)), rest), state:_, k_state:_}=> {
            c.k = Cons(BCon(i <= j), rest);
        },
        Cfg_mut {k: Cons(Not(box BCon(b)), rest), state:_, k_state:_}=> {
            c.k = Cons(BCon(!b), rest);
        },
        Cfg_mut {k: Cons(And(box BCon(true), box b), rest), state:_, k_state:_}=> {
            c.k = Cons(b, rest);
        },
        Cfg_mut {k: Cons(And(box BCon(false), _), rest), state:_, k_state:_}=> {
            c.k = Cons(BCon(false), rest);
        },
        Cfg_mut {k: Cons(Assign(i, box ACon(j)), box rest), state, k_state:_}=> {
            c.k = rest;
            c.state = state.insert(i, j);
        },
        Cfg_mut {k: Cons(Seq(box s1, box s2), rest), state:_, k_state:_}=> {
            c.k = Cons(s1, Box::new(Cons(s2, rest)));
        },
        Cfg_mut {k: Cons(Skip, box rest), state:_, k_state:_}=> {
            c.k = rest;
        },
        Cfg_mut {k: Cons(If(box BCon(true), box s, _), rest), state:_, k_state:_}=> {
            c.k = Cons(s, rest);
        },
        Cfg_mut {k: Cons(If(box BCon(false), _, box s), rest), state:_, k_state:_}=> {
            c.k = Cons(s, rest);
        },
        Cfg_mut {k: Cons(While(b, s), rest), state:_, k_state:_}=> {
            let s_ = s.clone();
            let b_ = b.clone();
            c.k = Cons(If(b, Box::new(Seq(s, Box::new(While(b_, s_)))), Box::new(Skip)), rest);
        },
        Cfg_mut {k: Cons(Pgm(Cons(i, box xs), s), box Nil), state, k_state:_}=> {
            c.k = Cons(Pgm(xs, s), Box::new(Nil));
            c.state = state.insert(i, 0);
        },
        Cfg_mut {k: Cons(Pgm(Nil, box s), box Nil), state:_, k_state:_}=> {
            c.k = Cons(s, Box::new(Nil));
        },
        // Heading/cooling rules 
        // Heating 
        Cfg_mut {k: Cons(Div(box e1, box e2), rest), state:_, k_state:_}=> {
            if !a_result(&e1) {
                c.k = Cons(e1, Box::new(Cons(DivL(Box::new(e2)), rest)));
            } else if !a_result(&e2) {
                c.k = Cons(e2, Box::new(Cons(DivR(Box::new(e1)), rest)));
            } else {
                c.k_state = KState::Stuck;
            }
        },
        Cfg_mut {k: Cons(Add(box e1, box e2), rest), state:_, k_state:_}=> {
            if !a_result(&e1) {
                c.k = Cons(e1, Box::new(Cons(AddL(Box::new(e2)), rest)));
            } else if !a_result(&e2) {
                c.k = Cons(e2, Box::new(Cons(AddR(Box::new(e1)), rest)));
            } else {
                c.k_state = KState::Stuck;
            }
        },
        Cfg_mut {k: Cons(Le(box e1, box e2), rest), state:_, k_state:_}=> {
            if !a_result(&e1) {
                c.k = Cons(e1, Box::new(Cons(LeL(Box::new(e2)), rest)));
            } else if !a_result(&e2) {
                c.k = Cons(e2, Box::new(Cons(LeR(Box::new(e1)), rest)));
            } else {
                c.k_state = KState::Stuck
            }
        },
        Cfg_mut {k: Cons(Not(box b), rest), state:_, k_state:_}=> {
            if !b_result(&b) {
                c.k = Cons(b, Box::new(Cons(NotF, rest)));
            } else {
                c.k_state = KState::Stuck;
            }
        },
        Cfg_mut {k: Cons(And(box b1, box b2), rest), state:_, k_state:_}=> {
            if !b_result(&b1) {
                c.k = Cons(b1, Box::new(Cons(AndL(Box::new(b2)), rest)));
            } else {
                c.k_state = KState::Stuck;
            }
        },
        Cfg_mut {k: Cons(Assign(i, box e), rest), state:_, k_state:_}=> {
            if !a_result(&e) {
                c.k =  Cons(e, Box::new(Cons(AssignR(i), rest)));
            } else {
                c.k_state = KState::Stuck;
            }
        },
        Cfg_mut {k: Cons(If(box b, s1, s2), rest), state:_, k_state:_}=> {
            if !b_result(&b) {
                c.k = Cons(b, Box::new(Cons(IfC(s1, s2), rest)));
            } else {
                c.k_state = KState::Stuck
            }
        },
        // Cooling
        // https://stackoverflow.com/questions/28638757/use-of-collaterally-moved-value-error-on-a-recursive-enum#28639004 
        /*   
        Cfg {k: Cons(ACon(e), box Cons(DivL(e2), rest)), state}=> {
            Ok(Cfg{k: Cons(Div(Box::new(ACon(e)), e2), rest), state})
        },
        */
        Cfg_mut {k: Cons(ACon(e), box more), state:_, k_state:_}=> {
            match more {
                Cons(DivL(e2), rest)=> {
                    c.k = Cons(Div(Box::new(ACon(e)), e2), rest);
                },
                Cons(DivR(e1), rest)=> {
                    c.k = Cons(Div(e1, Box::new(ACon(e))), rest);
                },
                Cons(AddL(e2), rest)=> {
                    c.k = Cons(Add(Box::new(ACon(e)), e2), rest);
                },
                Cons(AddR(e1), rest)=> {
                    c.k = Cons(Add(e1, Box::new(ACon(e))), rest);
                },
                Cons(LeL(e2), rest)=> {
                    c.k = Cons(Le(Box::new(ACon(e)), e2), rest);
                },
                Cons(LeR(e1), rest)=> {
                    c.k = Cons(Le(e1, Box::new(ACon(e))), rest);
                },
                Cons(AssignR(i), rest)=> {
                    c.k = Cons(Assign(i, Box::new(ACon(e))), rest);
                },
                _ => {
                    c.k_state = KState::Stuck;
                }
            }
        },
        Cfg_mut {k: Cons(BCon(e), box more), state:_, k_state:_}=> {
            match more {
                Cons(NotF, rest) => {
                    c.k = Cons(Not(Box::new(BCon(e))), rest);
                },
                Cons(AndL(e2), rest) => {
                    c.k = Cons(And(Box::new(BCon(e)), e2), rest);
                },
                Cons(IfC(s1, s2), rest) => {
                    c.k = Cons(If(Box::new(BCon(e)), s1, s2), rest);
                },
                _ => {
                    c.k_state = KState::Stuck;
                }

            }
        },
        _=> {
            c.k_state = KState::Stuck;
        }
    };
    c
    */
// }

#[allow(dead_code)]
fn step(c: Cfg) -> Result<Cfg, Cfg> {
    match c {
        Cfg {k: Cons(AVar(i), rest), state}=> {
            if state.contains_key(&i) {
                Ok(Cfg{k: Cons(ACon(*state.get(&i).unwrap()), rest), state: state})
            } else {
                Err(Cfg{k: Cons(ACon(*state.get(&i).unwrap()), rest), state})
            }
        },  
        Cfg {k: Cons(Div(box ACon(i), box ACon(j)), rest), state}=> {
            if j == 0 {
                Err(Cfg{k: Cons(Div(Box::new(ACon(i)), Box::new(ACon(j))), rest), state})
            } else {
                Ok(Cfg{k: Cons(ACon(i / j), rest), state})
            }
        },
        Cfg {k: Cons(Add(box ACon(i), box ACon(j)), rest), state}=> {
            Ok(Cfg{k: Cons(ACon(i + j), rest), state})
        },
        Cfg {k: Cons(Le(box ACon(i), box ACon(j)), rest), state}=> {
            Ok(Cfg{k: Cons(BCon(i <= j), rest), state})
        },
        Cfg {k: Cons(Not(box BCon(b)), rest), state}=> {
            Ok(Cfg{k: Cons(BCon(!b), rest), state})
        },
        Cfg {k: Cons(And(box BCon(true), box b), rest), state}=> {
            Ok(Cfg{k: Cons(b, rest), state})
        },
        Cfg {k: Cons(And(box BCon(false), _), rest), state}=> {
            Ok(Cfg{k: Cons(BCon(false), rest), state})
        },
        Cfg {k: Cons(Assign(i, box ACon(j)), box rest), state}=> {
            Ok(Cfg{k: rest, state: state.insert(i, j)})
        },
        Cfg {k: Cons(Seq(box s1, box s2), rest), state}=> {
            Ok(Cfg{k: Cons(s1, Box::new(Cons(s2, rest))), state})
        },
        Cfg {k: Cons(Skip, box rest), state}=> {
            Ok(Cfg{k: rest, state})
        },
        Cfg {k: Cons(If(box BCon(true), box s, _), rest), state}=> {
            Ok(Cfg{k: Cons(s, rest), state})
        },
        Cfg {k: Cons(If(box BCon(false), _, box s), rest), state}=> {
            Ok(Cfg{k: Cons(s, rest), state})
        },
        Cfg {k: Cons(While(b, s), rest), state}=> {
            let s_ = s.clone();
            let b_ = b.clone();
            Ok(Cfg{k: Cons(If(b, Box::new(Seq(s, Box::new(While(b_, s_)))), Box::new(Skip)), rest), state})            
        },
        Cfg {k: Cons(Pgm(Cons(i, box xs), s), box Nil), state}=> {
            Ok(Cfg{k: Cons(Pgm(xs, s), Box::new(Nil)), state: state.insert(i, 0)})
        },
        Cfg {k: Cons(Pgm(Nil, box s), box Nil), state}=> {
            Ok(Cfg{k: Cons(s, Box::new(Nil)), state})
        },
        // Heading/cooling rules 
        // Heating 
        Cfg {k: Cons(Div(box e1, box e2), rest), state}=> {
            if !a_result(&e1) {
                Ok(Cfg{k: Cons(e1, Box::new(Cons(DivL(Box::new(e2)), rest))) , state})
            } else if !a_result(&e2) {
                Ok(Cfg{k: Cons(e2, Box::new(Cons(DivR(Box::new(e1)), rest))) , state})
            } else {
                Err(Cfg{k: Cons(Div(Box::new(e1), Box::new(e2)), rest), state})
            }
        },
        Cfg {k: Cons(Add(box e1, box e2), rest), state}=> {
            if !a_result(&e1) {
                Ok(Cfg{k: Cons(e1, Box::new(Cons(AddL(Box::new(e2)), rest))) , state})
            } else if !a_result(&e2) {
                Ok(Cfg{k: Cons(e2, Box::new(Cons(AddR(Box::new(e1)), rest))) , state})
            } else {
                Err(Cfg{k: Cons(Add(Box::new(e1), Box::new(e2)), rest), state})
            }
        },
        Cfg {k: Cons(Le(box e1, box e2), rest), state}=> {
            if !a_result(&e1) {
                Ok(Cfg{k: Cons(e1, Box::new(Cons(LeL(Box::new(e2)), rest))) , state})
            } else if !a_result(&e2) {
                Ok(Cfg{k: Cons(e2, Box::new(Cons(LeR(Box::new(e1)), rest))) , state})
            } else {
                Err(Cfg{k: Cons(Le(Box::new(e1), Box::new(e2)), rest), state})
            }
        },
        Cfg {k: Cons(Not(box b), rest), state}=> {
            if !b_result(&b) {
                Ok(Cfg{k: Cons(b, Box::new(Cons(NotF, rest))) , state})
            } else {
                Err(Cfg{k: Cons(Not(Box::new(b)), rest), state})
            }
        },
        Cfg {k: Cons(And(box b1, box b2), rest), state}=> {
            if !b_result(&b1) {
                Ok(Cfg{k: Cons(b1, Box::new(Cons(AndL(Box::new(b2)), rest))) , state})
            } else {
                Err(Cfg{k: Cons(And(Box::new(b1), Box::new(b2)), rest), state})
            }
        },
        Cfg {k: Cons(Assign(i, box e), rest), state}=> {
            if !a_result(&e) {
                Ok(Cfg{k: Cons(e, Box::new(Cons(AssignR(i), rest))) , state})
            } else {
                Err(Cfg{k: Cons(Assign(i, Box::new(e)), rest), state})
            }
        },
        Cfg {k: Cons(If(box b, s1, s2), rest), state}=> {
            if !b_result(&b) {
                Ok(Cfg{k: Cons(b, Box::new(Cons(IfC(s1, s2), rest))) , state})
            } else {
                Err(Cfg{k: Cons(If(Box::new(b), s1, s2), rest), state})
            }
        },
        // Cooling
        // https://stackoverflow.com/questions/28638757/use-of-collaterally-moved-value-error-on-a-recursive-enum#28639004 
        /*   
        Cfg {k: Cons(ACon(e), box Cons(DivL(e2), rest)), state}=> {
            Ok(Cfg{k: Cons(Div(Box::new(ACon(e)), e2), rest), state})
        },
        */
        Cfg {k: Cons(ACon(e), box more), state}=> {
            match more {
                Cons(DivL(e2), rest)=> {
                    Ok(Cfg{k: Cons(Div(Box::new(ACon(e)), e2), rest), state})
                },
                Cons(DivR(e1), rest)=> {
                    Ok(Cfg{k: Cons(Div(e1, Box::new(ACon(e))), rest), state})
                },
                Cons(AddL(e2), rest)=> {
                    Ok(Cfg{k: Cons(Add(Box::new(ACon(e)), e2), rest), state})
                },
                Cons(AddR(e1), rest)=> {
                    Ok(Cfg{k: Cons(Add(e1, Box::new(ACon(e))), rest), state})
                },
                Cons(LeL(e2), rest)=> {
                    Ok(Cfg{k: Cons(Le(Box::new(ACon(e)), e2), rest), state})
                },
                Cons(LeR(e1), rest)=> {
                    Ok(Cfg{k: Cons(Le(e1, Box::new(ACon(e))), rest), state})
                },
                Cons(AssignR(i), rest)=> {
                    Ok(Cfg{k: Cons(Assign(i, Box::new(ACon(e))), rest), state})
                },
                _ => Err(Cfg{k: Cons(ACon(e), Box::new(more)), state})
            }
        },
        Cfg {k: Cons(BCon(e), box more), state}=> {
            match more {
                Cons(NotF, rest) => {
                    Ok(Cfg{k: Cons(Not(Box::new(BCon(e))), rest), state})
                },
                Cons(AndL(e2), rest) => {
                    Ok(Cfg{k: Cons(And(Box::new(BCon(e)), e2), rest), state})
                },
                Cons(IfC(s1, s2), rest) => {
                    Ok(Cfg{k: Cons(If(Box::new(BCon(e)), s1, s2), rest), state})
                },
                _ => Err(Cfg{k: Cons(BCon(e), Box::new(more)), state})

            }
        }
        _ => Err(Cfg{k: c.k, state: c.state})
    }
}

fn sum_pgm<'a>(size: i64)-> KItem<'a> {
    let n = "n";
    let sum = "sum";
    let args = Cons(n, Box::new(Cons(sum, Box::new(Nil))));
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
}

fn start(p: KItem)->Cfg {
    Cfg {
        k: Cons(p, Box::new(Nil)),
        state: TreeMap::new()
    }
}

fn run(mut c:Cfg) {
    loop {
        // println!("{:?}", c);
        let r = step(c);
        match r {
            Ok(c_)=> {
                c = c_;
                continue;
            },
            Err(c)=> {
                println!("Done {:?}", c);
                break;
            }
        }
    }
    // println!("{:?}", r);

}

fn main() {
    let timer = SystemTime::now();
    // let c = start(sum_pgm(1000000));
    let c = start(sum_pgm(10_000_000));
    // let c = start(sum_pgm(10_000_000));
    run(c);

    let elapsed = timer.elapsed().unwrap();
    let ms = elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000;

    println!("execution time  {}ms", ms);
}
