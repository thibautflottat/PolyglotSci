using LinearAlgebra
using Plots
using Distributed

# Truncate Hilbert space
n_max = 50

# Create local operators
a = diagm(1 => sqrt.(collect(range(1, stop=n_max))))
a_dag = transpose(a)
n = a_dag * a
identity = Matrix{Float64}(I, n_max+1, n_max+1)

# Build Hamiltonian
function get_BHMF_ham(t, mu, psi)
    ham = -6*t * (psi * (a + a_dag) - psi^2 * identity) + 0.5 * n * (n - identity) - mu * n
end

# Ground state
function ground_state(ham::Matrix{Float64})
    vals, vecs = eigen(ham)
end

# Find psi
function find_psi(t, mu, initial_guess)
    psi = Inf
    tol = 1e-4
    iter = 500

    while psi - initial_guess > tol || iter == 0

        vals, vecs = ground_state(get_BHMF_ham(t, mu, initial_guess))
        psi = transpose(vecs[:,1]) * a * vecs[:,1]
        initial_guess = psi
        iter -= 1

    end

    return psi
end

#########
resolution = 500
initial_guess = 1
ti = LinRange(0, 0.05, resolution)
mui = LinRange(0, 3, resolution)
psi_mat = zeros(Float64, resolution, resolution)

Threads.@threads for k1 in range(1, resolution)
    t=ti[k1]
    for k2 in range(1, resolution)
        mu=mui[k2]
        psi_mat[k2,k1] = abs(find_psi(t,mu,initial_guess))
    end
end

# Création des deux sous-graphiques
heatmap(psi_mat)