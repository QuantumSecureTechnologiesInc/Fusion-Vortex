//! Fusion Garbage Collector (Polyglot Heap)
//! Addresses: Limited garbage collection.
//! 
//! While Fusion primarily uses the Vortex borrow checker for zero-cost memory safety,
//! interacting with managed runtimes (Python/JS) via Supernova requires a GC tier.
//! This implements a basic Mark-and-Sweep memory manager.
use crate::types::*;
use std::collections::HashMap;

/// Represents a header for a dynamically allocated managed object.
#[derive(Clone, Debug)]
pub struct GcHeader {
    pub size: FSize,
    pub is_marked: FBool,
    pub type_tag: FString, // Used for runtime reflection later
}

/// A simulated managed memory block.
#[derive(Clone, Debug)]
pub struct GcObject {
    pub id: FSize,
    pub header: GcHeader,
    // Payload data would go here
    pub references: FVec<FSize>, // IDs of other managed objects this points to
}


pub struct PolyglotHeap {
    objects: FMap<FSize, GcObject>,
    roots: FVec<FSize>, // Known active variables from the stack/registers
    next_id: FSize,
    bytes_allocated: FSize,
    threshold: FSize,
}

impl PolyglotHeap {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            roots: Vec::new(),
            next_id: 1,
            bytes_allocated: 0,
            threshold: 1024 * 1024, // 1MB initial GC threshold
        }
    }

    /// Allocates a new managed object on the polyglot heap.
    pub fn allocate(&mut self, size: FSize, type_tag: FString) -> FSize {
        if self.bytes_allocated + size > self.threshold {
            self.collect_garbage();
        }

        let id = self.next_id;
        self.next_id += 1;

        let obj = GcObject {
            id,
            header: GcHeader {
                size,
                is_marked: false,
                type_tag,
            },
            references: Vec::new(),
        };

        self.objects.insert(id, obj);
        self.bytes_allocated += size;
        
        id
    }

    /// Adds an object ID to the root set (e.g., a local variable).
    pub fn add_root(&mut self, id: FSize) {
        if !self.roots.contains(&id) {
            self.roots.push(id);
        }
    }

    /// Removes an object ID from the root set (e.g., variable out of scope).
    pub fn remove_root(&mut self, id: FSize) {
        self.roots.retain(|&r| r != id);
    }

    /// Runs a full Mark-and-Sweep garbage collection cycle.
    pub fn collect_garbage(&mut self) {
        self.mark();
        self.sweep();
        
        // Dynamically adjust threshold
        self.threshold = (self.bytes_allocated * 2).max(1024 * 1024);
    }

    fn mark(&mut self) {
        let mut worklist = self.roots.clone();
        
        while !worklist.is_empty() {
            let current_id = worklist.remove(0);
            
            if let Some(obj) = self.objects.get_mut(&current_id) {
                if !obj.header.is_marked {
                    obj.header.is_marked = true;
                    // Add children to worklist
                    for &child_id in &obj.references {
                        worklist.push(child_id);
                    }
                }
            }
        }
    }

    fn sweep(&mut self) {
        let mut dead_objects = Vec::new();
        
        for (&id, obj) in self.objects.iter_mut() {
            if obj.header.is_marked {
                // Reset mark for next cycle
                obj.header.is_marked = false;
            } else {
                dead_objects.push(id);
                self.bytes_allocated -= obj.header.size;
            }
        }
        
        for id in dead_objects {
            self.objects.remove(&id);
        }
    }
}