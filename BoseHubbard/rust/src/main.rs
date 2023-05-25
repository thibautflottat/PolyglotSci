// usage: `bose_hubbard < system.yaml`
const N_MAX: usize = 50;
const RESOLUTION: usize = 500;
const INITIAL_GUESS: f64 = 1.0;
const TOL: f64 = 1E-4;
const ITER: usize = 500;

use std::{fs::File, io::Write, time::SystemTime};

use itertools_num::linspace;
use nalgebra::{SMatrix, SVector, SymmetricEigen};
use rayon::prelude::*;

fn main() {
    // Reading system
    // let input = read_to_string(stdin().lock()).expect("could not read file");
    // let system = System::from_str(input);
    // println!("\nParsed system:\n");
    // SYSTEM.show();
    println!();

    // Solving system
    println!("Solving...\n");
    let time = SystemTime::now();
    let answer = solve();
    let elapsed = time.elapsed().unwrap().as_secs();
    println!("Solved in {elapsed} seconds\n");

    // Saving answer
    println!("Saving answer to ./answer.csv\n");
    let mut output = String::from("# Answer\n");
    for row in answer {
        let s = row
            .iter()
            .map(|e| format!("{e:.3}"))
            .collect::<Vec<String>>()
            .join(" ");
        output.push_str(&s);
        output.push('\n');
    }
    let mut file = File::create("answer.csv").unwrap();
    file.write_all(output.as_bytes()).unwrap();
    println!("Done");
}

/// Solves the system
fn solve() -> Vec<Vec<f64>> {
    // Create local operators

    // a
    let a: SMatrix<f64, N_MAX, N_MAX> = SMatrix::from_fn(|i, j| {
        if i + 1 == j {
            ((i + 1) as f64).sqrt()
        } else {
            0.0
        }
    });

    // a_dag
    let a_dag = a.transpose();

    // n
    let n = &a_dag * &a;

    // identity
    let identity: SMatrix<f64, N_MAX, N_MAX> = SMatrix::identity();

    // compute psi
    let mut psi_mat: Vec<Vec<f64>> = vec![vec![0.0; RESOLUTION]; RESOLUTION];
    // let mut psi_mat = DMatrix::zeros(self.resolution, self.resolution);

    // Generate the range of values
    let ti: Vec<f64> = linspace(0.0, 0.05, RESOLUTION).collect();
    let mui: Vec<f64> = linspace(0.0, 3.0, RESOLUTION).collect();

    // fill psi_mat matrix using rayon / parallel loop
    psi_mat.par_iter_mut().enumerate().for_each(|(k2, row)| {
        let mu = mui[k2];
        row.iter_mut().enumerate().for_each(|(k1, psi)| {
            let t = ti[k1];
            *psi = find_psi(t, mu, &a, &a_dag, &n, &identity).abs();
        });
    });

    // return
    psi_mat
}

/// Find psi
fn find_psi(
    t: f64,
    mu: f64,
    a: &SMatrix<f64, N_MAX, N_MAX>,
    a_dag: &SMatrix<f64, N_MAX, N_MAX>,
    n: &SMatrix<f64, N_MAX, N_MAX>,
    identity: &SMatrix<f64, N_MAX, N_MAX>,
) -> f64 {
    let mut iter = ITER;
    let mut guess = INITIAL_GUESS;

    // Get BHMF Hamiltonian
    let bhmf_ham = get_bhmf_ham(t, mu, guess, a, a_dag, n, identity);
    let eigenvector = get_eigen(bhmf_ham);

    // compute psi one time
    let mut psi = eigenvector.transpose() * a * eigenvector;

    // converge
    while (psi[0] - guess).abs() > TOL && iter != 0 {
        guess = psi[0];
        let bhmf_ham = get_bhmf_ham(t, mu, guess, a, a_dag, n, identity);
        let eigenvector = get_eigen(bhmf_ham);
        psi = eigenvector.transpose() * a * eigenvector;
        iter -= 1;
    }

    // return
    psi[0]
}

/// Builds Hamiltonian
fn get_bhmf_ham(
    t: f64,
    mu: f64,
    psi: f64,
    a: &SMatrix<f64, N_MAX, N_MAX>,
    a_dag: &SMatrix<f64, N_MAX, N_MAX>,
    n: &SMatrix<f64, N_MAX, N_MAX>,
    identity: &SMatrix<f64, N_MAX, N_MAX>,
) -> SMatrix<f64, N_MAX, N_MAX> {
    let term1 = -6.0 * t * (psi * (a + a_dag) - psi.powf(2.0) * identity);
    let term2 = 0.5 * n * (n - identity);
    let term3 = -mu * n;

    term1 + term2 + term3
}

/// Calculates eigenvector
fn get_eigen(ham: SMatrix<f64, N_MAX, N_MAX>) -> SVector<f64, N_MAX> {
    // Calculate eigenvalues
    let eigen = SymmetricEigen::new(ham);

    // Get eigenvalues & vectors
    let eigenvalues = eigen.eigenvalues;
    let eigenvectors = eigen.eigenvectors;

    // Get smallest eigenvalue & associated eigenvector
    let eigenvalue = eigenvalues.iter().fold(0.0f64, |min, &val| val.min(min));
    let index = eigenvalues.iter().position(|r| *r == eigenvalue).unwrap();

    eigenvectors.column(index).clone_owned()
}
