<script lang="ts">
	import { Code, Slide } from '$lib';
</script>

<Slide>
	<Slide id="fragments">
		<h3 class="pt-[2ex]">Agenda</h3>
		<ol class="pt-[2ex]">
			<li class="pt-[1ex]">Memory safety and undefined behavior</li>
			<li class="pt-[1ex]">The Rust programming language</li>
			<li class="pt-[1ex]">
				Embedding Rust into a scientific Python project as a proof of concept:
			</li>
			<ul>
				<li>Linear algebra</li>
				<li>Variational method</li>
				<li>Parallelization</li>
			</ul>
			<li class="fragment pt-[1ex]">Transform eveyone in the room into a Rustacean</li>
		</ol>
	</Slide>
	<Slide>
		<h3>Memory safety</h3>
		<ul>
			<li class="pt-[1ex]">Microsoft: 70 percent of all security bugs are memory safety issues</li>
			<ul>
				<li>Not even world class C++ programmers can avoid making these mistakes</li>
			</ul>
			<div class="fragment">
				<li class="pt-[1ex]">
					Tony Hoare (most famous for his Quicksort algorithm) in 1964, stated his invention of null
					references was a “billion dollar mistake”
				</li>
				<li class="pt-[1ex]">Memory leaks, allocation issues</li>
				<ul>
					<li>Possible alternative: garbage collection, but performance tradeoff</li>
				</ul>
			</div>
			<li class="pt-[3ex] fragment">Worst than a crash: silent bug</li>
			<div class="fragment">
				<li class="pt-[3ex]">Why does it matter to us scientists?</li>
				<ul>
					<li>We rely on low-level code for paralellization and performance</li>
					<li>Silent errors give us wrong results</li>
				</ul>
			</div>
		</ul>
	</Slide>
	<Slide transition="fade">
		<h3>The Rust programming language</h3>
		<ul>
			<li>A system/general purpose programming language</li>
			<li>Why Rust?</li>
		</ul>
		<div class="r-stack">
			<div class="container fragment">
				<div class="col">
					<p>Performance</p>
					<ul>
						<li>No runtime</li>
						<li>No garbage collection</li>
					</ul>
				</div>
				<div class="col fragment">
					<p>Reliability</p>
					<ul>
						<li>Memory safety</li>
						<li>Thread safety</li>
						<li>Features built in the language</li>
						<li>Checks at compile time</li>
					</ul>
				</div>
				<div class="col fragment">
					<p>Productivity</p>
					<ul>
						<li>Documentation</li>
						<li>Error messages</li>
						<li>Tools</li>
					</ul>
				</div>
			</div>
		</div>
	</Slide>
	<Slide transition="fade">
		<h3>The Rust programming language</h3>
		<ul>
			<li>A system/general purpose programming language</li>
			<li>Why Rust?</li>
		</ul>
			<div class="container">
				<div class="col">
					<img class="center" src="/img/survey.png" alt="survey" />
				</div>
				<div class="col">
					<img class="top fragment" src="/img/linux.png" alt="linux" />
					<img class="bottom fragment" src="/img/azure.png" alt="azure" />
				</div>
			</div>
	</Slide>
	<Slide>
		<h3>The Option type</h3>
		<ul>
			<li>Rust doesn't have a null type, it has Option(type)</li>
			<li class="fragment">We can't use the Option type directly</li>
			<ul class="fragment">
				<li>Unwrap the option with possibility of runtime crash</li>
				<li>Deal with the error inside the code</li>
			</ul>
			<li class="fragment">No more silent bugs!</li>
		</ul>
	</Slide>
	<Slide transition="fade">
		<h3>The borrowing system</h3>
		<Code trim lineNumbers language="rust">
			{@html `
fn main() {
    let hello_world = String::from("Hello ");

    append_world(hello_world);

    println!("{hello_world}");
}

fn append_world(input: String) {
    input.push_str("World!");
}
        `}
		</Code>
		<div class="fragment">
			<Code trim lineNumbers>
				{@html `
error[E0596]: cannot borrow \`input\` as mutable, as it is not declared as mutable
  --> src/main.rs:10:5
   |
10 |     input.push_str("World!");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
   |
help: consider changing this to be mutable
   |
9  | fn append_world(mut input: String) {
   |                 +++
        `}
			</Code>
		</div>
	</Slide>
	<Slide transition="fade">
		<h3>The borrowing system</h3>
		<Code trim lineNumbers language="Rust">
			{@html `
fn main() {
    let hello_world = String::from("Hello ");

    append_world(hello_world);

    println!("{hello_world}");
}

fn append_world(mut input: String) {
    input.push_str("World!");
}
        `}
		</Code>
		<div class="fragment">
			<Code trim lineNumbers>
				{@html `
error[E0382]: borrow of moved value: \`hello_world\`
 --> src/main.rs:6:15
  |
2 |     let hello_world = String::from("Hello ");
  |         ----------- move occurs because \`hello_world\` has type \`String\`, which does not implement the \`Copy\` trait
3 |
4 |     append_world(hello_world);
  |                  ----------- value moved here
5 |
6 |     println!("{hello_world}");
  |               ^^^^^^^^^^^^^ value borrowed here after move
  |
note: consider changing this parameter type in function \`append_world\` to borrow instead if owning the value isn't necessary
        `}
			</Code>
		</div>
	</Slide>
	<Slide transition="fade">
		<h3>The borrowing system</h3>
		<Code trim lineNumbers language="Rust">
			{@html `
fn main() {
    let mut hello_world = String::from("Hello ");

    append_world(&mut hello_world);

    println!("{hello_world}");
}

fn append_world(input: &mut String) {
    input.push_str("World!");
}
        `}
		</Code>
		<div class="container">
			<div class="fragment center">
				<Code trim lineNumbers>
					{@html `
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running \`target/debug/mock\`
Hello World!
        `}
				</Code>
			</div>
		</div>
	</Slide>
	<Slide>
		<h3>Maturity</h3>
		<div class="container fragment">
			<div class="col">
				<img class="center" src="/img/web.png" alt="web" />
			</div>
			<div class="col">
				<img class="center fragment" src="/img/learn.png" alt="learn" />
			</div>
		</div>
	</Slide>
</Slide>

<style>
	.container {
		display: flex;
	}
	.col {
		flex: 1;
	}
</style>
