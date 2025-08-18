# Structure of RPanel

Under the hood rpanel consists of a bunch of structures called `Component` and `SubRoutine`.


## Component
Components contain a reference to their respective area where they are supposed to be rendered and a callback function.

The callback function receives its data as an immutable Rc. This is because the rendering function is only supposed to render the thing, and then return a Widget which is later used in the rendering stage.

## SubRoutine
Every Component has a corresponding SubRoutine. The SubRoutine is responsible for collecting all the data needed by the renderer. This data is stored in the Rc "store", which is a Mutex.

# The loop
On startup the selected document is read into Components and SubRoutines. After this the "loop" is started.

The loop is basically the TUI rendering and "data collection"/"background services" running in parallel, allowing a non-blocking way of collecting necessary data while also keeping the UI responsive

## Plugins
Plugins are basically fancy templates with scripts attached. As the subroutine collects data and puts them in the store, the render get's access to a pointer to this store. This store should (preferably) contain data for the renderer to use.

The python renderer function then returns a hashmap object that should contain all template id-keys. In the case of an id not existing inside the hashmap we default to an empty widget.