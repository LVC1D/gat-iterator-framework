# GAT Iterator Framework

Week 4, Day 1 - RBGD Curriculum

## Project Goal
Build a flexible iterator framework using Generic Associated Types (GATs) to demonstrate:
- Zero-cost abstractions
- Type-safe iteration patterns
- Advanced lifetime handling

## Status
🚧 In Progress - Day 1

## Progress Log
- Initial project setup
## Implementation Status
✅ LendingIterator trait defined with GATs
✅ WindowIterator: Sliding windows over slices
✅ Test coverage: 62.5% (documented tarpaulin quirk with generic code)
✅ Replication test: Passed in <20 minutes

## What I Learned
- GATs enable associated types with lifetime parameters
- The `where Self: 'a` bound connects item lifetime to iterator lifetime
- Lending iterators enable zero-copy iteration patterns
- Fixed trait-level lifetimes conflict with Rust's aliasing XOR mutability rule

## Key Insights
- Replication from scratch solidifies learning
- Test-driven development catches logic bugs early
- Documentation with doctests serves as living examples

## Next Steps
- Week 4, Day 2: Apply HRTB patterns with Git workflow
