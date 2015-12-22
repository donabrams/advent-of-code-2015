#![feature(slice_patterns, plugin, box_syntax)]
#![plugin(regex_macros)]
extern crate regex;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;

type GateName = String;

#[derive(Eq)]
#[derive(Hash)]
enum Prev {
	Named (GateName),
	Valued (u16),
}

impl PartialEq for Prev {
	fn eq(&self, other: &Self) -> bool {
		match self {
			&Prev::Named (ref name1) => match other {
				&Prev::Named (ref name2) => name1 == name2,
				&Prev::Valued (..) => false
			},
			&Prev::Valued (ref val1) => match other {
				&Prev::Named (..) => false,
				&Prev::Valued (ref val2) => val1 == val2,
			},
		}
	}
}

#[derive(Eq)]
#[derive(Hash)]
enum Gate {
	Value { name: GateName, a: Prev },
	And { name: GateName, a: Prev, b: Prev },
	Or { name: GateName, a: Prev, b: Prev },
	Lshift { name: GateName, a: GateName, num_bits: u16 },
	Rshift { name: GateName, a: GateName, num_bits: u16 },
	Not { name: GateName, a: GateName },
}

impl PartialEq for Gate {
	fn eq(&self, other: &Self) -> bool {
		let my_name = get_name(self);
		let their_name = get_name(other);
		my_name == their_name
	}
}

type Computer = HashMap<GateName, Gate>;

fn get_value<'a>(computer: &'a Computer, gate: &'a Gate,
                 mut cache: &mut HashMap<&'a Gate, u16>) -> u16{
	let val = match gate {
		&Gate::Value { ref a, .. } => {
			match a {
				&Prev::Named(ref name) => gate_value(&computer, &name, &mut cache),
				&Prev::Valued(val) => val,
			}
		},
		&Gate::And { ref a, ref b, .. } => {
			let left = match a {
				&Prev::Named(ref name) => gate_value(&computer, &name, &mut cache),
				&Prev::Valued(val) => val,
			};
			let right = match b {
				&Prev::Named(ref name) => gate_value(&computer, &name, &mut cache),
				&Prev::Valued(val) => val,
			};
			left & right
		},
		&Gate::Or { ref a, ref b, .. } => {
			let left = match a {
				&Prev::Named(ref name) => gate_value(&computer, &name, &mut cache),
				&Prev::Valued(val) => val,
			};
			let right = match b {
				&Prev::Named(ref name) => gate_value(&computer, &name, &mut cache),
				&Prev::Valued(val) => val,
			};
			left | right
		},
		&Gate::Lshift { ref a, num_bits, ..} => gate_value(&computer, &a, &mut cache) << num_bits,
		&Gate::Rshift { ref a, num_bits, ..} => gate_value(&computer, &a, &mut cache) >> num_bits,
		&Gate::Not { ref a, .. } => std::u16::MAX ^ gate_value(&computer, &a, &mut cache),
	};
	cache.insert(gate, val);
	val
}

fn gate_value<'a>(computer: &'a Computer, name: &GateName,
                  mut cache: &mut HashMap<&'a Gate, u16>) -> u16 {
	match (*computer).get(name) {
		Some(gate) => {
			if cache.contains_key(gate) {
				*cache.get(gate).unwrap()
			} else {
				get_value(&computer, gate, &mut cache)
			}
		}
		None => 0,
	}
}

fn get_name(gate: &Gate) -> GateName {
	match gate {
		&Gate::Value { ref name, .. } => name.clone(),
		&Gate::And { ref name, .. } => name.clone(),
		&Gate::Or { ref name, .. } => name.clone(),
		&Gate::Lshift { ref name, .. } => name.clone(),
		&Gate::Rshift { ref name, .. } => name.clone(),
		&Gate::Not { ref name, .. } => name.clone(),
	}
}

type GateBuilder = fn(&String) -> Option<Gate>;

fn create_gate(spec: String) -> Option<Gate> {
	let gate_builders: Vec<GateBuilder> = vec![
		value_gate_builder,
		and_gate_builder,
		or_gate_builder,
		lshift_gate_builder,
		rshift_gate_builder,
		not_gate_builder,
	];
	let build = move |acc: Option<Gate>, builder: &GateBuilder| {
		match acc {
			None => builder(&spec),
			Some(gate) => Some(gate),
		}
	};
	let gate = gate_builders.iter().fold(None, build);
	if gate.is_none() {
	    println!("OOPS");
	}
	gate
}

static VALUE_GATE_RE : Regex = regex!(r"^(?P<val>[[:alpha:]\d]+) -> (?P<a>[[:alpha:]]+)$");
fn value_gate_builder(spec: &String) -> Option<Gate> {
	match VALUE_GATE_RE.captures(spec) {
		Some(capture) => match &captures_to_vec(capture)[..] {
			[_, a, name] => Some(Gate::Value { 
				name: name.to_string(), 
				a: match a.parse::<u16>() {
					Ok(num) => Prev::Valued(num), 
					Err(_) => Prev::Named(a.to_string()),
				},
			}),
			_ => unreachable!(),
		},
		None => None,
	}
}

static AND_GATE_RE : Regex = regex!(r"^(?P<a>[[:alpha:]\d]+) AND (?P<b>[[:alpha:]\d]+) -> (?P<name>[[:alpha:]]+)$");
fn and_gate_builder(spec: &String) -> Option<Gate> {
	match AND_GATE_RE.captures(spec) {
		Some(capture) => match &captures_to_vec(capture)[..] {
			[_, a, b, name] => Some(Gate::And { 
				name: name.to_string(),
				a: match a.parse::<u16>() {
					Ok(num) => Prev::Valued(num), 
					Err(_) => Prev::Named(a.to_string()),
				}, 
				b: match b.parse::<u16>() {
					Ok(num) => Prev::Valued(num), 
					Err(_) => Prev::Named(b.to_string()),
				},
			}),
			_ => unreachable!(),
		},
		None => None,
	}
}

static OR_GATE_RE : Regex = regex!(r"^(?P<a>[[:alpha:]\d]+) OR (?P<b>[[:alpha:]\d]+) -> (?P<name>[[:alpha:]]+)$");
fn or_gate_builder(spec: &String) -> Option<Gate> {
	match OR_GATE_RE.captures(spec) {
		Some(capture) => match &captures_to_vec(capture)[..] {
			[_, a, b, name] => Some(Gate::Or {
				name: name.to_string(),
				a: match a.parse::<u16>() {
					Ok(num) => Prev::Valued(num), 
					Err(_) => Prev::Named(a.to_string()),
				}, 
				b: match b.parse::<u16>() {
					Ok(num) => Prev::Valued(num), 
					Err(_) => Prev::Named(b.to_string()),
				},
			}),
			_ => unreachable!(),
		},
		None => None,
	}
}

static LSHIFT_GATE_RE : Regex = regex!(r"^(?P<a>[[:alpha:]]+) LSHIFT (?P<num_bits>\d+) -> (?P<name>[[:alpha:]]+)$");
fn lshift_gate_builder(spec: &String) -> Option<Gate> {
	match LSHIFT_GATE_RE.captures(spec) {
		Some(capture) => match &captures_to_vec(capture)[..] {
			[_, a, num_bits, name] => match num_bits.parse::<u16>() {
				Ok(num_bits) => Some(Gate::Lshift {
					name: name.to_string(), 
					a: a.to_string(), 
					num_bits: num_bits,
				}),
				Err(e) => {
					println!("Error: {}", e.to_string());
					None
				},
			},
			_ => unreachable!(),
		},
		None => None,
	}
}

static RSHIFT_GATE_RE : Regex = regex!(r"^(?P<a>[[:alpha:]]+) RSHIFT (?P<num_bits>\d+) -> (?P<name>[[:alpha:]]+)$");
fn rshift_gate_builder(spec: &String) -> Option<Gate> {
	match RSHIFT_GATE_RE.captures(spec) {
		Some(capture) => match &captures_to_vec(capture)[..] {
			[_, a, num_bits, name] => match num_bits.parse::<u16>() {
				Ok(num_bits) => Some(Gate::Rshift {
					name: name.to_string(), 
					a: a.to_string(), 
					num_bits: num_bits,
				}),
				Err(e) => {
					println!("Error: {}", e.to_string());
					None
				},
			},
			_ => unreachable!(),
		},
		None => None,
	}
}

static NOT_GATE_RE : Regex = regex!(r"^NOT (?P<a>[[:alpha:]]+) -> (?P<name>[[:alpha:]]+)");
fn not_gate_builder(spec: &String) -> Option<Gate>{
	match NOT_GATE_RE.captures(spec) {
		Some(capture) => match &captures_to_vec(capture)[..] {
			[_, a, name] => Some(Gate::Not {
				name: name.to_string(), 
				a: a.to_string(),
			}),
			_ => unreachable!(),
		},
		None => None,
	}
}

fn captures_to_vec(captures: regex::Captures) -> Vec<&str> {
	let mut vec: Vec<&str> = Vec::with_capacity(captures.len());
	for i in 0..captures.len() {
		match captures.at(i) {
			Some(str) => vec.push(str),
			None => unreachable!()
		}
	}
	vec
}

fn get_line_vec(filename: &str) -> Result<Vec<String>, io::Error> {
	let f = try!(File::open(filename));
	let vec: Vec<String> = BufReader::new(f).lines()
	    .map(|line| line.unwrap())
	    .collect();
	Ok(vec)
}

fn get_computer() -> Result<Computer, io::Error> {
	let mut computer = Computer::new();
	let mut lines = try!(get_line_vec("gates.txt"));
	while let Some(line) = lines.pop() {
		let gate = create_gate(line);
		match gate {
			Some(gate) => { computer.insert(get_name(&gate), gate); },
			None => { unreachable!(); },
		}
	}
	Ok(computer)
}

fn main() {
	match get_computer() {
		Ok(mut computer) => {
			let val = gate_value(&computer, &("a".to_string()), &mut HashMap::new());
			println!("Original Value of gate a: {}", val);
			computer.insert("b".to_string(), Gate::Value {name: "b".to_string(), a: Prev::Valued(val) });
			let val = gate_value(&computer, &("a".to_string()), &mut HashMap::new());
			println!("New Value of gate a: {}", val);
		}
		Err(e) => println!("Error: {}", e.to_string()),
	}
}
