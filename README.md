# nitrogen_helium

Nitrogen-helium mixtures are used to detect leaks in gas systems. Due to the 
small size of helium molecules, even small leaks can be detected in the system.

`nitrogen_helium` is a tool developed in Rust for finding duplicate files on 
your system, detecting 'storage' leaks.

## TODO

- Improve error handling. At the moment `n_h` makes heavy use of `.expect`, which is suboptimal.
- Add support for specifying root directory. Currently `n_h` defaults to using the CWD of the process. 
