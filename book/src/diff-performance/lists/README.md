# Lists

The `Diffable` implementation for standard library lists such as `Vec<T>` and `[T; N]` relies on a
dynamic programming solution to the [longest common subsequence][lcs] problem.

This means that delta encoding lists has a time complexity of `O(M * N)`, where `M` and `N` are
the lengths of the before and after lists.

When your lists are small this is unlikely to be a performance bottleneck.

However, if your application deals with lots of large lists and you have benchmarked that delta encoding
your lists is a performance bottleneck, consider making use of a [changed flag](../changed-flags).

[lcs]: https://en.wikipedia.org/wiki/Longest_common_subsequence_problem
