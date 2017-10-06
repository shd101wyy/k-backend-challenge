// Implementation of imp in JavaScript
// It's meaningless, but I just want to see how fast it can run.  
var __extends = (this && this.__extends) || (function () {
    var extendStatics = Object.setPrototypeOf ||
        ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
        function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
(function () {
    var SIZE = 10000000;
    function direct() {
        var ACon = (function () {
            function ACon(n) {
                this.n = n;
            }
            return ACon;
        }());
        var AVar = (function () {
            function AVar(v) {
                this.v = v;
            }
            return AVar;
        }());
        var Div = (function () {
            function Div(x, y) {
                this.x = x;
                this.y = y;
            }
            return Div;
        }());
        var Add = (function () {
            function Add(x, y) {
                this.x = x;
                this.y = y;
            }
            return Add;
        }());
        var BCon = (function () {
            function BCon(b) {
                this.b = b;
            }
            return BCon;
        }());
        var Le = (function () {
            function Le(x, y) {
                this.x = x;
                this.y = y;
            }
            return Le;
        }());
        var Not = (function () {
            function Not(b) {
                this.b = b;
            }
            return Not;
        }());
        var And = (function () {
            function And(x, y) {
                this.x = x;
                this.y = y;
            }
            return And;
        }());
        var Assign = (function () {
            function Assign(variable, value) {
                this.variable = variable;
                this.value = value;
            }
            return Assign;
        }());
        var If = (function () {
            function If(condition, subsequent, alternative) {
                this.condition = condition;
                this.subsequent = subsequent;
                this.alternative = alternative;
            }
            return If;
        }());
        var While = (function () {
            function While(condition, body) {
                this.condition = condition;
                this.body = body;
            }
            return While;
        }());
        var Seq = (function () {
            function Seq(s1, s2) {
                this.s1 = s1;
                this.s2 = s2;
            }
            return Seq;
        }());
        var Skip = (function () {
            function Skip() {
            }
            return Skip;
        }());
        var Pgm = (function () {
            function Pgm(vars, body) {
                this.vars = vars;
                this.body = body;
            }
            return Pgm;
        }());
        var DivL = (function () {
            function DivL(v) {
                this.v = v;
            }
            return DivL;
        }());
        var DivR = (function () {
            function DivR(v) {
                this.v = v;
            }
            return DivR;
        }());
        var AddL = (function () {
            function AddL(v) {
                this.v = v;
            }
            return AddL;
        }());
        var AddR = (function () {
            function AddR(v) {
                this.v = v;
            }
            return AddR;
        }());
        var LeL = (function () {
            function LeL(v) {
                this.v = v;
            }
            return LeL;
        }());
        var LeR = (function () {
            function LeR(v) {
                this.v = v;
            }
            return LeR;
        }());
        var NotF = (function () {
            function NotF() {
            }
            return NotF;
        }());
        var AndL = (function () {
            function AndL(v) {
                this.v = v;
            }
            return AndL;
        }());
        var AssignR = (function () {
            function AssignR(variable) {
                this.variable = variable;
            }
            return AssignR;
        }());
        var IfC = (function () {
            function IfC(subsequent, alternative) {
                this.subsequent = subsequent;
                this.alternative = alternative;
            }
            return IfC;
        }());
        var Cfg = (function () {
            function Cfg(k, state) {
                this.k = k;
                this.state = state;
            }
            return Cfg;
        }());
        var Nil = (function () {
            function Nil() {
            }
            return Nil;
        }());
        var List = (function (_super) {
            __extends(List, _super);
            function List(first, rest) {
                var _this = _super.call(this) || this;
                _this.first = first;
                _this.rest = rest;
                return _this;
            }
            return List;
        }(Nil));
        var cons = function (x, y) { return new List(x, y); };
        var first = function (x) { return x.first; };
        var rest = function (x) { return x.rest; };
        var aresult = function (a) { return a instanceof ACon; };
        var bresult = function (b) { return b instanceof BCon; };
        var stuck = function (cfg) { return cfg; };
        var sumPgm = function (size) { return new Pgm(['n', 'sum'], new Seq(new Assign('n', new ACon(size)), new Seq(new Assign('sum', new ACon(0)), new While(new Not(new Le(new AVar('n'), new ACon(0))), new Seq(new Assign('sum', new Add(new AVar('sum'), new AVar('n'))), new Assign('n', new Add(new AVar('n'), new ACon(-1)))))))); };
        var step = function (cfg) {
            var k = cfg.k, state = cfg.state;
            if (!k.length)
                throw stuck(cfg);
            var task = k.pop();
            if (task instanceof AVar) {
                var v = task.v;
                if (v in state) {
                    return k.push(new ACon(state[v]));
                }
                else {
                    throw stuck(cfg);
                }
            }
            else if (task instanceof Div) {
                var x = task.x, y = task.y;
                if (!(x instanceof ACon)) {
                    k.push(new DivL(y));
                    k.push(x);
                }
                else if (!(y instanceof ACon)) {
                    k.push(new DivR(x));
                    k.push(y);
                }
                else if (y.n === 0) {
                    throw stuck(cfg);
                }
                else {
                    k.push(new ACon(x.n / y.n));
                }
            }
            else if (task instanceof Add) {
                var x = task.x, y = task.y;
                if (!(x instanceof ACon)) {
                    k.push(new AddL(y));
                    k.push(x);
                }
                else if (!(y instanceof ACon)) {
                    k.push(new AddR(x));
                    k.push(y);
                }
                else {
                    k.push(new ACon(x.n + y.n));
                }
            }
            else if (task instanceof Le) {
                var x = task.x, y = task.y;
                if (!(x instanceof ACon)) {
                    k.push(new LeL(y));
                    k.push(x);
                }
                else if (!(y instanceof ACon)) {
                    k.push(new LeR(x));
                    k.push(y);
                }
                else {
                    k.push(new BCon(x.n <= y.n));
                }
            }
            else if (task instanceof Not) {
                var b = task.b;
                if (b instanceof BCon) {
                    k.push(new BCon(!b.b));
                }
                else {
                    k.push(new NotF());
                    k.push(b);
                }
            }
            else if (task instanceof And) {
                var x = task.x, y = task.y;
                if (!(x instanceof BCon)) {
                    k.push(new AndL(y));
                    k.push(x);
                }
                else {
                    if (x.b) {
                        k.push(y);
                    }
                    else {
                        k.push(x);
                    }
                }
            }
            else if (task instanceof Assign) {
                var variable = task.variable, value = task.value;
                if (value instanceof ACon) {
                    state[variable] = value.n;
                }
                else {
                    k.push(new AssignR(variable));
                    k.push(value);
                }
            }
            else if (task instanceof Seq) {
                k.push(task.s2);
                k.push(task.s1);
            }
            else if (task instanceof Skip) {
            }
            else if (task instanceof If) {
                var condition = task.condition, subsequent = task.subsequent, alternative = task.alternative;
                if (condition instanceof BCon) {
                    if (condition.b) {
                        k.push(subsequent);
                    }
                    else {
                        k.push(alternative);
                    }
                }
                else {
                    k.push(new IfC(subsequent, alternative));
                    k.push(condition);
                }
            }
            else if (task instanceof While) {
                var condition = task.condition, body = task.body;
                k.push(new If(condition, new Seq(body, task), new Skip()));
            }
            else if (task instanceof Pgm) {
                var variables = task.vars, body = task.body;
                if (!variables.length) {
                    k.push(body);
                }
                else {
                    var variable = variables.pop();
                    state[variable] = 0;
                    k.push(task);
                }
            }
            else if (task instanceof ACon) {
                var n = task;
                task = k.pop();
                if (task instanceof DivL) {
                    k.push(new Div(n, task.v));
                }
                else if (task instanceof DivR) {
                    k.push(new Div(task.v, n));
                }
                else if (task instanceof AddL) {
                    k.push(new Add(n, task.v));
                }
                else if (task instanceof AddR) {
                    k.push(new Add(task.v, n));
                }
                else if (task instanceof LeL) {
                    k.push(new Le(n, task.v));
                }
                else if (task instanceof LeR) {
                    k.push(new Le(task.v, n));
                }
                else if (task instanceof AssignR) {
                    k.push(new Assign(task.variable, n));
                }
                else {
                    throw stuck(cfg);
                }
            }
            else if (task instanceof BCon) {
                var b = task;
                task = k.pop();
                if (task instanceof NotF) {
                    k.push(new Not(b));
                }
                else if (task instanceof AndL) {
                    k.push(new And(b, task.v));
                }
                else if (task instanceof IfC) {
                    k.push(new If(b, task.subsequent, task.alternative));
                }
                else {
                    throw stuck(cfg);
                }
            }
            else {
                throw stuck(cfg);
            }
        };
        var run = function (p) {
            var state = {};
            var cfg = new Cfg([p], state);
            try {
                while (cfg.k.length) {
                    step(cfg);
                }
                console.log(cfg.state);
            }
            catch (error) {
                console.log(error);
                return cfg.state;
            }
        };
        var test = function (size) { return run(sumPgm(size)); };
        var main = function () {
            var startTime = Date.now();
            test(SIZE);
            var endTime = Date.now();
            console.log("(direct) JavaScript IMP interpreter takes " + (endTime - startTime) + "ms.");
        };
        main();
    }
    function opt() {
        var Nil = function () { };
        var stuck = function () { return 'Stuck!'; };
        /** Correspondence to redexes */
        var compId = function (x) {
            return function (h) {
                return function (st) {
                    return h(st[x])(st);
                };
            };
        };
        var redDiv = function (i1, i2) {
            return function (h) {
                if (i2 === 0)
                    throw stuck;
                else
                    return h(i1 / i2);
            };
        };
        var redAdd = function (i1, i2) { return function (h) { return h(i1 + i2); }; };
        var redLeq = function (i1, i2) { return function (h) { return h(i1 <= i2); }; };
        var redNot = function (b) { return function (h) { return h(!b); }; };
        var redAnd = function (b, bexp) {
            return function (h) {
                if (b) {
                    return bexp(h);
                }
                else {
                    return h(false);
                }
            };
        };
        var compSkip = function (st) { return st; };
        var redAsgn = function (x, i) { return function (st) {
            st[x] = i;
            return st;
        }; };
        var compSeq = function (s1, s2) { return function (st) { return s2(s1(st)); }; };
        var redIf = function (b, s1, s2) { return b ? s1 : s2; };
        /** Abstract compilation, corresponding to heating rules */
        var compInt = function (n) { return function (h) { return h(n); }; };
        var compBool = function (b) { return function (h) { return h(b); }; };
        var compDiv = function (e1, e2) { return function (h) { return e1(divL(e2, h)); }; };
        var compAdd = function (e1, e2) { return function (h) { return e1(addL(e2, h)); }; };
        var compLeq = function (e1, e2) { return function (h) { return e1(leqL(e2, h)); }; };
        var compNot = function (e) { return function (h) { return e(notF(h)); }; };
        var compAnd = function (e1, e2) { return function (h) { return e1(andL(e2, h)); }; };
        var compAsgn = function (x, e) { return e(asgnR(x)); };
        var compIf = function (b, s1, s2) { return b(ifC(s1, s2)); };
        var compWhile = function (b, s) {
            var loop = function (st) { return compIf(b, compSeq(s, loop), compSkip)(st); };
            return loop;
        };
        var compPgm = function (dec, body) {
            if (dec.length === 0) {
                return body;
            }
            else {
                var x_1 = dec.splice(0, 1)[0];
                var p_1 = compPgm(dec, body);
                return function (st) {
                    st[x_1] = 0;
                    return p_1(st);
                };
            }
        };
        /** Correspondence to cooling rules */
        var divL = function (e, h) { return function (n) { return e(divR(n, h)); }; };
        var divR = function (i1, h) { return function (i2) { return redDiv(i1, i2)(h); }; };
        var addL = function (e, h) { return function (n) { return e(addR(n, h)); }; };
        var addR = function (i1, h) { return function (i2) { return redAdd(i1, i2)(h); }; };
        var leqL = function (e, h) { return function (n) { return e(leqR(n, h)); }; };
        var leqR = function (i1, h) { return function (i2) { return redLeq(i1, i2)(h); }; };
        var notF = function (h) { return function (b) { return redNot(b)(h); }; };
        var andL = function (e, h) { return function (b) { return redNot(b)(h); }; };
        var asgnR = function (x) { return function (i) { return redAsgn(x, i); }; };
        var ifC = function (s1, s2) { return function (b) { return redIf(b, s1, s2); }; };
        var sumPgm = function (size) { return compPgm(['n', 'sum'], compSeq(compAsgn("n", compInt(size)), compSeq(compAsgn("sum", compInt(0)), compWhile(compNot(compLeq(compId("n"), compInt(0))), compSeq(compAsgn("sum", compAdd(compId("sum"), compId("n"))), compAsgn("n", compAdd(compId("n"), compInt(-1)))))))); };
        var run = function (p) {
            var state = {};
            try {
                p(state);
            }
            catch (error) {
                console.log(error);
                return state;
            }
        };
        var test = function (size) { return run(sumPgm(size)); };
        var main = function () {
            var startTime = Date.now();
            var state = test(SIZE);
            var endTime = Date.now();
            console.log(state);
            console.log("(optimized) JavaScript IMP interpreter takes " + (endTime - startTime) + "ms.");
        };
        main();
    }
    var main = function () {
        direct();
        opt();
    };
    main();
})();
