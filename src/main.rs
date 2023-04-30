mod solver;
mod plot;

fn main () {
    let res = solver::solve();
    plot::create_plot(res).unwrap();
}
