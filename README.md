Developed from https://dev.to/xinnks/deploy-a-rust-website-on-heroku-1l45
To do:
Implement open simpson's rule
Allow unary functions (basic, trig, log, etc)pting
Closed interval extended simpson's rule overview:
* uniformly sized subintervals
* number of subintervals = 2^n (n >= 1)
* wts = 1-4-2-4-...-2-4-1
Pseudo-code:
pt struct has three fields: x, f, and wt
Start w/1 points (1 regular + 1 special), for which wts are 1&1
initialize dx as xf - xi
initialize integral as (xi.f * xi.wt + xf.f * xf.wt) * dx
initialize integral_new as infinity
doubling loop (while abs(integral - integral_new) > eps):
    integral_new = 0.
    dx /= 2
    for x in regular points:
        if x.wt == 4:
            x.wt = 2
        integral_new += x.f * x.wt
        create x_new
        x_new.wt = 4
        x_new.x = x.x + dx
        x_new.f = function evaluation at x_new.f
        integral_new += x_new.wt * x_new.wt
        push x_new on end of vector (w/out disrupting current loop)
    integral_new *= dx
    
end of pseudocode
