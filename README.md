I've been doing some competitive programming recently, so this year I want to focus on making optimal solutions in terms of time/space complexity.

Complexity of each day-part (where q is number of queries/inputs given, n is size of each input):

| Day | Part | TC                      | SC                   |
|-----|------|-------------------------|----------------------|
| 1   | 1    | O(q)                    | O(1)                 |
| 1   | 2    | O(q)                    | O(1)                 |
| 2   | 1    | O(q * log(n) * sqrt(n)) | O(log(n) * sqrt(n))  |
| 2   | 2    | O(q * log(n) * sqrt(n)) | O(log(n) * sqrt(n))  |
| 3   | 1    | O(n * q)                | O(n * q)             |
| 3   | 2    | O(n * q)                | O(n * q)             |
| 4   | 1    | O(n)                    | O(n)                 |
| 4   | 2    | O(n)                    | O(n)                 |
| 5   | 1    | O(n log(n) + q log(n))  | O(n)                 |
| 5   | 2    | O(n log(n))             | O(n)                 |
| 6   | 1    | O(n)                    | O(n)                 |
| 6   | 2    | O(n)                    | O(n)                 |
| 7   | 1    | O(n)                    | O(n)                 |
| 7   | 2    | O(n)                    | O(n)                 |
| 8   | 1    | O(n ^ 2)                | O(n)                 |
| 8   | 2    | O(n ^ 2 * log(n))       | O(n ^ 2)             |
