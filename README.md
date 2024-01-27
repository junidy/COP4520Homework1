# COP 4520 Homework 1

## Parallelizing Prime Number Search

The algorithm I implemented to search for primes is the ol' reliable Sieve of Eratosthenes, which has been used to find prime numbers since the Neolithic era.

The steps are as follows (credit to Simple English Wikipedia):

1. Write all the numbers from 2 up to n onto a piece of paper, in order. We will perform the following steps so that all the non-prime numbers will be crossed out, and what's left will be the primes.
2. Choose the first, i.e. the smallest available number. Call it p (it will be 2 at the start). If there's no more available numbers, stop.
3. Count up from p as 2p, 3p, 4p, ..., up to n in steps of p, and cross out each of those numbers. Some numbers will be already crossed out, that's okay. Do not cross out the number p itself but consider it no longer available.
4. Go back to step 2.

The parallelization occurs during step 3, specifically the "crossing out" part. My "sieve" is a vector of *n* booleans, and in this problem *n* is equal to 100,000,000. The rest of the algorithm is not conductive to parallelization, since each iteration must be performed sequentially to guarantee that *p* is a prime.

But the task of marking off the multiples of *p* in the vector introduced an opportunity for safe, painless parallelization. For every iteration of the algorithm, I subdivided the vector into *m* disjoint chunks of length *(n - p) / m*, and assigned each chunk to each of the *m* threads. Then, each of these vectors "marked off" (set to false) the indices of the vector that are a multiple of *p*, by modifying their chunks in place. This approach is very safe, since the sections of memory each thread operates on is guaranteed to be disjoint, and succeeds in equally distributing the work among each thread.

Unfortunately, my solution only achieves a modest improvement in speed (around 130% when I've tested it). Fortunately, my solution does not take longer than a serial approach, so at least I am making a profit off of the overhead introduced by parallelization.
