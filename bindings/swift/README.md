It would be more consistent to put Swift binding C headers here. But, that will not work because of two SPM limitations:

- Each target must have a unique directory
- C headers must be within that directory

Building both parsers as one target is possible, but would result in naming issue with the copied queries. If one day either of those limitations is removed, the per-parser bindings can be moved here.
