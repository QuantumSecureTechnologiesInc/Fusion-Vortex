# Collections Library Implementation Plan

**Date**: 2025-12-07
**Status**: ⏳ In Progress
**Priority**: High (Core functionality)
**Estimated Time**: 4-6 hours

---

## Overview

Implement a comprehensive collections library for Fusion, providing HashMap, HashSet, and Iterator trait - essential data structures for practical application development.

## Architecture

### Module Structure

```text
stdlib/
├── hashmap.fu      # HashMap<K, V> implementation
├── hashset.fu      # HashSetT implementation
└── iterator.fu     # Iterator trait and implementations
```text

### Core Components

1. **HashMap<K, V>** - Key-value hash table
2. **HashSetT** - Unique value set
3. **IteratorT** - Iteration trait
4. **Hash trait** - Hashing interface

---

## Phase 1: Hash Trait and Iterator (1 hour)

### Hash Trait

```fusion
trait Hash {
    fn hash(self) -> int;
}

// Implement for built-in types
impl Hash for int {
    fn hash(self) -> int {
        return self;
    }
}

impl Hash for string {
    fn hash(self) -> int {
        // FNV-1a hash algorithm
        let hash = 2166136261;
        // ... implementation
        return hash;
    }
}
```text

### Iterator Trait

```fusion
trait IteratorT {
    fn next(mut self) -> OptionT;
    fn has_next(self) -> bool;
}

// Helper methods
trait IteratorExtT: IteratorT {
    fn collect(mut self) -> VectorT;
    fn map<U>(mut self, f: fn(T) -> U) -> MapIterator<T, U>;
    fn filter(mut self, f: fn(T) -> bool) -> FilterIteratorT;
    fn count(mut self) -> int;
}
```text

---

## Phase 2: HashMap Implementation (2-3 hours)

### Data Structure

```fusion
class Bucket<K, V> {
    key: K;
    value: V;
    hash: int;
    next: Option<Bucket<K, V>>;  // Chaining for collisions
}

class HashMap<K, V> where K: Hash {
    buckets: Vector<Option<Bucket<K, V>>>;
    size: int;
    capacity: int;
    load_factor: float;
}
```text

### Core Operations

**Insert** - O(1) average:

```fusion
fn insert(mut self, key: K, value: V) -> Option<V> {
    let hash = key.hash();
    let index = hash % self.capacity;

    // Handle collision with chaining
    // Return old value if key exists
}
```text

**Get** - O(1) average:

```fusion
fn get(self, key: K) -> Option<V> {
    let hash = key.hash();
    let index = hash % self.capacity;
    // Linear probe or chain walk
}
```text

**Remove** - O(1) average:

```fusion
fn remove(mut self, key: K) -> Option<V> {
    // Remove and return old value
}
```text

**Resize** - O(n):

```fusion
fn resize(mut self) {
    // Double capacity when load factor > 0.75
    // Rehash all entries
}
```text

### Additional Methods

- `contains_key(key: K) -> bool`
- `is_empty() -> bool`
- `len() -> int`
- `clear()`
- `keys() -> KeyIterator<K>`
- `values() -> ValueIterator<V>`
- `entries() -> EntryIterator<K, V>`

---

## Phase 3: HashSet Implementation (1 hour)

### Data Structure

```fusion
class HashSetT where T: Hash {
    map: HashMap<T, bool>;  // Use HashMap internally
}
```text

### Core Operations

All operations delegate to internal HashMap:

```fusion
fn insert(mut self, value: T) -> bool {
    return self.map.insert(value, true).is_none();
}

fn contains(self, value: T) -> bool {
    return self.map.contains_key(value);
}

fn remove(mut self, value: T) -> bool {
    return self.map.remove(value).is_some();
}
```text

### Set Operations

- `union(other: HashSetT) -> HashSetT`
- `intersection(other: HashSetT) -> HashSetT`
- `difference(other: HashSetT) -> HashSetT`
- `is_subset(other: HashSetT) -> bool`
- `is_superset(other: HashSetT) -> bool`

---

## Phase 4: Testing (1 hour)

### Unit Tests

```fusion
fn test_hashmap_insert() {
    let mut map = HashMap::<int, string>::new();
    map.insert(1, "one");
    map.insert(2, "two");

    assert(map.get(1) == Some("one"));
    assert(map.get(2) == Some("two"));
    assert(map.get(3) == None);
}

fn test_hashmap_collision() {
    // Test hash collision handling
}

fn test_hashset_operations() {
    let mut set = HashSet::<int>::new();
    assert(set.insert(1) == true);
    assert(set.insert(1) == false);  // Duplicate
    assert(set.contains(1) == true);
}

fn test_iterator() {
    let map = HashMap::from([(1, "a"), (2, "b")]);
    let count = map.keys().count();
    assert(count == 2);
}
```text

---

## Implementation Strategy

### Step 1: Core Infrastructure (30 min)

- Create stub files
- Define traits (Hash, Iterator)
- Basic type signatures

### Step 2: HashMap (2 hours)

- Bucket structure
- Basic operations (insert, get, remove)
- Resize logic
- Iterator implementations

### Step 3: HashSet (1 hour)

- Wrapper around HashMap
- Set-specific operations
- Iterator delegation

### Step 4: Testing & Polish (1 hour)

- Unit tests
- Edge cases
- Documentation
- Examples

---

## Design Decisions

### Hash Algorithm

Use **FNV-1a** for strings:

- Fast
- Good distribution
- Simple to implement

### Collision Resolution

Use **separate chaining**:

- Simpler than open addressing
- Better for high load factors
- Easier to implement

### Load Factor

Use **0.75** as threshold:

- Industry standard
- Good balance of space/time

### Initial Capacity

Use **16** buckets:

- Powers of 2 for efficient modulo
- Reasonable starting size

---

## Success Criteria

- [ ] HashMap insert, get, remove work correctly
- [ ] HashMap handles collisions properly
- [ ] HashMap resizes when load factor exceeded
- [ ] HashSet wraps HashMap correctly
- [ ] HashSet set operations work
- [ ] Iterator trait implemented
- [ ] All tests pass
- [ ] Zero memory leaks (via borrow checker)

---

## Timeline

| Phase     | Task                     | Time          |
| :-------- | :----------------------- | :------------ |
| 1         | Hash & Iterator traits   | 1 hour        |
| 2         | HashMap implementation   | 2-3 hours     |
| 3         | HashSet implementation   | 1 hour        |
| 4         | Testing & polish         | 1 hour        |
| **Total** | **Complete Collections** | **5-6 hours** |

---

**Status**: Ready to implement
**Next Step**: Create hash trait and iterator infrastructure