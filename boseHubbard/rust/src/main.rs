use nalgebra::{DMatrix, DVector, SymmetricEigen};
use iter_num_tools::lin_space;
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn get_bhmf_ham(t: f64, mu: f64, psi: f64, a: &DMatrix<f64>, a_dag: &DMatrix<f64>, n: &DMatrix<f64>, identity: &DMatrix<f64>) -> DMatrix<f64> {

    let term1 = -6.0 * t * (psi * (a + a_dag) - psi.powf(2.0) * identity);
    let term2 = 0.5 * n * (n - identity);
    let term3 = -mu * n;

    let ham = term1 + term2 + term3;
    ham
}

fn get_eigen(ham:DMatrix<f64>) -> DVector<f64> {

    // Calculate eigenvalues
    let eigen = SymmetricEigen::new(ham);

    // Get eigenvalues & vectors
    let eigenvalues = eigen.eigenvalues;
    let eigenvectors = eigen.eigenvectors;

    // Get smallest eigenvalue & associated eigenvector
    let eigenvalue = eigenvalues.iter().fold(0.0f64, |min, &val| if val < min{ val } else{ min });
    let index = eigenvalues.iter().position(|&r| r == eigenvalue).unwrap();
    let eigenvector = eigenvectors.column(index).clone_owned();

    eigenvector
}

fn find_psi(t: f64, mu: f64, a: &DMatrix<f64>, a_dag: &DMatrix<f64>, n: &DMatrix<f64>, identity: &DMatrix<f64>, tol:f64) -> f64 {

    let mut iter = 500;
    let mut initial_guess = 1.0;
    
    // Get BHMF Hamiltonian
    let mut bhmf_ham = get_bhmf_ham(t, mu, initial_guess, &a, &a_dag, &n, &identity);

    // compute psi 
    // compute eigenvector
    let mut eigenvector = get_eigen(bhmf_ham);

    // compute psi one time
    let mut psi = eigenvector.transpose() * a * eigenvector;
    while (psi[0] - initial_guess).abs() > tol && iter != 0 {
        initial_guess = psi[0];
        bhmf_ham = get_bhmf_ham(t, mu, initial_guess, &a, &a_dag, &n, &identity);
        eigenvector = get_eigen(bhmf_ham);
        psi = eigenvector.transpose() * a * eigenvector;
        iter -= 1
    }

    psi[0]
}

fn main() {
    // Define parameters
    let before = Instant::now();
    let n_max = 50; 
    let tol = 1e-4;

    // Create local operators
    // a
    let mut a = DMatrix::zeros(n_max, n_max);
    for i in 0..n_max-1 {
        a[(i, i + 1)] = ((i + 1) as f64).sqrt();
    }
    // a_dag
    let a_dag = a.transpose();
    // n
    let n = &a_dag * &a;
    // identity
    let identity = DMatrix::<f64>::identity(n_max, n_max);

    // compute psi
    let resolution = 500; // Number of points in the range
    let mut psi_mat: Vec<Vec<f64>> = vec![vec![0.0; resolution]; resolution]; // 2D matrix for psi_mat

    // Generate the range of values
    let ti:Vec<f64> = lin_space(0.0..=0.05, resolution).collect();
    let mui:Vec<f64> = lin_space(0.0..=3.0, resolution).collect();

    // fill psi_mat matrix using rayon / parallel loop
    psi_mat
        .par_iter_mut()
        .enumerate()
        .for_each(|(k2, row)| {
            let mu = mui[k2];
            for k1 in 0..resolution {
                let t = ti[k1];
                row[k1] = find_psi(t, mu, &a, &a_dag, &n, &identity, tol).abs();
            }
        });

    println!("Elapsed time: {:.2?}", before.elapsed());
    // Save the psi_mat matrix to a text file
    let mut file = File::create("psi_mat.txt").expect("Failed to create file");
    for row in &psi_mat {
        for element in row {
            write!(file, "{:.2} ", element).expect("Failed to write to file");
        }
        writeln!(file).expect("Failed to write to file");
    }

}