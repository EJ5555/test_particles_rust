mod solver;
mod plot;

fn main () {
    solver::run_simulation("data/pos_array.csv");
    //plot::create_plot("data/pos_array.dat").unwrap();
}
