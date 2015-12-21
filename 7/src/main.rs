#![feature(slice_patterns)]
extern crate regex;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;

type GateName = String;

type Computer = HashMap<GateName, Gate>;

fn getGateValue(computer: &Computer, name: &GateName) {
	computer.get(name).getValue(computer);
}

const gateBuilders: Vec<fn(&str) -> Option<Gate>> = vec![
	ValueGateBuild, 
	AndGateBuild, 
	OrGateBuild, 
	LshiftGateBuild, 
	RshiftGateBuild, 
	NotGateBuild,
];

fn createGate(spec: &str) -> Option<Gate> {
	gateBuilders.iter().fold(None, |acc, builder| match acc {
		None => builder(spec),
		Some(gate) => Some(gate),
	});
}

trait Gate {
	fn getValue(&self, &Computer) -> i16;
	fn getName(&self) -> GateName {
		self.name;
	}
}

struct ValueGate {
	name: GateName,
	val: i16,
}
impl Gate for ValueGate {
	fn getValue(&self, computer: &Computer) -> i16 {
		self.val;
	}
}
const valueGateRe: Regex = Regex::new(r"^(?P<val>\d+) -> (?P<out>[[:alpha:]]+)$").unwrap();
fn ValueGateBuild(spec: &str) -> Option<Gate> {
	match valueGateRe.captures(spec) {
		Some([val, out]) => Some(ValueGate {
			name: out,
			val: try!(val.parse::<i16>()),
		}),
		None => None,
	}
}

struct AndGate {
	name: GateName,
	a: GateName,
	b: GateName,
}
impl Gate for AndGate {
	fn getValue(&self, computer: &Computer) -> i16 {
		getGateValue(computer, self.a) & getGateValue(computer, self.b);
	}
}
const andGateRe: Regex = Regex::new(r"^(?P<a>[[:alpha:]]+) AND (?P<b>[[:alpha:]]+) -> (?P<out>[[:alpha:]]+)$").unwrap();
fn AndGateBuild(spec: &str) -> Option<Gate> {
	match andGateRe.captures(spec) {
		Some([a, b, out]) => Some(AndGate{
			a: a,
			b: b,
			name: out,
		}),
		None => None,
	}
}

struct OrGate {
	name: GateName,
	a: GateName,
	b: GateName,
}
impl Gate for OrGate {
	fn getValue(&self, computer: &Computer) -> i16 {
		getGateValue(computer, self.a) | getGateValue(computer, self.b);
	}
}
const orGateRe: Regex = Regex::new(r"^(?P<a>[[:alpha:]]+) OR (?P<b>[[:alpha:]]+) -> (?P<out>[[:alpha:]]+)$").unwrap();
fn OrGateBuild(spec: &str) -> Option<Gate> {
	match orGateRe.captures(spec) {
		Some([a, b, out]) => Some(OrGate {
			a: a, 
			b: b,
			name: out,
		}),
		None => None,
	}
}

struct LshiftGate {
	name: GateName,
	a: GateName,
	numBits: i16,
}
impl Gate for LshiftGate {
	fn getValue(&self, computer: &Computer) -> i16 {
		getGateValue(computer, self.a) << self.numBits
	}
}
const lshiftGateRe: Regex = Regex::new(r"^(?P<a>[[:alpha:]]+) LSHIFT (?P<numBits>\d+) -> (?P<out>[[:alpha:]]+)$").unwrap();
fn LshiftGateBuild(spec: &str) -> Option<Gate> {
	match lshiftGateRe.captures(spec) {
		Some([a, numBits, out]) => Some(LshiftGate {
			a: a,
			numBits: try!(numBits.parse::<i16>()),
			name: out
		}),
		None => None,
	}
}

struct RshiftGate {
	name: GateName,
	a: GateName,
	numBits: i16,
}
impl Gate for RshiftGate {
	fn getValue(&self, computer: &Computer) -> i16 {
		getGateValue(computer, self.a) >> self.numBits
	}
}
const rshiftGateRe: Regex = Regex::new(r"^(?P<a>[[:alpha:]]+) RSHIFT (?P<numBits>\d+) -> (?P<out>[[:alpha:]]+)$").unwrap();
fn RshiftGateBuild(spec: &str) -> Option<Gate>{
	match rshiftGateRe.captures(spec) {
		Some([a, numBits, out]) => Some(RshiftGate {
			a: a,
			numBits: try!(numBits.parse::<i16>()),
			name: out
		}),
		None => None,
	}
}

struct NotGate {
	name: GateName,
	a: GateName,
}
impl Gate for NotGate {
	fn getValue(&self, computer: &Computer) -> i16 {
		!getGateValue(computer, self.a)
	}
}
const notGateRe: Regex = Regex::new(r"^NOT (?P<a>[[:alpha:]]+) -> (?P<out>[[:alpha:]]+)").unwrap();
fn NotGateBuild(spec: &str) -> Option<Gate>{
	match notGateRe.captures(spec) {
		Some([a, out]) => Some(NotGate {
			a: a,
			name: out
		}),
		None => None,
	}
}

fn main() {
	let mut computer = Computer::new();
	{
		let mut f = try!(File::open("gates.txt"));
		let mut reader = BufReader::new(f);
		let mut buffer = String::new();
		loop {
			// read a line into buffer
			try!(reader.read_line(&mut buffer));
			match createGate(buffer) {
				Some(gate) => {
					computer.add(gate);
				}
				None => None
			}
		}
	}
	println!("Value of gate a: {}", getGateValue(computer, "a"));
}
