using LinearAlgebra
using Plots

"Build Hamiltonian"
function get_BHMF_ham(a, a_dag, n, t, mu, psi)
    -6t * (psi * (a + a_dag) - psi^2 * I) + 0.5 * n * (n - I) - mu * n
end

"Find psi"
function find_psi(a, a_dag, n, t, mu, initial_guess; tol=1e-4, iter=500)
    psi = Inf

    while psi - initial_guess > tol || iter == 0
        _, vecs = get_BHMF_ham(a, a_dag, n, t, mu, initial_guess) |> eigen
        psi = transpose(vecs[:, 1]) * a * vecs[:, 1]
        initial_guess = psi
        iter -= 1
    end

    psi
end

"Solves"
function solve(; n_max=50, resolution=500, initial_guess=1)
    # Create local operators
    a = diagm(1 => sqrt.(1:n_max))
    a_dag = transpose(a)
    n = a_dag * a

    ti = range(start=0, stop=0.05, length=resolution)
    mui = range(start=0, stop=3, length=resolution)
    psi_mat = zeros(Float64, resolution, resolution)

    for k2 in 1:resolution, k1 in 1:resolution
        t = ti[k1]
        mu = mui[k2]
        psi_mat[k2, k1] = find_psi(a, a_dag, n, t, mu, initial_guess) |> abs
    end

    psi_mat
end

"Création des deux sous-graphiques"
function main()
    println("calculating")
    solution = solve()

    println("plotting")
    p = heatmap(solution)
    display(p)

    print("press enter to close figure")
    readline()
end

main()
