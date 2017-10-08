// ===========================================================
// ========  Optimized  ======================================
// ======== Change from state from immutable map to vector ===
// ===========================================================
use std::rc::Rc;

type VarOffset = usize;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum KItem {
    ACon(i64),
    AVar(VarOffset),
    Div(Rc<KItem>, Rc<KItem>),
    Add(Rc<KItem>, Rc<KItem>),
    BCon(bool),
    Le(Rc<KItem>, Rc<KItem>),
    Not(Rc<KItem>),
    And(Rc<KItem>, Rc<KItem>),
    Assign(VarOffset, Rc<KItem>),
    If(Rc<KItem>, Rc<KItem>, Rc<KItem>),
    While(Rc<KItem>, Rc<KItem>),
    Seq(Rc<KItem>, Rc<KItem>),
    Skip,
    Pgm(Rc<List<VarOffset>>, Rc<KItem>),
    // Pgm(Vec<VarOffset>, Rc<KItem>),
    DivL(Rc<KItem>),
    DivR(Rc<KItem>),
    AddL(Rc<KItem>),
    AddR(Rc<KItem>),
    LeL(Rc<KItem>),
    LeR(Rc<KItem>),
    NotF,
    AndL(Rc<KItem>),
    AssignR(VarOffset),
    IfC(Rc<KItem>, Rc<KItem>)
}
use self::KItem::*;

#[derive(Clone, Debug)]
pub enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil
}
use self::List::*;

#[derive(Clone, Debug)]
pub struct Cfg {
    k: Vec<Rc<KItem>>,
    state: Vec<i64>,
    stuck: bool
}

#[allow(dead_code)]
fn step(c: &mut Cfg) {
    let state = &mut c.state;
    let k = &mut c.k;

    if k.len() == 0 {
        c.stuck = true;
        return;
    }

    let task = k.pop().unwrap();

    if let AVar(offset) = *task {
        if offset < state.len() { // variable exists
            k.push(Rc::new(ACon(state[offset])));
        } else {
            c.stuck = true;
        }
    } else if let Div(ref x, ref y) = *task {
        match (x.as_ref(), y.as_ref()) {
            (&ACon(xi), &ACon(yi)) => {
                k.push(Rc::new(ACon(xi / yi)));
            },
            (_, &ACon(_))=> {
                k.push(Rc::new(DivL(y.clone())));
                k.push(x.clone());
            },
            _ => {
                k.push(Rc::new(DivL(x.clone())));
                k.push(y.clone());
            }
        }
    } else if let Add(ref x, ref y) = *task {
        match (x.as_ref(), y.as_ref()) {
            (&ACon(xi), &ACon(yi)) => {
                k.push(Rc::new(ACon(xi + yi)));
            },
            (_, &ACon(_))=> {
                k.push(Rc::new(AddL(y.clone())));
                k.push(x.clone());
            },
            _ => {
                k.push(Rc::new(AddR(x.clone())));
                k.push(y.clone());
            }
        }
    } else if let Le(ref x, ref y) = *task {
        match (x.as_ref(), y.as_ref()) {
            (&ACon(xi), &ACon(yi)) => {
                k.push(Rc::new(BCon(xi <= yi)));
            },
            (_, &ACon(_))=> {
                k.push(Rc::new(LeL(y.clone())));
                k.push(x.clone());
            },
            _ => {
                k.push(Rc::new(LeR(x.clone())));
                k.push(y.clone());
            }
        }
    } else if let Not(ref x) = *task {
        match x.as_ref() {
            &BCon(b)=> {
                k.push(Rc::new(BCon(!b)));
            },
            _=> {
                k.push(Rc::new(NotF));
                k.push(x.clone());
            }
        }
    } else if let And(ref x, ref y) = *task {
        match x.as_ref() {
            &BCon(b)=> {
                if b {
                    k.push(y.clone())
                } else {
                    k.push(x.clone())
                }
            },
            _=> {
                k.push(Rc::new(AndL(y.clone())));
                k.push(x.clone());
            }
        }
    } else if let Assign(offset, ref val) = *task {
        match val.as_ref() {
            &ACon(x)=> {
                state[offset] = x;
            },
            _=> {
                k.push(Rc::new(AssignR(offset)));
                k.push(val.clone());
            }
        }
    } else if let Seq(ref s1, ref s2) = *task {
        k.push(s2.clone());
        k.push(s1.clone());
    } else if let Skip = *task {
        // do nothing
    } else if let If(ref condition, ref subsequent, ref alternative) = *task {
        match condition.as_ref() {
            &BCon(b)=> {
                if b {
                    k.push(subsequent.clone());
                } else {
                    k.push(alternative.clone());
                }
            },
            _=> {
                k.push(Rc::new(IfC(subsequent.clone(), alternative.clone())));
                k.push(condition.clone());
            }
        }
    } else if let While(ref condition, ref body) = *task {
        k.push(Rc::new(If(condition.clone(), Rc::new(Seq(body.clone(), task.clone())), Rc::new(Skip))));
    } else if let Pgm(ref variables, ref body) = *task {
        match variables.as_ref() {
            &Cons(variable, ref rest) => {
                state[variable] = 0;
                k.push(Rc::new(Pgm(rest.clone(), body.clone())));
            },
            _=> {
                k.push(body.clone());
            }
        }
    } else if let ACon(_) = *task {
        let n = &task;
        let task = k.pop().unwrap();

        match *task {
            DivL(ref v)=> {
                k.push(Rc::new(Div(n.clone(), v.clone())));
            },
            DivR(ref v)=> {
                k.push(Rc::new(Div(v.clone(), n.clone())));
            },
            AddL(ref v)=> {
                k.push(Rc::new(Add(n.clone(), v.clone())));
            },
            AddR(ref v)=> {
                k.push(Rc::new(Div(v.clone(), n.clone())));
            },
            LeL(ref v)=> {
                k.push(Rc::new(Le(n.clone(), v.clone())));
            },
            LeR(ref v)=> {
                k.push(Rc::new(Le(n.clone(), v.clone())));
            },
            AssignR(offset)=> {
                k.push(Rc::new(Assign(offset, n.clone())));
            },
            _=> {
                c.stuck = true;
            }
        }
    } else if let BCon(_) = *task {
        let b = &task;
        let task = k.pop().unwrap();
        match *task {
            NotF=> {
                k.push(Rc::new(Not(b.clone())));
            },
            AndL(ref v)=> {
                k.push(Rc::new(And(b.clone(), v.clone())));
            },
            IfC(ref subsequent, ref alternative)=> {
                k.push(Rc::new(If(b.clone(), subsequent.clone(), alternative.clone())));
            },
            _ => {
                c.stuck = true;
            }
        }
    } else {
        c.stuck = true;
    }
}

pub fn sum_pgm(size: i64)-> KItem {
    let n = 0;
    let sum = 1;
    let args = Cons(n, Rc::new(Cons(sum, Rc::new(Nil))));
    Pgm(Rc::new(args), 
        Rc::new(Seq(
            Rc::new(Assign(n, Rc::new(ACon(size)))),
        Rc::new(Seq(
            Rc::new(Assign(sum, Rc::new(ACon(0)))),
            Rc::new(While(Rc::new(Not(Rc::new(Le(Rc::new(AVar(n)), Rc::new(ACon(0)))))),
                        Rc::new(Seq(
                            Rc::new(Assign(sum, Rc::new(Add(Rc::new(AVar(sum)), Rc::new(AVar(n)))))),
                            Rc::new(Assign(n, Rc::new(Add(Rc::new(AVar(n)), Rc::new(ACon(-1))))))
                        )))))))))
}

pub fn start(p: KItem)->Cfg {
    let mut k = Vec::new();
    k.push(Rc::new(p));
    Cfg {
        k,
        state: vec![0; 16],
        stuck: false
    }
}

pub fn run(mut c:Cfg) {
    loop {
        step(&mut c);
        if c.stuck {
            println!("Done {:?}", c);
            break;
        }
    }
}