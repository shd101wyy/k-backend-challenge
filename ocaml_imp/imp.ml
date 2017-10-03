type kitem = 
  | ACon of int
  | AVar of string
  | Div of kitem * kitem
  | Add of kitem * kitem
  | BCon of bool
  | Le of kitem * kitem
  | Not of kitem
  | And of kitem * kitem
  | Assign of string * kitem
  | If of kitem * kitem * kitem
  | While of kitem * kitem
  | Seq of kitem * kitem
  | Skip
  | Pgm of string list * kitem
  | DivL of kitem
  | DivR of kitem
  | AddL of kitem
  | AddR of kitem
  | LeL of kitem
  | LeR of kitem
  | NotF
  | AndL of kitem
  | AssignR of string
  | IfC of kitem * kitem ;;

module State = Map.Make(String) ;;
type cfg = { k: kitem list; state: int State.t } ;;
exception Stuck of cfg ;;

let aresult (a:kitem): bool =  
  match a with
  | ACon _ -> true
  | _ -> false ;;


let bresult (b:kitem): bool =  
  match b with
  | BCon _ -> true
  | _ -> false ;;

let withK k {k = _; state = state} = {k = k; state = state} ;;
  
let step (c:cfg) : cfg =
  match c with
  | {k = AVar i :: rest; state} -> 
    (try 
       let v = State.find i state in withK (ACon v :: rest) c
     with
     | _ -> raise (Stuck c))
  | {k = Div (ACon i, ACon j) :: rest; state} ->
    if j = 0 then raise (Stuck c) else
    withK (ACon (i / j)::rest) c
  | {k = Add (ACon i, ACon j) :: rest; state} ->
    withK (ACon (i+j) :: rest) c
  | {k = Le (ACon i, ACon j) :: rest; state} ->
    withK (BCon (i <= j) :: rest) c
  | {k = Not (BCon b) :: rest; state} ->
    withK (BCon (not b) :: rest) c
  | {k = And (BCon true, b) :: rest; state} ->
    withK (b :: rest) c
  | {k = And (BCon false, _) :: rest; state} ->
    withK (BCon false :: rest) c
  | {k = Assign (i,ACon j) :: rest; state} ->
    {k = rest; state = State.add i j state}
  | {k = Seq (s1, s2) :: rest; state} ->
    withK (s1::s2::rest) c
  | {k = Skip :: rest; state} ->
    withK (rest) c
  | {k = If (BCon true, s, _) :: rest; state} ->
    withK (s::rest) c
  | {k = If (BCon false, _, s) :: rest; state} ->
    withK (s::rest) c
  | {k = While (b, s) :: rest; state} ->
    withK (If (b, Seq (s, While (b, s)), Skip)::rest) c
  | {k = [Pgm (i :: xs, s)]; state} ->
    {k = [Pgm (xs,s)]; state = State.add i 0 state}
  | {k = [Pgm ([], s)]; state} ->
    withK ([s]) c
  (* Heating/cooling rules *)
  (* Heating *)
  | {k = Div (e1, e2)::rest; state} ->
    if not (aresult e1) then
      withK (e1::DivL e2::rest) c
    else if not (aresult e2) then
      withK (e2::DivR e1::rest) c
    else raise (Stuck c)
  | {k = Add (e1, e2)::rest; state} ->
    if not (aresult e1) then
      withK (e1::AddL e2::rest) c
    else if not (aresult e2) then
      withK (e2::AddR e1::rest) c
    else raise (Stuck c)
  | {k = Le (e1, e2)::rest; state} ->
    if not (aresult e1) then
      withK (e1::LeL e2::rest) c
    else if not (aresult e2) then
      withK (e2::LeR e1::rest) c
    else raise (Stuck c)
  | {k = Not b::rest; state} ->
    if not (bresult b) then
      withK (b::NotF::rest) c
    else raise (Stuck c)
  | {k = And (b1, b2)::rest; state} ->
    if not (bresult b1) then
      withK (b1::AndL b2::rest) c
    else raise (Stuck c)
  | {k = Assign (i,e)::rest; state} ->
    if not (aresult e) then
      withK (e::AssignR i::rest) c
    else raise (Stuck c)
  | {k = If (b,s1,s2)::rest; state} ->
    if not (bresult b) then
      withK (b::IfC (s1,s2)::rest) c
    else raise (Stuck c)
  (* Cooling *)
  | {k = (ACon _ as e)::DivL e2::rest; state} ->
    withK (Div (e,e2)::rest) c
  | {k = (ACon _ as e)::DivR e1::rest; state} ->
    withK (Div (e1,e)::rest) c
  | {k = (ACon _ as e)::AddL e2::rest; state} ->
    withK (Add (e,e2)::rest) c
  | {k = (ACon _ as e)::AddR e1::rest; state} ->
    withK (Add (e1,e)::rest) c
  | {k = (ACon _ as e)::LeL e2::rest; state} ->
    withK (Le (e,e2)::rest) c
  | {k = (ACon _ as e)::LeR e1::rest; state} ->
    withK (Le (e1,e)::rest) c
  | {k = (BCon _ as e)::NotF::rest; state} ->
    withK (Not e::rest) c
  | {k = (BCon _ as e)::AndL e2::rest; state} ->
    withK (And (e,e2)::rest) c
  | {k = (ACon _ as e)::AssignR i::rest; state} ->
    withK (Assign (i,e)::rest) c
  | {k = (BCon _ as e)::IfC (s1,s2)::rest; state} ->
    withK (If (e,s1,s2)::rest) c
  | _ -> raise (Stuck c) ;;

let sum_pgm size =
  Pgm (["n"; "sum"],
  Seq (Assign ("n",ACon size),
  Seq (Assign ("sum",ACon 0),
       While (Not (Le (AVar "n", ACon 0)),
         Seq (Assign ("sum",Add (AVar "sum", AVar "n")),
              Assign ("n", Add (AVar "n", ACon (-1)))))))) ;;

let run c =
  try
    let rec go c = go (step c) in go c
  with Stuck c' -> c' ;;

let start (p : kitem) : cfg =
  {k = [p]; state = State.empty} ;;

let print_state variable value = print_string(variable ^ " " ^ (string_of_int value) ^ "\n");;

let print_cfg (c: cfg) = State.iter print_state c.state ;;

let main = 
  let c = start(sum_pgm(10000000)) in
  let start_time = Sys.time() in
  let r = run(c) in
  let end_time = Sys.time() in
    Printf.printf "Execution time: %fs\n" (end_time -. start_time),
    print_cfg r
;;

main