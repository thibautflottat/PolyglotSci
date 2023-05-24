use nalgebra::{DMatrix, DVector, SymmetricEigen};
use iter_num_tools::lin_space;
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

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
    // Calculate eigenvalues
    let mut eigenvector = get_eigen(bhmf_ham);
    // let eigen = SymmetricEigen::new(bhmf_ham);

    // // Get eigenvalues & vectors
    // let eigenvalues = eigen.eigenvalues;
    // let eigenvectors = eigen.eigenvectors;

    // // Get smallest eigenvalue & associated eigenvector
    // let eigenvalue = eigenvalues.iter().fold(0.0f64, |min, &val| if val < min{ val } else{ min });
    // let index = eigenvalues.iter().position(|&r| r == eigenvalue).unwrap();
    // let eigenvector = eigenvectors.column(index);

    // compute psi one time
    let mut psi = eigenvector.transpose() * a * eigenvector;
    while psi[0] - initial_guess > tol && iter != 0 {
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
    let n_max = 10; 
    let psi = 1.0;
    let tol = 1e-4;
    let initial_guess = 1.0;


    // Define t and mu ranges
    // for x in (1..10).step_by(2) {
    //     println!("{}", x);
    // }

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

    // compute hamiltonian
    // let ham = get_bhmf_ham(t, mu, psi, &a, &a_dag, &n, &identity);

    // compute psi
    let start = 0.0;
    let stop = 0.05;
    let resolution = 100; // Number of points in the range
    let mut psi_mat_n:DMatrix<f64> = DMatrix::zeros(resolution, resolution);
    let mut psi_mat: Vec<Vec<f64>> = vec![vec![0.0; resolution]; resolution]; // 2D matrix for psi_mat

    // Generate the range of values
    let ti:Vec<f64> = lin_space(0.0..=0.05, resolution).collect();
    let mui:Vec<f64> = lin_space(0.0..=3.0, resolution).collect();

    for (k2, mu) in mui.iter().enumerate() {
        for (k1, t) in ti.iter().enumerate() {
            psi_mat_n[(k2,k1)] = find_psi(*t, *mu, &a, &a_dag, &n, &identity, tol).abs();
        }
    }

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
    // let mui_start = 0.0;
    // let mui_stop = 1.0;
    // let ti_start = 0.0;
    // let ti_stop = 0.05;
    // let resolution = 500; // Number of points in the range
    // let mut psi_mat: Vec<Vec<f64>> = vec![vec![0.0; resolution]; resolution]; // 2D matrix for psi_mat

    // let ti:Vec<f64> = lin_space(0.0..=0.05, resolution).collect();
    // let mui:Vec<f64> = lin_space(0.0..=3.0, resolution).collect();
    // psi_mat
    //     .par_iter_mut()
    //     .enumerate()
    //     .for_each(|(idx, row)| {
    //         let k2 = idx / resolution + 1;
    //         let k1 = idx % resolution + 1;
    //         let mu = mui[k2 - 1];
    //         let t = ti[k1 - 1];
    //         let psi = find_psi(t, mu, &a, &a_dag, &n, &identity, tol);
    //         row[k1 - 1] = psi.abs();
    //     });

    println!("{}", psi_mat_n);
    for row in &psi_mat {
        for element in row {
            print!("{:.2} ", element);
        }
        println!();
    }

    // Save the psi_mat matrix to a text file
    let mut file = File::create("psi_mat.txt").expect("Failed to create file");
    for row in &psi_mat {
        for element in row {
            write!(file, "{:.2} ", element).expect("Failed to write to file");
        }
        writeln!(file).expect("Failed to write to file");
    }

    // Save the psi_mat matrix to a text file
    let mut file = File::create("psi_mat_n.txt").expect("Failed to create file");
    let psi_mat_n_converted: Vec<Vec<f64>> = psi_mat_n.row_iter().map(|row| row.iter().cloned().collect()).collect();
    for row in &psi_mat_n_converted {
        for element in row {
            write!(file, "{:.10} ", element).expect("Failed to write to file");
        }
        writeln!(file).expect("Failed to write to file");
    }


}