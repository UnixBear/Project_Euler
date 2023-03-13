# Problem 9

## Problem definition


A Pythagorean triplet is a set of three natural numbers, `a < b < c`, for which,
`a² + b² = c²`

For example, `32 + 42 = 9 + 16 = 25 = 52`.

There exists exactly one Pythagorean triplet for which `a + b + c = 1000`.
Find the product `abc`.

## Solution
The first step is to breaking down Pythagorean triples into component parts vis a vis Euclid's formula, as defined below:

```
for an arbitrary pair of integers m,n where m > n > 0:

a=m² - n²
b=2 * m * n
c=m² + n²
```
always form a Pythagorean triple.

Further, there are a few other facts as candidates for optimization.  

- $\frac{(c-a)(c-b)}{2}$ is always a perfect square, so a function can quickly check this
- further facts can be found [here](https://en.wikipedia.org/wiki/Pythagorean_triple#Elementary_properties_of_primitive_Pythagorean_triples)


Looking at the paramaterization with a third integer k, we can use some math to optimize it via substitution.

- `1000 = a + b + c`
- `1000 = k(m² - n²) + 2kmn + k(m² + n²)`
- `1000 = km² {- kn²} + 2kmn + km² + {kn²}`
- `1000 = 2km² + 2kmn`
- `1000 = 2km(m+n)`
- `500 = km(m+n)`
- `500 = 2² * 5³`


Another fact:

`
if m and n are coprime, and m > n > 0, then m and (m + n) are coprime.
`

```
Proof:

 If a, b, and c are integers such that a is coprime with b and a divides bc, then a must divide c. This property is known as Euclid's Lemma.

Now, let's assume that m and n are coprime positive integers with m > n. We want to show that m is coprime with (m+n).

Suppose that there exists a positive integer k greater than 1 such that k divides both m and (m+n). Then we have:

m = ka
m+n = kb

where a and b are positive integers. Subtracting the first equation from the second, we get:


n = kb - ka = k(b-a)

Since k divides both m and (m+n), it must also divide their difference n. But we know that m and n are coprime, so k cannot be a factor of n unless k = 1.

Therefore, we have shown that if m and n are coprime positive integers with m > n, then m is coprime with (m+n).
```

Another fact: since m > n > 0, m can never be 1 since that n has to be between m and 0 and there are no other integers between 1 and 0.  This means m > 1.  Further, m + n > m since n > 0.

--> m + n > m > 1

Given above algebraic reduction to `500 = 2² * 5³`, we can see that 2 and 5 are both primes and so `2²` and `5³` are coprime.  Since `5³` > `2²` > `1` and coprime, `m` can equal `{1,2,2}²` and `(m + n)` can equal `{1,5,25,125}`. We can discard the `1` possibility because by m > 1.  By checking, `m + n = 5` while `m = 4` which makes `n = 1` and `k = 25`