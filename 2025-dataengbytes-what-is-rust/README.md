
# What is Rust and what does it have to do with data engineering?

This repository includes an example of customer cohort analysis implemented in both Rust and Python (with polars), demonstrating the performance benefits and ergonomics of Rust for data processing tasks. It was presented at DataEngBytes Wellington in April 2025.

https://www.meetup.com/wellington-data-engineering-meetup/events/306764648/

> 🎤 Tim McNamara Founder, Accelerant.dev
> 
> ## Title
>
> What is Rust and what's it got to do with data engineering?
> 
> ## Summary
>
> Keen to supercharge your data pipelines? This fun, hands-on session introduces the Rust programming language and will show you how it can support your existing data engineering stack to be more robust and easier to deploy, while processing data faster with fewer compute resources.
>
> - We'll start by introducing Rust for building data transformers—and see that it's possible to get something to run in seconds that takes minutes or more in Python.
> - The next step is integrating Rust within data engineering workflows, from Makefiles to newer tools like Airflow, Mage, Dagster and Prefect. Then we'll discover its secret sauce, deployment. 
> - We'll also see if we can cross-compiling Rust-based data utilities to run on multiple architectures (x86, ARM, WebAssembly/WASM) without needing to worry about dependencies and installation.
>
> ## Speaker Bio
> 
> These days, Tim McNamara's attention is focused primarily on supporting teams to adopt Rust - but his software background is actually from data science, Python, and natural language processing. Tim has worked for large internationals such as Amazon Web Services and Canonical, and also local companies like DOT Loves Data and Dragonfly Data Science.
>
> Tim is the founder of a consultancy that assists teams in adopting Rust, a programming language known for its high performance, energy efficiency, reliability, and safety. With experience in Rust since 2017, he published the widely acclaimed book Rust in Action, which introduces the language through various systems programming concepts and techniques. Additionally, he serves as a LinkedIn Learning instructor, creating courses and workshops for learners at all levels. You'll find him online as timClicks on social media. Tim also hosts a YouTube channel and runs a software podcast called [Compose](https://timclicks.dev/compose-podcast).
>
> His mission is to develop technology products and services that help make sense of our world and 



## Rust & Python Cohort Analysis Example


### Setup using Dev Container (Recommended)

This repository includes a `.devcontainer` configuration that sets up a complete environment with:

- Rust (stable) with Cargo and common development tools
- Python 3.10 with data science packages (polars, pandas, etc.)
- uv package manager
- VS Code extensions for Rust and Python development

To use the dev container:

1. Install [VS Code](https://code.visualstudio.com/) and the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
2. Clone this repository
3. Open the repository in VS Code
4. Click on the "Reopen in Container" prompt or run the "Dev Containers: Reopen in Container" command

### Running the examples

#### Rust implementation

```bash
cargo run --bin cohort-analysis
```

#### Python implementation

```bash
cd python-analysis
source .venv/bin/activate
cohort-analysis
```
