package main

import (
	"fmt"
	"time"
)

type kitem interface{}

type acon struct {
	n int64
}

type avar struct {
	v string
}

type div struct {
	x kitem
	y kitem
}

type add struct {
	x kitem
	y kitem
}

type bcon struct {
	b bool
}

type le struct {
	x kitem
	y kitem
}

type not struct {
	b kitem
}

type and struct {
	x kitem
	y kitem
}

type assign struct {
	variable string
	value    kitem
}

type _if struct {
	condition   kitem
	subsequent  kitem
	alternative kitem
}

type _while struct {
	condition kitem
	body      kitem
}

type seq struct {
	s1 kitem
	s2 kitem
}

type skip struct{}

type pgm struct {
	vars []string
	body kitem
}

type divl struct {
	v kitem
}

type divr struct {
	v kitem
}

type addl struct {
	v kitem
}

type addr struct {
	v kitem
}

type lel struct {
	v kitem
}

type ler struct {
	v kitem
}

type notf struct{}

type andl struct {
	v kitem
}

type assignr struct {
	variable string
}

type ifc struct {
	subsequent  kitem
	alternative kitem
}

type cfg struct {
	k     []kitem
	state map[string]int64
	stuck bool
}

func step(c *cfg) {
	k := c.k
	state := c.state
	if len(k) == 0 {
		c.stuck = true
		return
	}
	task := k[len(k)-1]
	k = k[:len(k)-1]

	// fmt.Println(c)

	switch task.(type) {
	case avar:
		v := task.(avar).v
		if val, ok := state[v]; ok {
			k = append(k, acon{val})
		} else {
			c.stuck = true
		}
	case div:
		x := task.(div).x
		y := task.(div).y
		if _, ok := x.(acon); !ok {
			k = append(k, divl{y}, x)
		} else if _, ok := y.(acon); !ok {
			k = append(k, divr{x}, y)
		} else if y.(acon).n == 0 {
			c.stuck = true
		} else {
			k = append(k, acon{x.(acon).n / y.(acon).n})
		}
	case add:
		x := task.(add).x
		y := task.(add).y
		if _, ok := x.(acon); !ok {
			k = append(k, addl{y}, x)
		} else if _, ok := y.(acon); !ok {
			k = append(k, addr{x}, y)
		} else {
			k = append(k, acon{x.(acon).n + y.(acon).n})
		}
	case le:
		x := task.(le).x
		y := task.(le).y
		if _, ok := x.(acon); !ok {
			k = append(k, lel{y}, x)
		} else if _, ok := y.(acon); !ok {
			k = append(k, ler{x}, y)
		} else {
			k = append(k, bcon{x.(acon).n <= y.(acon).n})
		}
	case not:
		b := task.(not).b
		if _, ok := b.(bcon); ok {
			k = append(k, bcon{!b.(bcon).b})
		} else {
			k = append(k, notf{}, b)
		}
	case and:
		x := task.(and).x
		y := task.(and).y
		if _, ok := x.(bcon); !ok {
			k = append(k, andl{y}, x)
		} else {
			if x.(bcon).b {
				k = append(k, y)
			} else {
				k = append(k, x)
			}
		}
	case assign:
		variable := task.(assign).variable
		value := task.(assign).value
		if _, ok := value.(acon); ok {
			state[variable] = value.(acon).n
		} else {
			k = append(k, assignr{variable}, value)
		}
	case seq:
		k = append(k, task.(seq).s2, task.(seq).s1)
	case skip:
	case _if:
		condition := task.(_if).condition
		subsequent := task.(_if).subsequent
		alternative := task.(_if).alternative
		if _, ok := condition.(bcon); ok {
			if condition.(bcon).b {
				k = append(k, subsequent)
			} else {
				k = append(k, alternative)
			}
		} else {
			k = append(k, ifc{subsequent, alternative}, condition)
		}
	case _while:
		condition := task.(_while).condition
		body := task.(_while).body
		k = append(k, _if{condition, seq{body, task}, skip{}})
	case pgm:
		p := task.(pgm)
		variables := p.vars
		body := p.body
		if len(variables) == 0 {
			k = append(k, body)
		} else {
			variable, variables := variables[0], variables[1:]
			state[variable] = 0
			p.vars = variables
			k = append(k, p)
		}
	case acon:
		n := task
		task, k = k[len(k)-1], k[:len(k)-1]
		switch task.(type) {
		case divl:
			k = append(k, div{n, task.(divl).v})
		case divr:
			k = append(k, div{task.(divr).v, n})
		case addl:
			k = append(k, add{n, task.(addl).v})
		case addr:
			k = append(k, add{task.(addr).v, n})
		case lel:
			k = append(k, le{n, task.(lel).v})
		case ler:
			k = append(k, le{task.(ler).v, n})
		case assignr:
			k = append(k, assign{task.(assignr).variable, n})
		default:
			c.stuck = true
		}
	case bcon:
		b := task
		task, k = k[len(k)-1], k[:len(k)-1]
		switch task.(type) {
		case notf:
			k = append(k, not{b})
		case andl:
			k = append(k, and{b, task.(andl).v})
		case ifc:
			k = append(k, _if{b, task.(ifc).subsequent, task.(ifc).alternative})
		default:
			c.stuck = true
		}
	default:
		c.stuck = true
	}

	c.k = k
	c.state = state

	// fmt.Println("enter here", k)
	// c.stuck = true
}

func sumPgm(size int64) pgm {
	var vars = []string{"n", "sum"}
	var body = seq{assign{"n", acon{size}},
		seq{assign{"sum", acon{0}},
			_while{not{le{avar{"n"}, acon{0}}},
				seq{assign{"sum", add{avar{"sum"}, avar{"n"}}},
					assign{"n", add{avar{"n"}, acon{-1}}}}}}}

	return pgm{
		vars: vars,
		body: body}
}

func run(p pgm) cfg {
	state := make(map[string]int64)
	k := make([]kitem, 0, 5)
	k = append(k, p)
	c := cfg{k: []kitem{p}, state: state, stuck: false}

	for {
		step(&c)
		if c.stuck {
			return c
		}
	}
}

func test(size int64) {
	start := time.Now()
	c := run(sumPgm(size))
	elapsed := time.Since(start)
	fmt.Println(c)
	fmt.Println("Execution time: ", elapsed.Seconds()*1000, " ms")
}

func main() {
	test(10000000)
}
