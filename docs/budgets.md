# Budgets for input that is not trusted

The svg-dom-graph library imposes no limits of its own.
Its [security notes](https://github.com/ChrisWhealy/svg-dom-graph/blob/main/docs/security.md) say that an application that builds a scene from outside data has to bound the work first.
This application does, in three places, and each has a test.

## What the input is

A scenario is a seed, a handful of numbers and a script of events.
In this application the scenarios are fixed.
Each is parsed and run the way a scenario from a file or a URL would be, so the checks are the ones that such an input would meet.

## Three bounds

| Bound | Where | What it stops | Test |
|---|---|---|---|
| The parameters are checked before anything runs | `crates/service-sim-engine/src/params.rs` | A server count, a queue size or a retry limit outside its range, and a configuration that disagrees with the parameters it came from | `known_answers::parameters_and_configuration_that_differ_are_refused` |
| A run has ceilings on its arrivals, its heap, its events and its slices | `Limits` in the engine, and the ceilings that each scenario declares | A run that would go on for ever or grow without bound. It is an error and never a truncation | `known_answers::a_limit_that_is_exceeded_is_an_error_and_not_a_truncation` |
| The run is replayed to its end before the scene is built | `Spec::preflight` and `PreflightError` in `crates/service-sim/src/session` | A scene for a run that is over its ceilings, or has more than five servers. Nothing is drawn, and the page keeps the scenario it had | `catalogue::unit_tests::the_ceiling_of_each_scenario_is_its_total_so_that_one_more_step_is_refused` |

## The size of the scene does not depend on the run

The library's cost grows with the nodes, the edges and the cells that a caller draws.
This application draws a fixed set: the dispatch node, at most five servers, the waiting queue, and one data node of eight counters.
Stepping through a run rewrites the counters and the colours, and adds nothing.

`app/tests/browser.rs` has the proof: `the_scene_does_not_grow_with_the_length_of_the_run` seeks to the end of a 487 snapshot scenario and checks that the scene holds the elements it started with.
The test fails if a change makes the scene grow with the run.

## For your own input

1. **Decide the ceilings from the cost, and write them down.** The library's cost is per node, per edge and per cell of a data node, and `Scene::move_node` redraws every edge of the node.
2. **Check before you draw.** A refusal after the scene is built is a scene to take down.
   Refuse a request in your own types, and return the reason to the person who made it.
3. **Make the over-limit case an error.** Do not cut the input to fit, since a result that is silently shorter is wrong in a way that nobody sees.
4. **Test the edge.** A test for one more than the ceiling, and one at it, is what keeps the ceiling where it was written.
