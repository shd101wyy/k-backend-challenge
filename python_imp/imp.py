
import time
current_milli_time = lambda: int(round(time.time() * 1000))

class KItem(object):
    pass

class ACon(KItem):
    def __init__(self, n):
        self.n = n

class AVar(KItem):
    def __init__(self, v):
        self.v = v

class Div(KItem):
    def __init__(self, x, y):
        self.x = x 
        self.y = y 

class Add(KItem):
    def __init__(self, x, y):
        self.x = x 
        self.y = y

class BCon(KItem):
    def __init__(self, b):
        self.b = b

class Le(KItem):
    def __init__(self, x, y):
        self.x = x 
        self.y = y 

class Not(KItem):
    def __init__(self, b):
        self.b = b 

class And(KItem):
    def __init__(self, x, y):
        self.x = x 
        self.y = y

class Assign(KItem):
    def __init__(self, variable, value):
        self.variable = variable
        self.value = value 

class If(KItem):
    def __init__(self, condition, subsequent, alternative):
        self.condition = condition
        self.subsequent = subsequent 
        self.alternative = alternative

class While(KItem):
    def __init__(self, condition, body):
        self.condition = condition
        self.body = body 

class Seq(KItem):
    def __init__(self, s1, s2):
        self.s1 = s1 
        self.s2 = s2 

class Skip(KItem):
    def __init__(self):
        pass

class Pgm(KItem):
    def __init__(self, vars, body):
        self.vars = vars
        self.body = body

class DivL(KItem):
    def __init__(self, v):
        self.v = v 

class DivR(KItem):
    def __init__(self, v):
        self.v = v 

class AddL(KItem):
    def __init__(self, v):
        self.v = v 

class AddR(KItem):
    def __init__(self, v):
        self.v = v 

class LeL(KItem):
    def __init__(self, v):
        self.v = v 

class LeR(KItem):
    def __init__(self, v):
        self.v = v 

class NotF(KItem):
    def __init__(self):
        pass

class AndL(KItem):
    def __init__(self, v):
        self.v = v 

class AssignR(KItem):
    def __init__(self, variable):
        self.variable = variable

class IfC(KItem):
    def __init__(self, subsequent, alternative):
        self.subsequent = subsequent
        self.alternative = alternative

class Cfg(object):
    def __init__(self, k, state):
        self.k = k 
        self.state = state 
        self.stuck = False

def sumPgm(size):
    return Pgm(['n', 'sum'], Seq(Assign('n', ACon(size)), Seq(Assign('sum', ACon(0)), While(Not(Le(AVar('n'), ACon(0))), Seq(Assign('sum', Add(AVar('sum'), AVar('n'))), Assign('n', Add(AVar('n'), ACon(-1))))))))

# 
def step(cfg):
    k = cfg.k 
    state = cfg.state

    if len(k) == 0:
        cfg.stuck = True
        return
    
    task = k.pop()
    if isinstance(task, AVar):
        v = task.v 
        if v in state:
            k.append(ACon(state[v]))
        else:
            cfg.stuck = True
    elif isinstance(task, Div):
        x = task.x 
        y = task.y 
        if not isinstance(x, ACon):
            k.append(DivL(y))
            k.append(x)
        elif not isinstance(y, ACon):
            k.append(DivR(x))
            k.append(y)
        elif y.n == 0:
            cfg.stuck = True 
        else:
            k.append(ACon(x.n / y.n))
    elif isinstance(task, Add):
        x = task.x 
        y = task.y 
        if not isinstance(x, ACon):
            k.append(AddL(y))
            k.append(x)
        elif not isinstance(y, ACon):
            k.append(AddR(x))
            k.append(y)
        else:
            k.append(ACon(x.n + y.n))
    elif isinstance(task, Le):
        x = task.x 
        y = task.y 
        if not isinstance(x, ACon):
            k.append(LeL(y))
            k.append(x)
        elif not isinstance(y, ACon):
            k.append(LeR(x))
            k.append(y)
        else:
            k.append(BCon(x.n <= y.n))
    elif isinstance(task, Not):
        b = task.b 
        if isinstance(b, BCon):
            k.append(BCon(not b.b))
        else:
            k.append(NotF())
            k.append(b)
    elif isinstance(task, And):
        x = task.x 
        y = task.y 
        if not isinstance(x, BCon):
            k.append(AndL(y))
            k.append(x)
        else:
            if x.b:
                k.append(y)
            else:
                k.append(x)
    elif isinstance(task, Assign):
        variable = task.variable
        value = task.value 
        if isinstance(value, ACon):
            state[variable] = value.n
        else:
            k.append(AssignR(variable))
            k.append(value) 
    elif isinstance(task, Seq):
        k.append(task.s2)
        k.append(task.s1)
    elif isinstance(task, Skip):
        pass
    elif isinstance(task, If):
        condition = task.condition
        subsequent = task.subsequent 
        alternative = task.alternative 
        if isinstance(condition, BCon):
            if condition.b:
                k.append(subsequent)
            else:
                k.append(alternative)
        else:
            k.append(IfC(subsequent, alternative))
            k.append(condition)
    elif isinstance(task, While):
        condition = task.condition 
        body = task.body 
        k.append(If(condition, Seq(body, task), Skip()))
    elif isinstance(task, Pgm):
        variables = task.vars 
        body = task.body 
        if len(variables) == 0:
            k.append(body)
        else:
            variable = variables.pop()
            state[variable] = 0
            k.append(task)
    elif isinstance(task, ACon):
        n = task
        task = k.pop()
        if isinstance(task, DivL):
            k.append(Div(n, task.v))
        elif isinstance(task, DivR):
            k.append(Div(task.v, n))
        elif isinstance(task, AddL):
            k.append(Add(n, task.v))
        elif isinstance(task, AddR):
            k.append(Add(task.v, n))
        elif isinstance(task, LeL):
            k.append(Le(n, task.v))
        elif isinstance(task, LeR):
            k.append(Le(task.v, n))
        elif isinstance(task, AssignR):
            k.append(Assign(task.variable, n))
        else:
            cfg.stuck = True
    elif isinstance(task, BCon):
        b = task 
        task = k.pop()
        if isinstance(task, NotF):
            k.append(Not(b))
        elif isinstance(task, AndL):
            k.append(And(b, task.v))
        elif isinstance(task, IfC):
            k.append(If(b, task.subsequent, task.alternative))
        else:
            cfg.stuck = True
    else:
        cfg.stuck = True

        
def test(size):
    return run(sumPgm(size))

def run(p):
    cfg = Cfg([p], {})
    while True:
        step(cfg)
        if cfg.stuck:
            return cfg

def main():
    n = 10000 
    start_time = current_milli_time()
    cfg = test(n)
    print(cfg.state)
    print "Execution time in ms: "
    print (current_milli_time() - start_time)

main()