def seive_of_era():
    num = 1000005
    prime_list = []
    p = 2
    prime = [True for i in range(num + 1)]
    while (p * p <= num):
        if ([prime[p]==True]):
            for i in range(p*p, num+1, p):
                prime[i]=False
        p += 1
    for p in range(2,num+1):
        if prime[p]:
            prime_list.append(p)
    return prime_list
            

def functional_seive(num):
    # create a list of all numbers from 2 to n
    numbers = list(range(2, num+1))

    # iterate over the list of numbers
    while numbers:
        # get the next prime number
        prime = numbers[0]

        # remove all multiples of the prime number from the list
        numbers = list(filter(lambda x: x % prime != 0, numbers))

        # yield the prime number
        yield prime