use immutable_map::TreeMap;

// ================================
// ========  Unoptimized  =========
// ================================
#[derive(Clone, Debug)]
pub enum KItem<'a> {
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
use self::KItem::*;

#[derive(Clone, Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}
use self::List::*;

#[derive(Clone, Debug)]
pub struct Cfg<'a> {
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

#[allow(dead_code)]
fn step(c: Cfg) -> Result<Cfg, Cfg> {
    let k = c.k;
    let state = c.state;
    match k {
        Cons(AVar(i), rest) => {
            if state.contains_key(&i) {
                Ok(Cfg{k: Cons(ACon(*state.get(&i).unwrap()), rest), state: state})
            } else {
                Err(Cfg{k: Cons(ACon(*state.get(&i).unwrap()), rest), state})
            }
        },  
        Cons(Div(box ACon(i), box ACon(j)), rest)=> {
            if j == 0 {
                Err(Cfg{k: Cons(Div(Box::new(ACon(i)), Box::new(ACon(j))), rest), state})
            } else {
                Ok(Cfg{k: Cons(ACon(i / j), rest), state})
            }
        },
        Cons(Add(box ACon(i), box ACon(j)), rest)=> {
            Ok(Cfg{k: Cons(ACon(i + j), rest), state})
        },
        Cons(Le(box ACon(i), box ACon(j)), rest)=> {
            Ok(Cfg{k: Cons(BCon(i <= j), rest), state})
        },
        Cons(Not(box BCon(b)), rest)=> {
            Ok(Cfg{k: Cons(BCon(!b), rest), state})
        },
        Cons(And(box BCon(true), box b), rest)=> {
            Ok(Cfg{k: Cons(b, rest), state})
        },
        Cons(And(box BCon(false), _), rest)=> {
            Ok(Cfg{k: Cons(BCon(false), rest), state})
        },
        Cons(Assign(i, box ACon(j)), box rest)=> {
            Ok(Cfg{k: rest, state: state.insert(i, j)})
        },
        Cons(Seq(box s1, box s2), rest)=> {
            Ok(Cfg{k: Cons(s1, Box::new(Cons(s2, rest))), state})
        },
        Cons(Skip, box rest)=> {
            Ok(Cfg{k: rest, state})
        },
        Cons(If(box BCon(true), box s, _), rest)=> {
            Ok(Cfg{k: Cons(s, rest), state})
        },
        Cons(If(box BCon(false), _, box s), rest)=> {
            Ok(Cfg{k: Cons(s, rest), state})
        },
        Cons(While(b, s), rest)=> {
            let s_ = s.clone();
            let b_ = b.clone();
            Ok(Cfg{k: Cons(If(b, Box::new(Seq(s, Box::new(While(b_, s_)))), Box::new(Skip)), rest), state})            
        },
        Cons(Pgm(Cons(i, box xs), s), box Nil)=> {
            Ok(Cfg{k: Cons(Pgm(xs, s), Box::new(Nil)), state: state.insert(i, 0)})
        },
        Cons(Pgm(Nil, box s), box Nil)=> {
            Ok(Cfg{k: Cons(s, Box::new(Nil)), state})
        },
        // Heading/cooling rules 
        // Heating 
        Cons(Div(box e1, box e2), rest)=> {
            if !a_result(&e1) {
                Ok(Cfg{k: Cons(e1, Box::new(Cons(DivL(Box::new(e2)), rest))) , state})
            } else if !a_result(&e2) {
                Ok(Cfg{k: Cons(e2, Box::new(Cons(DivR(Box::new(e1)), rest))) , state})
            } else {
                Err(Cfg{k: Cons(Div(Box::new(e1), Box::new(e2)), rest), state})
            }
        },
        Cons(Add(box e1, box e2), rest)=> {
            if !a_result(&e1) {
                Ok(Cfg{k: Cons(e1, Box::new(Cons(AddL(Box::new(e2)), rest))) , state})
            } else if !a_result(&e2) {
                Ok(Cfg{k: Cons(e2, Box::new(Cons(AddR(Box::new(e1)), rest))) , state})
            } else {
                Err(Cfg{k: Cons(Add(Box::new(e1), Box::new(e2)), rest), state})
            }
        },
        Cons(Le(box e1, box e2), rest)=> {
            if !a_result(&e1) {
                Ok(Cfg{k: Cons(e1, Box::new(Cons(LeL(Box::new(e2)), rest))) , state})
            } else if !a_result(&e2) {
                Ok(Cfg{k: Cons(e2, Box::new(Cons(LeR(Box::new(e1)), rest))) , state})
            } else {
                Err(Cfg{k: Cons(Le(Box::new(e1), Box::new(e2)), rest), state})
            }
        },
        Cons(Not(box b), rest)=> {
            if !b_result(&b) {
                Ok(Cfg{k: Cons(b, Box::new(Cons(NotF, rest))) , state})
            } else {
                Err(Cfg{k: Cons(Not(Box::new(b)), rest), state})
            }
        },
        Cons(And(box b1, box b2), rest)=> {
            if !b_result(&b1) {
                Ok(Cfg{k: Cons(b1, Box::new(Cons(AndL(Box::new(b2)), rest))) , state})
            } else {
                Err(Cfg{k: Cons(And(Box::new(b1), Box::new(b2)), rest), state})
            }
        },
        Cons(Assign(i, box e), rest)=> {
            if !a_result(&e) {
                Ok(Cfg{k: Cons(e, Box::new(Cons(AssignR(i), rest))) , state})
            } else {
                Err(Cfg{k: Cons(Assign(i, Box::new(e)), rest), state})
            }
        },
        Cons(If(box b, s1, s2), rest)=> {
            if !b_result(&b) {
                Ok(Cfg{k: Cons(b, Box::new(Cons(IfC(s1, s2), rest))) , state})
            } else {
                Err(Cfg{k: Cons(If(Box::new(b), s1, s2), rest), state})
            }
        },
        // Cooling
        // https://stackoverflow.com/questions/28638757/use-of-collaterally-moved-value-error-on-a-recursive-enum#28639004 
        /*   
        Cons(ACon(e), box Cons(DivL(e2), rest)), state}=> {
            Ok(Cfg{k: Cons(Div(Box::new(ACon(e)), e2), rest), state})
        },
        */
        Cons(ACon(e), box more)=> {
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
        Cons(BCon(e), box more)=> {
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
        _ => Err(Cfg{k, state})
    }
}

pub fn sum_pgm<'a>(size: i64)-> KItem<'a> {
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

pub fn start(p: KItem)->Cfg {
    Cfg {
        k: Cons(p, Box::new(Nil)),
        state: TreeMap::new()
    }
}

pub fn run(mut c:Cfg) {
    loop {
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
}