#!/usr/bin/env -S ocamlrun ocaml

let parse_ints line = 
	let tokens = String.split_on_char ' ' line in
	let filtered = List.filter (fun s -> s <> "") tokens in
	let values = List.tl filtered in
	List.map int_of_string values

let parse_int line =
	let f s c = match c with '0'..'9' -> s ^ (String.make 1 c) | _ -> s in
	int_of_string (String.fold_left f "" line)

let solve t d =
	let make_even x = x - x mod 2 in
	let disc = t*t - 4*d in
	t - 1 - make_even (t - int_of_float (Float.ceil (Float.sqrt (float_of_int disc))))

let prod = List.fold_left ( * ) 1

let () =
	let ts = read_line () in
	let ds = read_line () in
	Printf.printf "Part 1: %d\n" (prod (List.map2 solve (parse_ints ts) (parse_ints ds)));
	Printf.printf "Part 2: %d\n" (solve (parse_int ts) (parse_int ds));
