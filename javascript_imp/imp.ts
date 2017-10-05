// Implementation of imp in JavaScript
// It's meaningless, but I just want to see how fast it can run.  

(function() {
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
  const n = 10000
  const startTime = Date.now()
  const state = test(n)
  const endTime = Date.now()  
  console.log(state)
  console.log(`JavaScript IMP interpreter takes ${endTime - startTime}ms.`)
}

main()
})()