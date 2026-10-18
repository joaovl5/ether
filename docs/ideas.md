# Ideas

Compilation of ideas that we could build upon for making Ether.

## Frustrations with Nix

1. Nix language
    - needs macros
    - **at least optional/gradual typing**
    - proper tooling, lsp, completions
    - stable intermediate representation for allowing other langs to get
      compatibility
2. Slow evals
    - may benefit from scaling sqlite-based solutions, optionally using
      postgres or some other graph db - store may be abstracted so that it can
      be easily distributed
3. Store hashes depending on absolute paths - this breaks substituters if
   store location is changed from the default of `/nix/store`
4. Store objects shouldn't need to be necessarily file system objects
    - output of a derivation may be as stdout/stderr bytes instead of a file
      having those
    - store doesn't need to be an actal file system, it could be useful for it
      to *feel* like one
    - so `/ether/store` acts as an abstraction that under the hood may call
      the database
      - `/ether/store/by-hash/<hash>/`
        - `/info` has object info as json
        - `/content` (may be file/folder) has object content
        - `/depends-on` has its dependencies as json
        - `/depended-by` has objects that depend on it, as json
      - maybe separate content-addressed and input-addressed as in
        - `/ether/store/by-hash/ca/<ca-hash>`
        - `/ether/store/by-hash/ia/<ia-hash>` (and would require additional
          signature metadata possibly)
5. Possibly avoid NAR-like things by just using something like a Merkle graph
   for content-addressing deep folders
