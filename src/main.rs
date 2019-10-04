extern crate patricia_tree;

use patricia_tree::PatriciaMap;

fn main() {
    let mut pmap = PatriciaMap::new();
    pmap.insert("foo", 1);
}
