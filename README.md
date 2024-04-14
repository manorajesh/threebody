# threebody
A basic three-body simulation written in Rust. Used [macroquad](https://macroquad.rs/) for graphics.

## How
Each `Body` struct is initialized with position, velocity, acceleration, mass, and force (there are some other values but those are not as relevant).

On each `update`, the force between two bodies is calculated with [Newton's Universal Law of Gravitation](https://en.wikipedia.org/wiki/Newton%27s_law_of_universal_gravitation):
$$F = G \frac{m_1 m_2}{r^2}$$
Since this is a 2-dimensional simulation, the resultant force $F$ is multiplied by the $\Delta x$ and $\Delta y$ to get the $x$ and $y$ components of the force respectively.
Each force acting on the body should be summed:
$$F_{total} = \sum_{j \neq i} F$$

Then, $F=ma$ is used to solve for $a$ or accelration. Rewriting the equation: $a = \frac{F}{m}$

From there, basic  kinematics equations use `dt` or $\Delta t$ to solve for an updated velocity component and position component:
$$v = v_i + at$$
$$p = p_i + vt$$

## Why
I recently watched the [3 Body Problem](https://en.wikipedia.org/wiki/3_Body_Problem_(TV_series)) show.
