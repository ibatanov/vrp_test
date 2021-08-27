// #[global_allocator]
// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[macro_use]
extern crate rocket;

use std::io::{BufReader, BufWriter};
use std::sync::Arc;

use rocket::{Build, Rocket};
use rocket::serde::json::{Json};
use serde::{Serialize, Deserialize};
use vrp_pragmatic::core::solver::Builder;
use vrp_pragmatic::core::solver::hyper::StaticSelective;
use vrp_pragmatic::core::utils::Environment;
use vrp_pragmatic::core::models::{Problem as CoreProblem, Solution as CoreSolution};
use vrp_pragmatic::format::problem::{Matrix, Problem, PragmaticProblem};
use vrp_pragmatic::format::solution::{deserialize_solution, Solution, PragmaticSolution};

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index, test])
}

#[derive(Deserialize, Serialize)]
struct Request {
    problem: Problem,
    matrix: Matrix,
}

//ab -p /Users/igor/Desktop/rust/inroute-core/test2.json -T application/json -c 4 -n 10000 http://127.0.0.1:8000/

#[post("/", format = "json", data = "<request>")]
fn index(request: Json<Request>) -> Json<Solution> {
    let environment = Arc::new(Environment::default());
    let core_problem = (request.problem.clone(), vec![request.matrix.clone()]).read_pragmatic();
    let core_problem = Arc::new(core_problem.unwrap());
    let solver = Builder::new(core_problem.clone(), environment.clone())
        .with_max_time(Some(60))
        .with_max_generations(Some(100))
        .with_hyper(Box::new(StaticSelective::new_with_defaults(core_problem.clone(), environment.clone())))
        .build()
        .unwrap();
    let (solution, _cost, _) = solver.solve().unwrap();
    let solution = get_pragmatic_solution(&core_problem, &solution);
    Json(solution)
}

#[get("/test")]
fn test() -> String {
    let a = String::from("qergsdghdhdfhdf dhdf ");
    let _b = String::from("qergsdghdhdfhdf dhdf ");
    let _c = String::from("qergsdghdhdfhdf dhdf ");
    a
}

fn get_pragmatic_solution(problem: &CoreProblem, solution: &CoreSolution) -> Solution {
    let mut buffer = String::new();
    let writer = unsafe { BufWriter::new(buffer.as_mut_vec()) };
    solution.write_pragmatic_json(problem, writer).expect("cannot write pragmatic solution");
    deserialize_solution(BufReader::new(buffer.as_bytes())).expect("cannot deserialize solution")
}