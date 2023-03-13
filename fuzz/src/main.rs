#![feature(is_sorted)]
#![feature(exact_size_is_empty)]
#![feature(iter_advance_by)]
#![feature(variant_count)]
mod generation;
mod place;
mod ptable;
mod ty;

use std::env::args;

use crate::generation::GenerationCtx;
use mir::{serialize::Serialize, syntax::*};
use petgraph::dot::Dot;
use rand::{rngs::SmallRng, SeedableRng};

fn main() {
    env_logger::init();

    // let mut body = Body::new(&vec![Ty::Bool], Ty::Bool);
    // let statements = vec![Statement::Assign(
    //     Place::RETURN_SLOT,
    //     Rvalue::UnaryOp(
    //         UnOp::Not,
    //         Operand::Copy(Place {
    //             local: body.args_iter().next().unwrap(),
    //         }),
    //     ),
    // )];
    // let terminator = Some(Terminator::Return);
    // body.new_basic_block(BasicBlockData {
    //     statements,
    //     terminator,
    // });
    // let mut program = Program::new();
    // program.push_fn(body);

    // println!("{}", program.serialize());

    // let mut rng = SmallRng::seed_from_u64(0);
    // let skeleton = build_skeleton(&mut rng);
    // let dot = Dot::new(&skeleton);
    // println!("{dot:?}");

    let seed: u64 = args().skip(1).next().unwrap().parse().unwrap();
    let genctxt = GenerationCtx::new(seed);
    let program = genctxt.generate();
    println!("{}", program.serialize());
}
