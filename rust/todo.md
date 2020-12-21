# HashLink VM

The immediate list of things needed are:

- Need to analyze the best architecture for analyzing and transforming the hash-link bytecode to
  SSA form
- Need to decide on a solution for this that also allows for some simple lifetime annotation
- Need to be able to easily add optimization passes over the top of the bytecode when such
  opportunities arise.
    - An example would be determining when objects do not out live the function they are allocated
      in so we can use plain stack allocation rather than a full GC alloc.
        - Do not use the "C" stack to prevent stack overflows, make this stack an explicit part of
          the GC to allow for dynamic allocation as long as the value doesn't outlive the stack
          frame.

## Garbage Collector
- Investigate a tracing, precise, moving, generational, stop the world garbage collector
    - Ideally find an off the shelf algorithm and implement it
    - Needs to match the expected workload of games, so a high churn of resources that wont live
      longer than a single frame
    - Generational will very likely be the ideal solution
    - Need to be very, very reliable and consistent with runtime overhead and should never ever ever
      spike in runtime
        - Take inspiration from game lisp where GC overhead is directly tied to the amount of resources
          allocated since the last GC cycle
        - The GC should run for a short amount of time every frame
        - Almost all resources will not live longer than the span of a single frame so optimize for
          being able to free entire blocks of memory quickly
        - Most long living resources will be external to the VM and accessed through handles that
          wont need to be traced by the GC. The less GC memory we allocate the less time we'll spend
          collecting.
    - Stop the world for simplicity, keep an atomic value around so we can know when there is any
      haxe code executing inside the VM
        - JIT special trampolines for calling into the VM from outside which automatically handle
          incrementing and decrementing the execution counter for tracking when the VM is currently
          executing

## JIT Compiler
- Using LLVM
- BUG: Current algorithm for propagating the live register set can't handled basic block graphs with
  cycles
- Need to figure out how to implement exceptions
- Optimization Passes
    - Look at lifetimes of allocated objects and conservatively deduce when stack allocation is
      applicable to avoid allocation overhead and reduce GC pressure

## LLVM Issues
- May need to temporarily switch to a nightly compiler due to linkage problems on windows gnu target
- No feature flags needed but the needed behaviour of static linking is only available on nightly

# Interface Generation

## Exposing to HashLink

- Idea: using a procedural macro to auto generate the bindings directly in the crate without having
  to actually write code out to files
    - This could be very good for simplifying the build system
    - Not writing out to a file means the crate can just be consumed as a library and can operate
      over the dependent crate directly
