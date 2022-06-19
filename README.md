Developed from https://dev.to/xinnks/deploy-a-rust-website-on-heroku-1l45

To do:

- handle divergences appropriately
- flesh out readme
- perform char replacement (^, space, exp <-> x) more DRY-ly.
- implement extended rule for an OPEN range of integration
- for root-finding, implement ability to avoid specified roots (using an array?)
- given a single number, compute a local max (not necessarily the closest one), with the ability to avoid specified ones through the use of some sort of deflation.  If they want min, simply multiply function by -1.
- implement json results for ODE & ODE2.
- render ODE & ODE2 results graphically
