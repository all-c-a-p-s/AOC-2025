I've been doing some competitive programming recently, so this year I want to focus on making optimal solutions in terms of time/space complexity.

Complexity of each day-part (where q is number of queries/inputs given, n is size of each input):

| Day | Part | TC                      | SC                   |
|-----|------|-------------------------|----------------------|
| 1   | 1    | O(q)                    | O(1)                 |
| 1   | 2    | O(q)                    | O(1)                 |
| 2   | 1    | O(q * log(n) * sqrt(n)) | O(log(n) * sqrt(n))  |
| 2   | 2    | O(q * log(n) * sqrt(n)) | O(log(n) * sqrt(n))  |
