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
                <img src="/BH.png" alt="BH" class="pb-10" />
                $${`\\mathcal{H}_{\\text{Bose-Hubbard}} = -t \\sum_{i} a_i^\\dagger a_{i+1} + \\frac{U}{2} \\sum_i n_i (n_i - 1) - \\mu \\sum_i n_i
                `}$$
            </div>
            <div class="fragment center"  data-fragment-index="1">
                <img src="/BHMF.png" alt="BH" class="pb-10" />
                Mean field approximation : <br/>
                ${`a_i = \\psi + \\delta_{a_i}`}$<br />
                ${`a_i^\\dagger = \\varpsi + \\delta_{a_i}`}$
                <!-- $${`\\mathcal{H}_{\\text{Mean Field}} =\sum _{i}\left[-\mu {\hat {n}}_{i}+{\frac {U}{2}}{\hat {n}}_{i}({\hat {n}}_{i}-1)-zt(\psi ^{*}{\hat {b}}_{i}+\psi {\hat {b}}_{i}^{\dagger })+zt\psi ^{*}\psi \right]\end{aligned}}}"
                `}$$ -->
            </div>
        </div>

    </Slide>
    <Slide transition='fade'>
        <h3>Phase diagram of Bose-Hubbard model</h3>
    </Slide>
</Slide>