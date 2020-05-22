mod grid;
mod solver;

fn main(){

    let mut soduku_grid = grid::generate_grid(26);

    let mut solutions = solver::solve(&soduku_grid);

    while solutions.is_err(){

        soduku_grid = grid::generate_grid(26);
        
        solutions = solver::solve(&soduku_grid);

    }

    println!("{}", soduku_grid);

    for sol in solutions.unwrap(){

        println!("{}", sol);

    }

}