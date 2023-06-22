<script lang="ts">
	import { Code, Slide } from '$lib';
</script>


<Slide>
    <Slide transition='fade'>
        <h3>Ultra-cold atoms</h3>
        <!-- <video data-autoplay loop onloadstart="this.playbackRate = 1.5;"> -->
        <video id="BEC" autoplay muted class="center">
            <source data-src="/vid/BEC.webm" type="video/webm" />
            <source data-src="/vid/BEC.mp4" type="video/mp4" />
        </video>

    </Slide>
    <Slide transition='fade'>
        <h3>Ultra-cold atoms on optical lattices</h3>
        <div class="r-stack">
            <div class="fragment center fade-out" data-fragment-index="0">
                <img src="/LAT.png" alt="LAT" />
            </div>
            <div class="fragment center current-visible"  data-fragment-index="0">
                <img src="/BH.png" alt="BH" class="pb-10 center" />
                $${`\\mathcal{H}_{\\text{Bose-Hubbard}} = -t \\sum_{i} a_i^\\dagger a_{i+1} + \\frac{U}{2} \\sum_i n_i (n_i - 1) - \\mu \\sum_i n_i
                `}$$
                How to obtain the phase diagram of the Bose-Hubbard model in the (${`t/U, \\mu/U`}$) plane ?
            </div>
            <div class="fragment center current-visible" data-fragment-index="1">
                <img src="/BHMF.png" alt="BH" class="center pb-10" />
                
                <div class="text-left">Mean field approximation:</div> 
                <p class="text-left flex flex-col space-y-4">
                ${`a_i = \\langle a_i \\rangle + \\delta_{a_i}`}$
                ${`a_i^\\dagger = \\langle a_i^\\dagger \\rangle + \\delta_{a_i^\\dagger}`}$
                ${`\\langle a_i \\rangle = \\langle a_i^\\dagger \\rangle ^\\star = \\Phi_i`}$
                ${`\\delta_{a_i}\\delta_{a_i^\\dagger} = 0`}$
                </p>
                <!-- $${`\\mathcal{H}_{\\text{Mean Field}} =\sum _{i}\left[-\mu {\hat {n}}_{i}+{\frac {U}{2}}{\hat {n}}_{i}({\hat {n}}_{i}-1)-zt(\psi ^{*}{\hat {b}}_{i}+\psi {\hat {b}}_{i}^{\dagger })+zt\psi ^{*}\psi \right]\end{aligned}}}"
                `}$$ -->
            </div>
            <div class="fragment center current-visible" data-fragment-index="2">
                <img src="/BHMF.png" alt="BH" class="center pb-10" />
                $${`\\mathcal{H}_{\\text{MF}} = -2t \\sum_{i} (\\Phi (a_i^\\dagger + a_i) - \\Phi^2) + \\frac{U}{2} \\sum_i n_i (n_i - 1) - \\mu \\sum_i n_i
                `}$$
                ${`\\rightarrow`}$ Find ${`\\Phi`}$ which minimizes the free energy of the system.
            </div>

        </div>
    </Slide>
    <Slide transition='fade'>
        <h3>Variational method</h3>
        <ol>
            <li>Generate ${`\\mathcal{H}_{\\text{MF}}`}$ for a ${`(t,\\mu)`}$ pair value and an initial guess ${`\\Phi_{\\text{init}}`}$ (${`=1`}$) ;
            </li>
            <li>Diagonalize ${`\\mathcal{H}_{\\text{MF}}`}$ to find the lowest eigenvalue ${`\\epsilon_0`}$ and associated eigenvector ${`\\phi_0`}$ ;</li>
            <li>Compute ${`\\Phi`}$ such as ${`\\Phi = \\langle \\phi_0 | a | \\phi_0 \\rangle`}$ ;</li>
            <li>Repeat from 1. until convergence ;</li>
            <li>Iterate over each ${`(t,\\mu)`}$.</li>
        </ol>
    </Slide>
    <Slide transition='fade'>
        <h3>Phase diagram</h3>
        <div class="flex flex-row items-center">
        <img class="w-[75%]" alt="BHPD" src="/BHPD.png" />
        <ul class="pl-5">
            <li class="text-red-300 fragment">Python: 120s</li>
            <li class="text-green-300 fragment">Rust: 60s</li>
        </ul>
    </Slide>
</Slide>

<!-- \[
\begin{bmatrix}
1.8 & -1.8 & 0.0 & 0.0 & 0.0 & \dots & 0.0 \\
-1.8 & -0.2 & -2.54558 & 0.0 & 0.0 & \dots & 0.0 \\
0.0 & -2.54558 & -1.2 & -3.11769 & 0.0 & \dots & 0.0 \\
0.0 & 0.0 & -3.11769 & -1.2 & -3.6 & \dots & 0.0 \\
0.0 & 0.0 & 0.0 & -3.6 & -0.2 & \dots & 0.0 \\
\vdots & \vdots & \vdots & \vdots & \vdots & \ddots & \vdots \\
0.0 & 0.0 & 0.0 & 0.0 & 0.0 & \dots & -12.4708 \\
0.0 & 0.0 & 0.0 & 0.0 & 0.0 & \dots & 1033.8 \\
0.0 & 0.0 & 0.0 & 0.0 & 0.0 & \dots & -12.6 \\
0.0 & 0.0 & 0.0 & 0.0 & 0.0 & \dots & 1079.8 \\
0.0 & 0.0 & 0.0 & 0.0 & 0.0 & \dots & -12.7279 \\
0.0 & 0.0 & 0.0 & 0.0 & 0.0 & \dots & 0.0 \\
\end{bmatrix}
\] -->
