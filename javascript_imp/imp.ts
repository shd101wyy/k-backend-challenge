// Implementation of imp in JavaScript
// It's meaningless, but I just want to see how fast it can run.  

(function() {
const SIZE = 10000000

function direct() {
  interface KItem {}

  class ACon implements KItem {
    public n:number
    constructor(n:number) {
      this.n = n
    }
  }

  class AVar implements KItem {
    public v:string
    constructor(v:string) {
      this.v = v
    }
  }

  class Div implements KItem {
    public x:KItem
    public y:KItem
    constructor(x:KItem, y:KItem) {
      this.x = x
      this.y = y
    }
  }

  class Add implements KItem {
    public x:KItem
    public y:KItem
    constructor(x:KItem, y:KItem) {
      this.x = x
      this.y = y
    }
  }

  class BCon implements KItem {
    public b:boolean
    constructor(b:boolean) {
      this.b = b
    }
  }

  class Le implements KItem {
    public x:KItem
    public y:KItem
    constructor(x:KItem, y:KItem) {
      this.x = x
      this.y = y
    }  
  }

  class Not implements KItem {
    public b:KItem
    constructor(b:KItem) {
      this.b = b
    }
  }

  class And implements KItem {
    public x:KItem
    public y:KItem
    constructor(x:KItem, y:KItem) {
      this.x = x
      this.y = y
    }  
  }

  class Assign implements KItem {
    public variable:string
    public value:KItem
    constructor(variable:string, value:KItem) {
      this.variable = variable
      this.value = value
    }  
  }

  class If implements KItem {
    public condition:KItem
    public subsequent:KItem
    public alternative:KItem
    constructor(condition:KItem, subsequent:KItem, alternative:KItem) {
      this.condition = condition
      this.subsequent = subsequent
      this.alternative = alternative
    }  
  }

  class While implements KItem {
    public condition:KItem
    public body:KItem
    constructor(condition:KItem, body:KItem) {
      this.condition = condition
      this.body = body
    }
  }

  class Seq implements KItem {
    public s1:KItem
    public s2:KItem
    constructor(s1:KItem, s2:KItem) {
      this.s1 = s1
      this.s2 = s2 
    }
  }

  class Skip implements KItem {}

  class Pgm implements KItem {
    public vars: string[]
    public body: KItem
    constructor(vars:string[], body:KItem) {
      this.vars = vars 
      this.body = body
    }
  }

  class DivL implements KItem {
    public v: KItem 
    constructor(v:KItem) {
      this.v = v
    }
  }

  class DivR implements KItem {
    public v: KItem 
    constructor(v:KItem) {
      this.v = v
    }
  }

  class AddL implements KItem {
    public v: KItem 
    constructor(v:KItem) {
      this.v = v
    }
  }

  class AddR implements KItem {
    public v: KItem 
    constructor(v:KItem) {
      this.v = v
    }
  }

  class LeL implements KItem {
    public v: KItem 
    constructor(v:KItem) {
      this.v = v
    }
  }

  class LeR implements KItem {
    public v: KItem 
    constructor(v:KItem) {
      this.v = v
    }
  }

  class NotF implements KItem {
  }

  class AndL implements KItem {
    public v: KItem 
    constructor(v:KItem) {
      this.v = v
    }
  }

  class AssignR implements KItem {
    public variable: string 
    constructor(variable:string) {
      this.variable = variable
    }
  }

  class IfC implements KItem {
    public subsequent:KItem
    public alternative:KItem
    constructor(subsequent:KItem, alternative: KItem) {
      this.subsequent = subsequent
      this.alternative = alternative
    }
  }

  class Cfg {
    public k: KItem[]
    public state: {[key:string]:number}
    constructor(k, state) { 
      this.k = k 
      this.state = state
    }
  }

  class Nil {}
  class List extends Nil {
    public first: KItem;
    public rest: KItem;
    constructor(first:KItem, rest:KItem) {
      super()
      this.first = first
      this.rest = rest
    }
  }
  const cons = (x:KItem, y:KItem)=> new List(x, y)
  const first = (x:List)=> x.first
  const rest = (x:List)=> x.rest
  const aresult = (a:KItem)=> a instanceof ACon
  const bresult = (b:KItem)=> b instanceof BCon
  const stuck = (cfg)=> cfg


  const sumPgm = (size:number):Pgm => new Pgm(['n', 'sum'],
    new Seq(new Assign('n', new ACon(size)),
    new Seq(new Assign('sum', new ACon(0)),
    new While(new Not(new Le(new AVar('n'), new ACon(0))),
      new Seq(new Assign('sum', new Add(new AVar('sum'), new AVar('n'))),
          new Assign('n', new Add(new AVar('n'), new ACon(-1))))))))

  const step = (cfg:Cfg)=> {
    const {k, state} = cfg
    if (!k.length) throw stuck(cfg)
    
    let task = k.pop()

    if (task instanceof AVar) {
      const v = task.v
      if (v in state) {
        return k.push(new ACon(state[v]))
      } else {
        throw stuck(cfg)
      }
    } else if (task instanceof Div) {
      const x = task.x, 
            y = task.y
      if (!(x instanceof ACon)) {
        k.push(new DivL(y))
        k.push(x)        
      } else if (!(y instanceof ACon)) {
        k.push(new DivR(x))
        k.push(y)
      } else if (y.n === 0) {
        throw stuck(cfg)
      } else {
        k.push(new ACon(x.n / y.n))
      }
    } else if (task instanceof Add) {
      const x = task.x, 
            y = task.y
      if (!(x instanceof ACon)) {
        k.push(new AddL(y))
        k.push(x)        
      } else if (!(y instanceof ACon)) {
        k.push(new AddR(x))
        k.push(y)
      } else {
        k.push(new ACon(x.n + y.n))
      }
    } else if (task instanceof Le) {
      const x = task.x, 
            y = task.y
      if (!(x instanceof ACon)) {
        k.push(new LeL(y))
        k.push(x)        
      } else if (!(y instanceof ACon)) {
        k.push(new LeR(x))
        k.push(y)
      } else {
        k.push(new BCon(x.n <= y.n))
      }
    } else if (task instanceof Not) {
      const b = task.b
      if (b instanceof BCon) {
        k.push(new BCon(!b.b))
      } else {
        k.push(new NotF())
        k.push(b)
      }
    } else if (task instanceof And) {
      const x = task.x, 
            y = task.y
      if (!(x instanceof BCon)) {
        k.push(new AndL(y))
        k.push(x)        
      } else {
        if (x.b) {
          k.push(y)
        } else {
          k.push(x)
        }
      } 
    } else if (task instanceof Assign) {
      const variable = task.variable,
            value = task.value
      if (value instanceof ACon) {
        state[variable] = value.n
      } else {
        k.push(new AssignR(variable))
        k.push(value)
      }
    } else if (task instanceof Seq) {
      k.push(task.s2)
      k.push(task.s1)
    } else if (task instanceof Skip) {
    } else if (task instanceof If) {
      const condition = task.condition,
            subsequent = task.subsequent,
            alternative = task.alternative
      if (condition instanceof BCon) {
        if (condition.b) {
          k.push(subsequent)
        } else {
          k.push(alternative)
        }
      } else {
        k.push(new IfC(subsequent, alternative))
        k.push(condition)
      }
    } else if (task instanceof While) {
      const condition = task.condition,
            body = task.body
      k.push(new If(condition, new Seq(body, task), new Skip()))
    } else if (task instanceof Pgm) {
      let variables = task.vars,
          body = task.body
      if (!variables.length) {
        k.push(body)
      } else {
        let variable = variables.pop()
        state[variable] = 0
        k.push(task)
      }
    } else if (task instanceof ACon) {
      const n = task
      task = k.pop()
      if (task instanceof DivL) {
        k.push(new Div(n, task.v))
      } else if (task instanceof DivR) {
        k.push(new Div(task.v, n))
      } else if (task instanceof AddL) {
        k.push(new Add(n, task.v))
      } else if (task instanceof AddR) {
        k.push(new Add(task.v, n))
      } else if (task instanceof LeL) {
        k.push(new Le(n, task.v))
      } else if (task instanceof LeR) {
        k.push(new Le(task.v, n))
      } else if (task instanceof AssignR) {
        k.push(new Assign(task.variable, n))
      } else {
        throw stuck(cfg)
      }
    } else if (task instanceof BCon) {
      const b = task
      task = k.pop()
      if (task instanceof NotF) {
        k.push(new Not(b))
      } else if (task instanceof AndL) {
        k.push(new And(b, task.v))
      } else if (task instanceof IfC) {
        k.push(new If(b, task.subsequent, task.alternative))
      } else {
        throw stuck(cfg)
      }
    } else {
      throw stuck(cfg)
    }
  }


  const run = (p:Pgm) => {
    const state = {}
    const cfg = new Cfg([p], state)
    try {
      while (cfg.k.length) {
        step(cfg)
      }
      console.log(cfg.state)
    } catch(error) {
      console.log(error)
      return cfg.state
    }
  }
  
  
  const test = (size:number) => run(sumPgm(size))
  
  
  const main = ()=> {
    const startTime = Date.now()
    test(SIZE)
    const endTime = Date.now()  
    console.log(`(direct) JavaScript IMP interpreter takes ${endTime - startTime}ms.`)
  }
  
  main()
}


function opt() {

  const Nil = function(){}
  
  type St = {[key:string]:number}
  type Comp = (st:St)=>St
  type AHole = (n:number)=>Comp
  type BHole = (b:boolean)=>Comp
  type AComp = (h:AHole)=>Comp
  type BComp = (h:BHole)=>Comp
  const stuck = ()=> 'Stuck!'
  
  /** Correspondence to redexes */
  const compId = (x:string):AComp => 
    (h:AHole)=> {
      return (st:St)=> {
        return h(st[x])(st)
      }
    }
  
  const redDiv = (i1:number, i2:number):AComp => 
    (h: AHole)=> {
      if (i2 === 0)  
        throw stuck 
      else 
        return h(i1 / i2)
    }
  
  const redAdd = (i1:number, i2:number):AComp => (h: AHole)=> h(i1 + i2)
  const redLeq = (i1:number, i2:number):BComp => (h: BHole)=> h(i1 <= i2)
  const redNot = (b:boolean):BComp => (h: BHole)=> h(!b)
  const redAnd = (b:boolean, bexp: BComp):BComp => 
    (h: BHole)=> {
      if (b) {
        return bexp(h)
      } else {
        return h(false)
      }
    }
  
  const compSkip = (st: St):St => st
  const redAsgn = (x:string, i:number):Comp => (st:St)=> {
    st[x] = i
    return st
  }
  const compSeq = (s1: Comp, s2: Comp): Comp => (st:St)=> s2(s1(st))
  const redIf = (b:boolean, s1: Comp, s2: Comp):Comp => b ? s1 : s2
  
  /** Abstract compilation, corresponding to heating rules */
  const compInt = (n: number):AComp => (h:AHole) => h(n)
  const compBool = (b: boolean):BComp => (h:BHole)=> h(b)
  const compDiv = (e1: AComp, e2: AComp):AComp => (h:AHole)=> e1(divL(e2, h))
  const compAdd = (e1: AComp, e2: AComp):AComp => (h:AHole)=> e1(addL(e2, h))
  const compLeq = (e1: AComp, e2: AComp):BComp => (h:BHole)=> e1(leqL(e2, h))
  const compNot = (e:BComp):BComp => (h:BHole)=> e(notF(h))
  const compAnd = (e1: BComp, e2: BComp):BComp => (h:BHole)=> e1(andL(e2, h))
  const compAsgn = (x: string, e:AComp):Comp => e(asgnR(x))
  const compIf = (b: BComp, s1: Comp, s2: Comp):Comp => b(ifC(s1, s2))
  const compWhile = (b:BComp, s: Comp):Comp => {
    const loop = (st:St):St => compIf(b, compSeq(s, loop), compSkip)(st)
    return loop
  }
  const compPgm = (dec: string[], body: Comp):Comp => {
    if (dec.length === 0) {
      return body
    } else {
      const x = dec.splice(0, 1)[0]
      const p = compPgm(dec, body)
      return (st:St)=> {
        st[x] = 0
        return p(st)
      }
    }
  } 
  
  /** Correspondence to cooling rules */
  const divL = (e: AComp, h: AHole):AHole => (n)=> e(divR(n, h))
  const divR = (i1: number, h: AHole):AHole => (i2)=> redDiv(i1, i2)(h)
  const addL = (e: AComp, h: AHole):AHole => (n)=> e(addR(n, h))
  const addR = (i1: number, h: AHole):AHole => (i2)=> redAdd(i1, i2)(h)
  const leqL = (e: AComp, h: BHole):AHole => (n:number)=> e(leqR(n, h))
  const leqR = (i1: number, h: BHole):AHole => (i2)=> redLeq(i1, i2)(h)
  const notF = (h: BHole):BHole => (b:boolean)=> redNot(b)(h)
  const andL = (e: BComp, h: BHole):BHole => (b:boolean)=> redNot(b)(h)
  const asgnR = (x:string):AHole=> (i:number)=> redAsgn(x, i)
  const ifC = (s1:Comp, s2: Comp): BHole => (b)=> redIf(b, s1, s2)
  
  const sumPgm = (size: number):Comp => compPgm(['n', 'sum'],
    compSeq(compAsgn("n", compInt(size)),
    compSeq(compAsgn("sum", compInt(0)),
    compWhile(compNot(compLeq(compId("n"), compInt(0))),
      compSeq(compAsgn("sum", compAdd(compId("sum"), compId("n"))),
              compAsgn("n", compAdd(compId("n"), compInt(-1))))))))
  
  const run = (p:Comp):St => {
    const state = {}
    try {
      p(state)
    } catch(error) {
      console.log(error)
      return state
    }
  }
  
  
  const test = (size:number) => run(sumPgm(size))
  
  
  const main = ()=> {
    const startTime = Date.now()
    const state = test(SIZE)
    const endTime = Date.now()  
    console.log(state)
    console.log(`(optimized) JavaScript IMP interpreter takes ${endTime - startTime}ms.`)
  }
  
  main()
}

const main = ()=> {
  direct()
  opt()
}
main()
})()