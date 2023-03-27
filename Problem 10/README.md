# Problem 10
## Question
```
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
```

### Solutions
This one will be pretty much all computation, where we'll check every number from 1 to 1,000,000 and see if they're prime. Then, if they are, we'll add that number to some running sum. 

### Primality
We'll be using the Miller-Rabin primality test, which get's a very highly probable guess to whether a number is prime.