use std::{
    fs::File,
    io::{read_to_string, stdin, Write},
    time::SystemTime,
};

use itertools_num::linspace;
use nalgebra::{DMatrix, DVector, SymmetricEigen};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

fn main() {
    // Reading system
    let system = System::from_stdin();
    println!("\nParsed system:\n");
    system.show();
    println!();

    // Solving system
    println!("Solving...\n");
    let time = SystemTime::now();
    let answer = system.solve();
    let elapsed = time.elapsed().unwrap().as_secs();
    println!("Solved in {elapsed} s\n");

    // Saving answer
    println!("Saving answer to ./answer.csv\n");
    let mut output = String::from("# Answer\n");
    for row in answer {
        let s = row
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        output.push_str(&s);
        output.push_str("\n");
    }
    let mut file = File::create("answer.csv").unwrap();
    file.write_all(output.as_bytes()).unwrap();
    println!("Done");
}

/// Holds the parameters for solving the system
#[derive(Serialize, Deserialize, Debug)]
struct System {
    n_max: usize,
    resolution: usize,
    initial_guess: f64,
    tol: f64,
    iter: usize,
}

impl System {
    /// Instantiates a `System` from a yaml string in stdin
    fn from_stdin() -> Self {
        let stdin = read_to_string(stdin().lock()).expect("could not read file");
        serde_yaml::from_str(&stdin).expect("could not parse json file")
    }

    /// Prints `System` to stout
    fn show(&self) {
        println!("{self:#?}");
    }

    /// Solves the system
    fn solve(&self) -> Vec<Vec<f64>> {
        // Create local operators

        // a
        let a = DMatrix::from_fn(self.n_max, self.n_max, |i, j| {
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
        let identity = DMatrix::<f64>::identity(self.n_max, self.n_max);

        // compute psi
        let resolution = 500; // Number of points in the range
        let mut psi_mat: Vec<Vec<f64>> = vec![vec![0.0; resolution]; resolution]; // 2D matrix for psi_mat

        // Generate the range of values
        let ti: Vec<f64> = linspace(0.0, 0.05, self.resolution).collect();
        let mui: Vec<f64> = linspace(0.0, 3.0, self.resolution).collect();

        // fill psi_mat matrix using rayon / parallel loop
        psi_mat.par_iter_mut().enumerate().for_each(|(k2, row)| {
            let mu = mui[k2];
            row.iter_mut().enumerate().for_each(|(k1, psi)| {
                let t = ti[k1];
                *psi = self.find_psi(t, mu, &a, &a_dag, &n, &identity).abs();
            });
        });

        psi_mat
    }

    fn find_psi(
        &self,
        t: f64,
        mu: f64,
        a: &DMatrix<f64>,
        a_dag: &DMatrix<f64>,
        n: &DMatrix<f64>,
        identity: &DMatrix<f64>,
    ) -> f64 {
        let mut remaining_iter = self.iter;
        let mut last_guess = self.initial_guess;

        // Get BHMF Hamiltonian
        let bhmf_ham = get_bhmf_ham(t, mu, last_guess, &a, &a_dag, &n, &identity);
        let eigenvector = get_eigen(bhmf_ham);

        // compute psi one time
        let mut psi = eigenvector.transpose() * a * eigenvector;

        while (psi[0] - last_guess).abs() > self.tol && remaining_iter != 0 {
            let bhmf_ham = get_bhmf_ham(t, mu, last_guess, &a, &a_dag, &n, &identity);
            let eigenvector = get_eigen(bhmf_ham);
            last_guess = psi[0];
            psi = eigenvector.transpose() * a * eigenvector;
            remaining_iter -= 1;
        }

        psi[0]
    }
}

/// Builds Hamiltonian
fn get_bhmf_ham(
    t: f64,
    mu: f64,
    psi: f64,
    a: &DMatrix<f64>,
    a_dag: &DMatrix<f64>,
    n: &DMatrix<f64>,
    identity: &DMatrix<f64>,
) -> DMatrix<f64> {
    let term1 = -6.0 * t * (psi * (a + a_dag) - psi.powf(2.0) * identity);
    let term2 = 0.5 * n * (n - identity);
    let term3 = -mu * n;

    term1 + term2 + term3
}

/// Calculates eigenvector
fn get_eigen(ham: DMatrix<f64>) -> DVector<f64> {
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
