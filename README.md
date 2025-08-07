# Galois Fields

This was going to be a rust crate which implemented galois fields via quotienting polynomial rings over the base fields $\mathbb{F}_p$ for prime $p$. However, I've determined that rust's trait requirement tracking doesn't seem to be able to handle the complicated data required for efficient polynomial operation handling — addition via + being by reference rather than consuming the data. Might be possible if I'd implemented rust's Add/etc. core traits myself, but that's not something I can really do I don't think.
