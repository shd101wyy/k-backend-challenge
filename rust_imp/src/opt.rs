// ================================
// ========== Optimized ===========
// ================================
/*
#[derive(Clone, Debug)]
pub enum KItem {
    ACon(i64),
    AVar(usize), // <= Var is denoted as number now.  
    Div(Box<KItem>, Box<KItem>),
    Add(Box<KItem>, Box<KItem>),
    BCon(bool),
    Le(Box<KItem>, Box<KItem>),
    Not(Box<KItem>),
    And(Box<KItem>, Box<KItem>),
    Assign(usize, Box<KItem>),
    If(Box<KItem>, Box<KItem>, Box<KItem>),
    While(Box<KItem>, Box<KItem>),
    Seq(Box<KItem>, Box<KItem>),
    Skip,
    Pgm(Vec<usize>, Box<KItem>),

    DivL(Box<KItem>),
    DivR(Box<KItem>),
    AddL(Box<KItem>),
    AddR(Box<KItem>),
    LeL(Box<KItem>),
    LeR(Box<KItem>),
    NotF,
    AndL(Box<KItem>),
    AssignR(usize),
    IfC(Box<KItem>, Box<KItem>)
}
use self::KItem::*;

#[derive(Clone, Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}
use self::List::*;

#[derive(Clone, Debug)]
pub struct CfgOpt {
    k: List<KItem>,
    state: Vec<i64>,
    stuck: bool
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



// Mutable
fn step_opt(mut c: CfgOpt)->CfgOpt {
    let k =  c.k;
    let state = &c.state;

    match k {
        Cons(AVar(i), rest)=> {
            if i < state.len() {
                c.k = *rest;
            } else {
                c.stuck = true;
            }
        },
        Cons(Div(box ACon(i), box ACon(j)), rest)=> {
        },
        Cons(Add(box x, box y), rest)=> {
        },
        Cons(Le(box x, box y), rest)=> {
        },
        Cons(Not(box x), rest)=> {
        },
        Cons(And(box x, box y), rest)=> {
        },
        _ => {
            c.stuck = true;
        }
    };
    c
}
*/
/*
type State = [i64; 128];
type Comp = Fn(&mut State)->&mut State;
type AHole = Fn(i64)->Box<Comp>;
type BHole = Fn(bool)->Box<Comp>;
type AComp = Fn(Box<AHole>)->Box<Comp>;
type BComp = Fn(Box<BHole>)->Box<Comp>;
type Test = Fn(i32)->i32;

fn comId(id: usize)-> Box<AComp> {
    Box::new(move |h: Box<AHole>| {
        Box::new(move |state: &mut State| {
            h(state[id])(state)
        })
    })
}
fn redDiv(i1: i64, i2: i64)->Box<AComp> {
    Box::new(move |h: Box<AHole>| {
        match i2 {
            0=> panic!("error"),
            _=> h(i1 / i2)
        }
    })
}
fn redAdd(i1: i64, i2: i64)->Box<AComp> {
    Box::new(move |h: Box<AHole>| h(i1 + i2))
}
fn redLeq(i1: i64, i2: i64)->Box<BComp> {
    Box::new(move |h: Box<BHole>| h(i1 <= i2))
}
fn redNot(b:bool)->Box<BComp> {
    Box::new(move |h: Box<BHole>| h(!b))
}
fn redAnd(b:bool, bexp: Box<BComp>)->Box<BComp> {
    Box::new(move |h: Box<BHole>| {
        if b {
            bexp(h)
        } else {
            h(false)
        }
    })
}
fn redAsgn(id: usize, i: i64)-> Box<Comp> {
    Box::new(move |state: &mut State| {
        state[id] = i;
        state
    })
}
fn compSeq(s1: Box<Comp>, s2: Box<Comp>)-> Box<Comp> {
    Box::new(move |state: &mut State| {
        s2(s1(state))
    })
}

fn redIf(b: bool, s1: Box<Comp>, s2: Box<Comp>)-> Box<Comp> {
    if b {s1} else {s2}
}
*/


/* Abstract compilation, corresponding to heating rules */

/*
fn compInt(n: i64)-> Box<AComp> {
    Box::new(move |h: Box<AHole>| h(n))
}
fn compBool(b: bool)-> Box<BComp> {
    Box::new(move |h: Box<BHole>| h(b))
}
fn compDiv(e1: Box<AComp>, e2: Box<AComp>)-> Box<AComp> {
    Box::new(move |h: Box<AHole>| e1(divL(e2, h)))
}
fn compAdd(e1: Box<AComp>, e2: Box<AComp>)-> Box<AComp> {
    Box::new(move |h: Box<AHole>| e1(addL(e2, h)))
}
fn compLeq(e1: Box<AComp>, e2: Box<AComp>)-> Box<BComp> {
    Box::new(move |h: Box<BHole>| e1(leqL(e2, h)))
}
fn compNot(e: Box<BComp>)->Box<BComp> {
    Box::new(move |h: Box<BHole>| e(notF(h)))
}
fn compAnd(e1: Box<BComp>, e2: Box<BComp>)-> Box<BComp> {
    Box::new(move |h: Box<BHole>| e1(andL(e2, h)))
}
fn compAsgn(x: usize, e: Box<AComp>)-> Box<Comp> {
    e(asgnR(x))
}
fn compIf(b: Box<BComp>, s1: Box<Comp>, s2: Box<Comp>)-> Box<Comp> {
    b(ifC(s1, s2))
}

struct Helper {
    f: Box<Comp>
}
fn compWhile(b: Box<BComp>, s: Box<Comp>)-> Box<Comp> {
    let compSkip: Box<Comp> = Box::new(|st: &mut State| st);

    let mut loop_ = Helper {
        f: Box::new(|st: &mut State| st)
    };
    loop_.f = Box::new(|st: &mut State| compIf(b, compSeq(s, loop_.f), compSkip)(st));
    loop_.f
}
*/

/* Correspondence to cooling rules */
/*
fn divL(e: Box<AComp>, h: Box<AHole>)-> Box<AHole> {
    Box::new(move |n: i64| e(divR(n, h)))
}
fn divR(i1: i64, h: Box<AHole>)-> Box<AHole> {
    Box::new(move |i2: i64| redDiv(i1, i2)(h))
}
fn addL(e: Box<AComp>, h: Box<AHole>)-> Box<AHole> {
    Box::new(move |n: i64| e(addR(n, h)))
}
fn addR(i1: i64, h: Box<AHole>)-> Box<AHole> {
    Box::new(move |i2: i64| redAdd(i1, i2)(h))
}
fn leqL(e: Box<AComp>, h: Box<BHole>)-> Box<AHole> {
    Box::new(move |n: i64| e(leqR(n, h)))
}
fn leqR(i1: i64, h: Box<BHole>)-> Box<AHole> {
    Box::new(move |i2: i64| redLeq(i1, i2)(h))
}
fn notF(h: Box<BHole>)-> Box<BHole> {
    Box::new(move |b: bool| redNot(b)(h))
}
fn andL(e: Box<BComp>, h: Box<BHole>)-> Box<BHole> {
    Box::new(move |b: bool| redAnd(b, e)(h))
}
fn asgnR(x: usize)-> Box<AHole> {
    Box::new(move |i: i64| redAsgn(x, i))
}
fn ifC(s1: Box<Comp>, s2: Box<Comp>)-> Box<BHole> {
    Box::new(move |b: bool| redIf(b, s1, s2))
}
*/