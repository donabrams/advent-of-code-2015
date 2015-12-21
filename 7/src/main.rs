#![feature(slice_patterns)]
extern crate regex;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;
use regex::Regex;

type GateName = String;

enum Gate {
	Value(GateName, i16), 
	And(GateName, GateName, GateName), 
	Or(GateName, GateName, GateName),
	Lshift(GateName, GateName, i16), 
	Rshift(GateName, GateName, i16), 
	Not(GateName, GateName),
}

type Computer = HashMap<GateName, Gate>;

fn getValue(computer: &Computer, gate: &Gate) -> i16 {
	match *gate {
		Gate::Value(_, ref val) => *val,
		Gate::And(_, ref a, ref b) => getGateValue(computer, &a) & getGateValue(computer, &b),
		Gate::Or(_, ref a, ref b) => getGateValue(computer, &a) | getGateValue(computer, &b),
		Gate::Lshift(_, ref a, ref numBits) => getGateValue(computer, &a) << numBits,
		Gate::Rshift(_, ref a, ref numBits) => getGateValue(computer, &a) >> numBits,
		Gate::Not(_, ref a) => !getGateValue(computer, &a),
	}
}

fn getGateValue(computer: &Computer, name: &str) -> i16 {
	match computer.get(name) {
		Some(gate) => getValue(computer, gate),
		None => 0,
	}
}

fn getName(gate: &Gate) -> GateName {
	match *gate {
		Gate::Value(ref name, _) => name,
		Gate::And(ref name, _, _) => name,
		Gate::Or(ref name, _, _) => name,
		Gate::Lshift(ref name, _, _) => name,
		Gate::Rshift(ref name, _, _) => name,
		Gate::Not(ref name, _) => name,
	}
}

type GateBuilder = fn(Regex, &str) -> Option<Gate>;

fn createGate(&gateBuilders: &Vec<GateBuilder>, &regexes: &Vec<Regex>, spec: &str) -> Option<Gate> {
	gateBuilders.iter().enumerate().fold(None, |acc, (i, builder)| match acc {
		None => builder(regexes[i], spec),
		Some(gate) => Some(gate),
	})
}

fn ValueGateBuild(valueGateRe: Regex, spec: &str) -> Option<Gate> {
	match valueGateRe.captures(spec) {
		Some(capture) => match &capturesToVec(capture)[..] {
			[_, val, name] => match val.parse::<i16>() {
				Ok(val) => Some(Gate::Value(name.to_string(), val)),
				Err(E) => None,
			},
			_ => None,
		},
		None => None,
	}
}

fn AndGateBuild(andGateRe: Regex, spec: &str) -> Option<Gate> {
	match andGateRe.captures(spec) {
		Some(capture) => match &capturesToVec(capture)[..] {
			[_, a, b, name] => Some(Gate::And(name.to_string(), a.to_string(), b.to_string())),
			_ => None,
		},
		None => None,
	}
}

fn OrGateBuild(orGateRe: Regex, spec: &str) -> Option<Gate> {
	match orGateRe.captures(spec) {
		Some(capture) => match &capturesToVec(capture)[..] {
			[_, a, b, name] => Some(Gate::Or(name.to_string(), a.to_string(), b.to_string())),
			_ => None,
		},
		None => None,
	}
}

fn LshiftGateBuild(lshiftGateRe: Regex, spec: &str) -> Option<Gate> {
	match lshiftGateRe.captures(spec) {
		Some(capture) => match &capturesToVec(capture)[..] {
			[_, a, numBits, name] => match numBits.parse::<i16>() {
				Ok(numBits) => Some(Gate::Lshift(name.to_string(), a.to_string(), numBits)),
				Err(E) => None,
			},
			_ => None,
		},
		None => None,
	}
}

fn RshiftGateBuild(rshiftGateRe: Regex, spec: &str) -> Option<Gate> {
	match rshiftGateRe.captures(spec) {
		Some(capture) => match &capturesToVec(capture)[..] {
			[_, a, numBits, name] => match numBits.parse::<i16>() {
				Ok(numBits) => Some(Gate::Rshift(name.to_string(), a.to_string(), numBits)),
				Err(E) => None,
			},
			_ => None,
		},
		None => None,
	}
}

fn NotGateBuild(notGateRe: Regex, spec: &str) -> Option<Gate>{
	match notGateRe.captures(spec) {
		Some(capture) => match &capturesToVec(capture)[..] {
			[_, a, name] => Some(Gate::Not(name.to_string(), a.to_string())),
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

fn go(&regexes: &Vec<Regex>, &gateBuilders: &Vec<GateBuilder>) -> Result<i16, Box<Error>> {
	let mut computer = Computer::new();
	{
		let mut f = try!(File::open("gates.txt"));
		let mut reader = BufReader::new(f);
		let mut buffer = String::new();
		loop {
			try!(reader.read_line(&mut buffer));
			match createGate(&gateBuilders, &regexes, &buffer) {
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
	let valueGateRe: Regex = Regex::new(r"^(?P<val>\d+) -> (?P<name>[[:alpha:]]+)$").unwrap();
	let andGateRe: Regex = Regex::new(r"^(?P<a>[[:alpha:]]+) AND (?P<b>[[:alpha:]]+) -> (?P<name>[[:alpha:]]+)$").unwrap();
	let orGateRe: Regex = Regex::new(r"^(?P<a>[[:alpha:]]+) OR (?P<b>[[:alpha:]]+) -> (?P<name>[[:alpha:]]+)$").unwrap();
	let lshiftGateRe: Regex = Regex::new(r"^(?P<a>[[:alpha:]]+) LSHIFT (?P<numBits>\d+) -> (?P<name>[[:alpha:]]+)$").unwrap();
	let rshiftGateRe: Regex = Regex::new(r"^(?P<a>[[:alpha:]]+) RSHIFT (?P<numBits>\d+) -> (?P<name>[[:alpha:]]+)$").unwrap();
	let notGateRe: Regex = Regex::new(r"^NOT (?P<a>[[:alpha:]]+) -> (?P<name>[[:alpha:]]+)").unwrap();
	let regexes: Vec<Regex> = vec![
		valueGateRe, 
		andGateRe, 
		orGateRe, 
		lshiftGateRe, 
		rshiftGateRe, 
		notGateRe,
	];
	let gateBuilders: Vec<GateBuilder> = vec![
		ValueGateBuild, 
		AndGateBuild, 
		OrGateBuild, 
		LshiftGateBuild, 
		RshiftGateBuild, 
		NotGateBuild,
	];
	match go(&regexes, &gateBuilders) {
		Ok(val) => println!("Value of gate a: {}", val),
		Err(e) => println!("Error: {}", e.to_string()),
	}
}
