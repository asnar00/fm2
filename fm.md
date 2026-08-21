# fm
*feature-modular architecture*

## introduction

*fm* ("feature modular") is an approach to software architecture that organises code according to user-visible capabilities (colloquially referred to as "features"), rather than technical function.

The goal of feature-modular architecture is to give users control over what features their software contains. Each user is able to customise their software by removing, creating or sharing features freely.

Functionality is defined using a tree of *feature nodes* under the `features` folder. Each feature modifies the functionality of its parent.

Executable programs are stored in the `products` folder. Each product is composed from some subset of the feature tree using the `fm linker`.

### example

Feature composition is best illustrated using a simple `hello world` example. In rust fm (`fm.rs`) we'd define it like this:

    // hello.rs
    struct feature_Hello;
    impl feature_Hello {
        fn main() {
            println!("Hello, world!");
        }
    }

The fm linker would turn this into the a single global function:

    fn main() {
        feature_Hello::main();
    }

Running this would print our greeing:

    >./main
    Hello, world!

Now let's say we wanted to print "goodbye" at the end of the program. We'd do this by adding subfolder to `hello/` called `goodbye`, and define the new behaviour:

    // goodbye.rs
    struct feature_Goodbye;
    impl feature_Goodbye {
        fn main() {
            existing.main()
            println!("Goodbye...");
        }
    }

The `existing` keyword is picked up by the fm linker, which adds the call to `goodbye()` to the end of the existing definition of the global `hello()` function, resulting in this:

    fn main() {
        feature_Hello::main();
        feature_Goodbye::main()
    }

The result is as follows:

    >./hello
    Hello, world!
    Goodbye...

### struct composition

To illustrate struct composition, we imagine that we first want to define a colour object using red, green and blue. In `fm.rs` we'd do it like this:

    // colour.rs

    pub struct colour {
        pub r : f32;
        pub g : f32;
        pub b: f32;
    }

The linker would create a global `colour` struct like this:

    pub struct colour {
        pub r : f32;
        pub g : f32;
        pub b: f32;
    }

If we now decided we wanted to add an alpha channel to `colour`, we'd do it in a subfeature called `colour/alpha`:

    // alpha.rs

    pub struct colour {
        pub a : f32;
    }

And the linker would add it to the global `colour` struct like this:

    pub struct colour {
        pub r : f32;
        pub g : f32;
        pub b : f32;
        pub a: f32;
    }

Within both `colour` and `alpha` features, we only need to refer to `col.r` or `col.a`; the linker automatically converts them to `colour.colour.r` or `colour.alpha.a`.

### products

A *product* is an executable composed of some subset of feature nodes from the global feature tree. This lets us create multiple versions of a product with related but different feature sets.

A product is expressed as a tree of feature nodes containing symlinks to the original feature folder in the `features/` tree. A symlink to a feature folder automatically imports the feature and all its subfeatures.

If a product wants to define its own version of a feature without modifying the shared code, it simply replaces the symlink with its own versions of the files within the feature folder. The linker then uses the local definition instead of the shared one.

### feature-scoped variables

A feature may wish to specify state that's common across all users (such as the port used to run a web server) or that can differ from user to user (such as font sizes or UI colours). These are declared as properties of the `feature_` struct, with a comment before each one (either `// @shared`, the default, or `// @user` if per-user). 

### contexts

A further enhancement of functionality is to allow some features to be *dynamically enabled or disabled*. The user may decide to turn some features on or off (or change their settings) at different times, depending on what they are trying to achieve. 

To formalise this, we create the concept of a *context* : a collection of enable/disable flags and feature settings that can be selected at a potentially fine-grained level, even down to a single function call.

Each user can manage multiple contexts on their own behalf; and different users' contexts don't interfere with each other.

## feature nodes

A feature node with path `A/B/C` (meaning "subfeature C of subfeature B of feature A") contains a *specification* (`A/B/C/C.md`) and one or more *implementations* (eg `A/B/C/C.rs` or `A/B/C/C.ts`).

It is taken as read that all specs and implementations are written by agents in response to user requests made as part of a conversation. Therefore, every feature node must refer to the specific user prompt or request in context. This makes it very clear which text is a user request and which is written by agents.

### specification

The spec has a standard format, as follows:

    # title (one to three words)
    *single-line description*

    > (reference to user prompt/request in agent conversation transcript)
    > actual user prompt/request that triggered this feature

    ## user

    up to 100 words describing to the user how they should use the functionality, if applicable. If we're talking about a library function, the "user" is the agent writing the code; otherwise, it should be written at a level that the user understands, assuming they understand all the terms introduced by parent features.

    ## spec
    
    up to 500 words describing what the feature needs to do, written for agents. This should define the problem to be solved, and the functionality introduced to solve it.

    ## glossary
    
    Any new terms introduced are defined here. Any terms used that come from the combined glossary should be demarcated using `/`, eg. `/specification`, so a reader app can create a link to the appropriate term definition.

    ## code description

    An agent-facing tutorial describing the code in the implementation file, with line number references. This should introduce salient functions first (i.e. new functions introduced) followed by a description of how they modify the existing program; and only then descriptions of internal functionality and boilerplate. Separate paragraph for each element being described.

A few style rules must be followed:

- single approved term for each concept
- simple, concise language to reduce token usage
- avoid burble such as "... and this is load-bearing"

### implementation

Code in implementation files should be commented to a decent standard - agents should use their discretion. In general, tend towards smaller functions rather than large blocks of code.

In particular, avoid this construction

    // do something
    a = b + c
    d = a * some_fun(e)

And prefer

    d = do_something(b, c, e)

with a separate function

    fn do_something(b: f32, c: f32, e: f32) -> f32 {
        let a: f32 = b + c;
        return a * some_func(e);
    }

This allows later features to extend or modify the behaviour of the feature much more easily.

### refactoring to allow extension

When trying to extend a feature, it is permissible to refactor the feature's code to extract extensible functions, as long as the feature isn't broken in the process. To ensure this, a feature should define a set of tests that can be run automatically.

### ordering

The subfeatures of a feature `A` are composed in the order defined by a file called `order.md` which should exist in every feature folder. If the file doesn't exist, use the timestamp drawn from the conversation reference, and compose oldest features first.

## feature tree rules

### one prompt per feature

If a feature node resolves to multiple user prompts, the feature should be decomposed into smaller sub-features, until each node resolves to only one user prompt. This ensures that features can be enabled or disabled at a fine-grained level.

### 4-6 children per feature

Humans have difficulty holding more than 7 items in short-term memory; therefore, we cap the number of children at 6, and aim for 4. Nodes with higher numbers of children should be re-organised, grouping similar children together under a new sub-feature. The grouped node is allowed to skip a provenance prompt, and can instead just say that it was created in response to this rule.

### tree-global names

A feature's name should describe its function and be unique across the tree, i.e. we shouldn't have to know its parent feature names to understand what it does.