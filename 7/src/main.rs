#![feature(slice_patterns)]
#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;
use std::mem;
use regex::Regex;

type GateName = &'static str;

enum Gate {
	Value { name: GateName, val: i16 },
	And { name: GateName, a: GateName, b: GateName },
	Or { name: GateName, a: GateName, b: GateName },
	Lshift { name: GateName, a: GateName, numBits: i16 },
	Rshift { name: GateName, a: GateName, numBits: i16 },
	Not { name: GateName, a: GateName },
}

type Computer = HashMap<GateName, Gate>;

fn getValue(computer: &Computer, gate: &Gate) -> i16 {
	match *gate {
		Gate::Value { name, val } => val,
		Gate::And { name, a, b } => getGateValue(computer, a) & getGateValue(computer, b),
		Gate::Or { name, a, b } => getGateValue(computer, a) | getGateValue(computer, b),
		Gate::Lshift { name, a, numBits} => getGateValue(computer, a) << numBits,
		Gate::Rshift { name, a, numBits} => getGateValue(computer, a) >> numBits,
		Gate::Not { name, a } => !getGateValue(computer, a),
	}
}

fn getGateValue(computer: &Computer, name: GateName) -> i16 {
	match computer.get(name) {
		Some(gate) => getValue(computer, gate),
		None => 0,
	}
}

fn getName(gate: &Gate) -> GateName {
	match *gate {
		Gate::Value { name, val } => name,
		Gate::And { name, a, b } => name,
		Gate::Or { name, a, b } => name,
		Gate::Lshift { name, a, numBits } => name,
		Gate::Rshift { name, a, numBits } => name,
		Gate::Not { name, a } => name,
	}
}

type GateBuilder = fn(&str) -> Option<Gate>;

fn createGate(gateBuilders: Vec<GateBuilder>, spec: &str) -> Option<Gate> {
	gateBuilders.iter().fold(None, |acc, builder| match acc {
		None => builder(spec),
		Some(gate) => Some(gate),
	})
}

fn map2<T, U, V, F: Fn(T, U) -> V>(f: F, a: Option<T>, b: Option<U>) -> Option<V> {
    match a {
        Some(x) => match b {
            Some(y) => Some(f(x, y)),
            None => None,
        },
        None => None,
    }
}

fn makeStrStatic(s: &str) -> &'static str {
    unsafe {
        let ret = mem::transmute(s as &str);
        mem::forget(s);
        ret
    }
}

static valueGateRe : Regex = regex!(r"^(?P<val>\d+) -> (?P<name>[[:alpha:]]+)$");
fn ValueGateBuild(spec: &str) -> Option<Gate> {
	match valueGateRe.captures(spec) {
		Some(caps) => {
			let name = caps.name("name");
			let val = caps.name("val").and_then(|val: &str| { val.parse::<i16>().ok() });
			let f = |name: &str, val: i16| {
				println!("yay {}, boo {}", name, val);
				Gate::Value { 
					name: makeStrStatic(name), 
					val: val,
				}
			};
			map2(f, name, val)
		},
		None => None,
	}
}
// fn ValueGateBuild(spec: &str) -> Option<Gate> {
// 	match valueGateRe.captures(spec) {
// 		Some(capture) => match &capturesToVec(capture)[..] {
// 			[_, val, name] => match val.parse::<i16>() {
// 				Ok(val) => Some(Gate::Value { 
// 					name: name, 
// 					val: val,
// 				}),
// 				Err(E) => None,
// 			},
// 			_ => None,
// 		},
// 		None => None,
// 	}
// }

static andGateRe : Regex = regex!(r"^(?P<a>[[:alpha:]]+) AND (?P<b>[[:alpha:]]+) -> (?P<name>[[:alpha:]]+)$");
fn AndGateBuild(spec: &str) -> Option<Gate> {
	match andGateRe.captures(spec) {
		Some(capture) => match &capturesToVec(capture)[..] {
			[_, a, b, name] => Some(Gate::And { 
				name: makeStrStatic(name), 
				a: makeStrStatic(a), 
				b: makeStrStatic(b),
			}),
			_ => None,
		},
		None => None,
	}
}

static orGateRe : Regex = regex!(r"^(?P<a>[[:alpha:]]+) OR (?P<b>[[:alpha:]]+) -> (?P<name>[[:alpha:]]+)$");
fn OrGateBuild(spec: &str) -> Option<Gate> {
	match orGateRe.captures(spec) {
		Some(capture) => match &capturesToVec(capture)[..] {
			[_, a, b, name] => Some(Gate::Or {
				name: makeStrStatic(name), 
				a: makeStrStatic(a), 
				b: makeStrStatic(b),
			}),
			_ => None,
		},
		None => None,
	}
}

static lshiftGateRe : Regex = regex!(r"^(?P<a>[[:alpha:]]+) LSHIFT (?P<numBits>\d+) -> (?P<name>[[:alpha:]]+)$");
fn LshiftGateBuild(spec: &str) -> Option<Gate> {
	match lshiftGateRe.captures(spec) {
		Some(capture) => match &capturesToVec(capture)[..] {
			[_, a, numBits, name] => match numBits.parse::<i16>() {
				Ok(numBits) => Some(Gate::Lshift {
					name: makeStrStatic(name), 
					a: makeStrStatic(a), 
					numBits: numBits,
				}),
				Err(E) => None,
			},
			_ => None,
		},
		None => None,
	}
}

static rshiftGateRe : Regex = regex!(r"^(?P<a>[[:alpha:]]+) RSHIFT (?P<numBits>\d+) -> (?P<name>[[:alpha:]]+)$");
fn RshiftGateBuild(spec: &str) -> Option<Gate> {
	match rshiftGateRe.captures(spec) {
		Some(capture) => match &capturesToVec(capture)[..] {
			[_, a, numBits, name] => match numBits.parse::<i16>() {
				Ok(numBits) => Some(Gate::Rshift {
					name: makeStrStatic(name), 
					a: makeStrStatic(a), 
					numBits: numBits,
				}),
				Err(E) => None,
			},
			_ => None,
		},
		None => None,
	}
}

static notGateRe : Regex = regex!(r"^NOT (?P<a>[[:alpha:]]+) -> (?P<name>[[:alpha:]]+)");
fn NotGateBuild(spec: &str) -> Option<Gate>{
	match notGateRe.captures(spec) {
		Some(capture) => match &capturesToVec(capture)[..] {
			[_, a, name] => Some(Gate::Not {
				name: makeStrStatic(name), 
				a: makeStrStatic(a),
			}),
			_ => None,
		},
		None => None,
	}
}

fn capturesToVec(captures: regex::Captures) -> Vec<&str> {
	let mut vec: Vec<&str> = Vec::with_capacity(captures.len());
	for i in 0..captures.len() {
		match captures.at(i) {
			Some(str) => vec.push(str),
			None => unreachable!()
		}
	}
	vec
}

fn go(gateBuilders: Vec<GateBuilder>) -> Result<i16, Box<Error>> {
	let mut computer = Computer::new();
	{
		let mut f = try!(File::open("gates.txt"));
		let mut reader = BufReader::new(f);
		let mut buffer = String::new();
		loop {
			try!(reader.read_line(&mut buffer));
			match createGate(gateBuilders, buffer.as_str()) {
				Some(gate) => {
					computer.insert(getName(&gate), gate);
					continue;
				},
				None => {
					unreachable!()
				},
			}
		}
	}
	getGateValue(&computer, &"a");
}

fn main() {
	let gateBuilders: Vec<GateBuilder> = vec![
		ValueGateBuild, 
		AndGateBuild, 
		OrGateBuild, 
		LshiftGateBuild, 
		RshiftGateBuild, 
		NotGateBuild,
	];
	match go(gateBuilders) {
		Ok(val) => println!("Value of gate a: {}", val),
		Err(e) => println!("Error: {}", e.to_string()),
	}
}
