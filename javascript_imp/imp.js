// Implementation of imp in JavaScript
// It's meaningless, but I just want to see how fast it can run.  
(function () {
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
        var n = 10000;
        var startTime = Date.now();
        var state = test(n);
        var endTime = Date.now();
        console.log(state);
        console.log("JavaScript IMP interpreter takes " + (endTime - startTime) + "ms.");
    };
    main();
})();
