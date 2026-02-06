
![Rust CI](https://github.com/nihcosnosaj/thomas-attractor-rs/actions/workflows/rust.yml/badge.svg)

thomas-attractor-rs (need to find a better name) is a high-performance, real-time chaos visualizer built with Rust and the eframe (egui) framework. It simulates the trajectory of a particle through a 3D vector field defined by the Thomas' Cyclically Symmetric Attractor.

Live Demo LInk HERE

### The Math!

Thomas' Cyclically Symmetric Attractor is a dynamical system with very very high degrees of symmetry. It also looks really cool visually. 

The system is governed by three coupled, non-linear ordinary differential equations:

$$
dtdx​=sin(y)−bx
dtdy​=sin(z)−by
dtdz​=sin(x)−bz
$$

where $b$ represents the dissipation constant.

The most interesting visuals happen when $b$ is at or lower than 0.2081. We can visually see chaos.

I used Euler integration for the trajectory approximation. I've read RK4 is more precise, but I just didn't get the same level of visuals that I was after. 