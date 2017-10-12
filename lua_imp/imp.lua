function ACon(n)
    return {n=n, tag="ACon"}
end

function AVar(v)
    return {v=v, tag="AVar"}
end

function Div(x, y)
    return {x=x, y=y, tag="Div"}
end

function Add(x, y)
    return {x=x, y=y, tag="Add"}
end

function BCon(b)
    return {b=b, tag="BCon"}
end

function Le(x, y)
    return {x=x, y=y, tag="Le"}
end

function Not(b)
    return {b=b, tag="Not"}
end

function And(x, y)
    return {x=x, y=y, tag="And"}
end

function Assign(variable, value)
    return {variable=variable, value=value, tag="Assign"}
end

function If(condition, subsequent, alternative)
    return {condition=condition, subsequent=subsequent, alternative=alternative, tag="If"}
end

function While(condition, body)
    return {condition=condition, body=body, tag="While"}
end

function Seq(s1, s2) 
    return {s1=s1, s2=s2, tag="Seq"}
end

function Skip()
    return {tag="Skip"}
end

function Pgm(vars, body)
    return {vars=vars, body=body, tag="Pgm"}
end

function DivL(v)
    return {v=v, tag="DivL"}
end

function DivR(v)
    return {v=v, tag="DivR"}
end

function AddL(v)
    return {v=v, tag="AddL"}
end

function AddR(v)
    return {v=v, tag="AddR"}
end

function LeL(v)
    return {v=v, tag="LeL"}
end

function LeR(v)
    return {v=v, tag="LeR"}
end

function NotF()
    return {tag="NotF"}
end

function AndL(v)
    return {v=v, tag="AndL"}
end

function AssignR(variable)
    return {variable=variable, tag="AssignR"}
end

function IfC(subsequent, alternative) 
    return {subsequent=subsequent, alternative=alternative, tag="IfC"}
end

function Cfg(k, state) 
    return {k=k, state=state, stuck=false}
end 

function step(cfg)
    local k = cfg.k
    local state = cfg.state 
    if #k == 0 then
        cfg.stuck = true 
        return
    end

    local task = k[#k]
    local tag = task.tag 
    table.remove(k, #k)

    if tag == 'AVar' then
        local v = task.v 
        if state[v] == nil then
            cfg.stuck = true 
        else      
            table.insert(k, ACon(state[v]))
        end
    elseif tag == 'Div' then
        local x = task.x 
        local y = task.y 
        if x.tag ~= 'ACon' then
            table.insert(k, DivL(y))
            table.insert(k, x)
        elseif y.tag ~= 'ACon' then 
            table.insert(k, DivR(x))
            table.insert(k, y)
        elseif y.n == 0 then 
            cfg.stuck = true 
        else  
            table.insert(k, ACon(x.n / y.n))
        end
    elseif tag == 'Add' then
        local x = task.x 
        local y = task.y 
        if x.tag ~= 'ACon' then
            table.insert(k, AddL(y))
            table.insert(k, x)
        elseif y.tag ~= 'ACon' then 
            table.insert(k, AddR(x))
            table.insert(k, y)
        else  
            table.insert(k, ACon(x.n + y.n))
        end
    elseif tag == 'Le' then 
        local x = task.x 
        local y = task.y 
        if x.tag ~= 'ACon' then
            table.insert(k, LeL(y))
            table.insert(k, x)
        elseif y.tag ~= 'ACon' then 
            table.insert(k, LeR(x))
            table.insert(k, y)
        else  
            table.insert(k, BCon(x.n <= y.n))
        end
    elseif tag == 'Not' then     
        local b = task.b 
        if b.tag == 'BCon' then 
            table.insert(k, BCon(not b.b))
        else 
            table.insert(k, NotF())
            table.insert(k, b)
        end
    elseif tag == 'And' then 
        local x = task.x 
        local y = task.y 
        if x.tag ~= 'BCon' then 
            table.insert(k, AndL(y))
            table.insert(k, x)
        else 
            if x.b then 
                table.insert(k, y)
            else 
                table.insert(k, x)
            end
        end
    elseif tag == 'Assign' then 
        local variable = task.variable
        local value = task.value 
        if value.tag == 'ACon' then
            state[variable] = value.n 
        else 
            table.insert(k, AssignR(variable))
            table.insert(k, value)
        end
    elseif tag == 'Seq' then
        table.insert(k, task.s2)
        table.insert(k, task.s1)
    elseif tag == 'Skip' then
        -- nothing
    elseif tag == 'If' then
        local condition = task.condition 
        local subsequent = task.subsequent
        local alternative = task.alternative
        if condition.tag == 'BCon' then 
            if condition.b then
                table.insert(k, subsequent)
            else 
                table.insert(k, alternative)
            end 
        else 
            table.insert(k, IfC(subsequent, alternative))
            table.insert(k, condition)
        end 
    elseif tag == 'While' then 
        local condition = task.condition
        local body = task.body 
        table.insert(k, If(condition, Seq(body, task), Skip()))
    elseif tag == 'Pgm' then 
        local variables = task.vars 
        local body = task.body 
        if #variables == 0 then
            table.insert(k, body)
        else 
            local variable = variables[#variables]
            table.remove( variables, #variables)
            state[variable] = 0
            table.insert(k, task)
        end
    elseif tag == 'ACon' then
        local n = task
        task = k[#k]
        table.remove(k, #k)

        local tag = task.tag 
        if tag == 'DivL' then
            table.insert(k, Div(n, task.v))
        elseif tag == 'DivR' then 
            table.insert(k, Div(task.v, n))
        elseif tag == 'AddL' then
            table.insert(k, Add(n, task.v))
        elseif tag == 'AddR' then 
            table.insert(k, Add(task.v, n))
        elseif tag == 'LeL' then
            table.insert(k, Le(n, task.v))
        elseif tag == 'LeR' then 
            table.insert(k, Le(task.v, n))
        elseif tag == 'AssignR' then 
            table.insert(k, Assign(task.variable, n))
        else 
            cfg.stuck = true 
        end 
    elseif tag == 'BCon' then 
        local b = task 
        task = k[#k]
        table.remove(k, #k)

        local tag = task.tag 
        if tag == 'NotF' then
            table.insert(k, Not(b))
        elseif tag == 'AndL' then 
            table.insert(k, And(b, task.v))
        elseif tag == 'IfC' then 
            table.insert(k, If(b, task.subsequent, task.alternative))
        else 
            cfg.stuck = true 
        end 
    else
        cfg.stuck = true 
    end
end

function sumPgm(size)
    return Pgm({'n', 'sum'}, Seq(
        Assign('n', ACon(size)),
    Seq(
        Assign('sum', ACon(0)),
        While(Not(Le(AVar('n'), ACon(0))), Seq(
            Assign('sum', Add(AVar('sum'), AVar('n'))),
            Assign('n', Add(AVar('n'), ACon(-1))))))))
end


function run(p)
    cfg = Cfg({p}, {})
    while true do 
        step(cfg)
        if cfg.stuck then
            return cfg
        end
    end
end

function test(size)
    return run(sumPgm(size))
end


function main() 
    local n = 10000000
    local cfg = test(n)
    print("Done execution")
    print('n = ' .. cfg.state['n'])
    print('sum = ' .. cfg.state['sum'])
end 
main()

