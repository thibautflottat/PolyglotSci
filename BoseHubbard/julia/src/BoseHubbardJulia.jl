using LinearAlgebra
using Plots
using Dates
using SharedArrays
using Distributed

#using BenchmarkTools

"Build Hamiltonian"
function get_BHMF_ham(a, a_dag, n, t, mu, psi)
    -6t * (psi * (a + a_dag) - psi^2 * I) + 0.5 * n * (n - I) - mu * n
end

"Find psi"
function find_psi(a, a_dag, n, t, mu, initial_guess; tol=1e-4, iter=500)

    _, vecs = get_BHMF_ham(a, a_dag, n, t, mu, initial_guess) |> eigen
    psi = transpose(vecs[:, 1]) * a * vecs[:, 1]

    while abs(psi - initial_guess) > tol && iter != 0
        initial_guess = psi
        _, vecs = get_BHMF_ham(a, a_dag, n, t, mu, initial_guess) |> eigen
        psi = transpose(vecs[:, 1]) * a * vecs[:, 1]
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
    # psi_mat = Matrix{Float64}(undef, resolution, resolution)
    # psi_mat = SharedArray{Float64}(undef, resolution, resolution)
    psi_mat = SharedArray{Float64, 2}((resolution, resolution))
    # Threads.@threads for k2 in 1:resolution
    @distributed for k2 in 1:resolution
        mu = mui[k2]
        for k1 in 1:resolution
          t = ti[k1]
          psi_mat[k2, k1] = find_psi(a, a_dag, n, t, mu, initial_guess) |> abs
        end
    end

    psi_mat
end

"Création des deux sous-graphiques"
function main()
    s = time()
    println("calculating")
    solution = solve()
    println(time() - s)

    # println("plotting")
    # p = heatmap(solution)
    # display(p)

    # print("press enter to close figure")
    # readline()
end

main()
#@benchmark solve()
