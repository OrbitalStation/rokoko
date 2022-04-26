## About

In this file are described all the things I'd like to implement in the future

## List
<ul>
    <li>
        `MyFrom` custom trait, see `FIXME` in `vec::new`.
    </li>
    <li>
        Support for multiple specification of callbacks, i.e.
        so that `.on_init(...).on_init(...)` are possible.
    </li>
    <li>
        `Resource` conception - if `move` a variable
        into multiple closures it will be copied in each, but what if
        we need to access a resource between closures?
    </li>
    <li>
        `async` closures. If a closure is marked as `async` then
        it can be launched in a separate thread, and `.await` could be done
        when the same callback needs to be called again
    </li>
    <li>
        `World` conception, i.e. a set of windows is composed in a
        single world. The main feature is that all these windows share
        the very same `EventLoop` and are able to send & receive data
        from & to each other.
    </li>
    <li>
        The function of `WindowBuilder` -- `on_separate_thread`, which
        changes the return type of `create` to a future and runs windows in
        a separate thread, so that continuation of a caller could continue.
    </li>
    <li>
        `Backend` conception, i.e. backend is a custom(or predefined) struct
        which manage windows' graphics. Backends are `Vulkan`, `OpenGL`, `No backend`, etc.
    </li>
    <li>
        `FixedPolygon` and `DynamicPolygon` conceptions
    </li>
</ul>
