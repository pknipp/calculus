Developed from https://dev.to/xinnks/deploy-a-rust-website-on-heroku-1l45

To do:

- handle divergences appropriately
- flesh out readme
- perform char replacement (^, space, exp <-> x) more DRY-ly.
- implement extended rule for an OPEN range of integration
- for root-finding, implement ability to avoid specified roots (using an array?)
- given a single number, compute closest local max or min (w/ability to avoid specified ones?)

ODE:

- implement 1st-order w/following params: /xi/tf/dt/(dx/dt as fn of x and t)
- implement 2nd-order w/following params: /xi/vi/tf/dt/(dv/dt as fn of v, x and t)
