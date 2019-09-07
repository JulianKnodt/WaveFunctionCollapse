# WFC

An implementation of the wave function collapse algorithm.

## Divergence From Original

This implementations abandons the original's approach to "symmetry", mainly because I found it
confusing. Instead, I define it more like a puzzle, where pieces edges are compatible with one
another if they have equivalent edges. The edge is a parametrized type which implements Eq (+
Hash for use in HashMaps), so that a user can define a type more easily. I like to think of it
as the sides of a puzzle that just need to be fit together. Some shortcomings of this approach
are that rotational matchings are not defined, os it's assumed that if a piece is rotated, the
side will still match up, which is not always true. There are some ways to fix this, but I
didn't see the value in spending time on it.

In addition, I use a hashmap for locations, in order to possibly define in the future non square
outputs, as well as easily generalize to higher dimensions. The same struct can be used for 1d,
2d, and 3d output, only by defining some type which implements a relationship. I only
implemented it for 2d images, but I'm confident in the implementation for other items.

## Potential Future Optimizations

This solution is intractable for more complicated textures, and also the nature of it makes hard
to parallelize, in that any location might be able to affect any other location. Yet, it might
be the case that there ways to divide the number of remaining locations into separate
components (as in the graph definition of component). If separate components can be identified
quickly, then this algorithm will be more easily parallelizable. Some criterion for separate
components might be if there is a set of tiles which have already been decided completely
separate two tiles, they must be in separate components, or something of the sort.

Another way to make it tractable might be to create large tiles which are compatible with each
other, and combine those in a recursive fashion until the end result is a desired size. Each
step of the process will be independent except for combining, so that might be efficient.

## Credit

Credit to `mxgmn` for the algorithm, as well as all the samples. In addition, a lot of my
understanding for the algorithm came from [Robert Heaton][simple explanation], because his
explanation is very clear.

[simple explanation]: https://robertheaton.com/2018/12/17/wavefunction-collapse-algorithm/
