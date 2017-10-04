use immutable_map::TreeMap;
use std::rc::Rc;

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
    k: &'a List<KItem<'a>>,
    state: TreeMap<&'a str, i64>,
    stuck: bool
}

impl<'a> Cfg<'a> {
    #[allow(dead_code)]
    fn step(&mut self) {
        let k = &self.k;
        /*
        match k {
            Cons(AVar(i), rest)=> {
                if self.state.contains_key(&i) {
                    let v = *self.state.get(&i).unwrap();
                    self.k = Cons(ACon(v), rest);
                } else {
                    self.stuck = true;
                }
            },  
            _=> {
                self.stuck = true;
            }
        };
        */
        self.k = &Nil;
        ()
    }
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
        state: TreeMap::new(),
        stuck: false
    }
}

pub fn run(mut c:Cfg) {
    loop {
        c.step();
        if c.stuck {
            println!("Done {:?}", c);
            break;
        }
    }
}
*/