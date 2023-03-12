# Problem 7

## Problem definition

```

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
```

## Solution

The solution is here is implementing the **_Sieve of Eratosthenes_**

Here is the pseudocode lifted straight for it's [wikipedia page](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes):

```algorithm Sieve of Eratosthenes is
    input: an integer n > 1.
    output: all prime numbers from 2 through n.

    let A be an array of Boolean values, indexed by integers 2 to n,
    initially all set to true.

    for i = 2, 3, 4, ..., not exceeding √n do
        if A[i] is true
            for j = i2, i2+i, i2+2i, i2+3i, ..., not exceeding n do
                set A[j] := false

    return all i such that A[i] is true.

```
