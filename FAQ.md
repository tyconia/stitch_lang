# FAQ

## ❓ What is Stitch Lang?

Stitch Lang is a flow orchestration language. It stitches together logical units (called Beads) into declarative, observable pipelines (Fabrics) for deterministic, scriptable behaviors.

## ❓ What are Beads, Bead Slots, Fabrics, and Stitches in Stitch Lang?

### Bead
Bead is an isolated, reusable unit of logic. It contains implementation and metadata but does not directly interact — interaction flows strictly through its Bead Slots. Beads themselves can technically become arguments (meta-composition), but that's considered advanced and rare.

### Bead Slot
A Bead Slot is the defined input or output terminal of a Bead. Data flows explicitly from one Bead Slot to another — never from bead to bead directly. This promotes clear, contract-first flow control and avoids hidden coupling.

### Fabric
A Fabric is a fully declared, static composition of Beads and their Slot-to-Slot connections. Fabrics are parsed from RON and compiled at load time, ensuring that runtime execution is predictable and validated up front.

### Stitch
A Stitch is the link between two Bead Slots. It defines how data flows from one slot to another — from a source slot (A) to a destination slot (B). A Stitch ensures that input and output connections between Bead Slots are properly established within a Fabric, forming a complete and operational data flow.

## ❓ Is Stitch Lang an actor model?

Not quite. Stitch Lang is actor-inspired but deliberately scoped smaller. Beads are lightweight single-threaded actors inside the fabric only. Stitch Lang does not manage distributed or concurrent actors. It expresses flow, not compute concurrency.

## ❓ Is Stitch Lang multi-threaded?

No. Stitch Lang is strictly single-threaded. It focuses on expressiveness and predictability. Heavy workloads are dispatched outside, to backend systems better suited for parallelization.

## ❓ Can fabrics be modified at runtime?

No. Fabrics are compiled once and remain static during execution. This ensures reliability, traceability, and prevents runtime chaos.

## ❓ Can Beads create other Beads?

Yes — but only declaratively. For example, you can use meta-beads like CreateBead inside a fabric to define new Beads for future compilation. But they do not "spawn" arbitrarily at runtime.

## ❓ What are the rules for valid bead wiring?

A bead must respect flow clarity:

Its predecessor must be an Event node.

Its successors must be Effect node(s) unless it is itself an Event node.
This maintains clean, mutation-driven patterns.

##  ❓ Is Stitch Lang a compute language?

No. Stitch Lang is orchestration-first. Actual computation happens in implementations beneath Beads, or dispatched to backend services.

##  ❓ Does Stitch Lang support conditions, loops, or branches?

Yes, but only declaratively within the fabric. Think: dataflow conditional paths, not runtime control structures.

##  ❓ What is Weaver?

Weaver is the visual editor and fabric design environment for Stitch Lang. It allows users to design, validate, and export fabrics interactively, while keeping strict adherence to static fabric principles.

## ❓ Why not just use scripting languages like Lua or JavaScript?

Because Stitch Lang is not an imperative scripting language. It enforces architectural clarity and flow-driven design, removing the temptation for runtime chaos. Fabrics act as contracts, not freeform code.

## ❓ What if I need to process large data streams or perform intensive logic?

Pass control to a backend system. Stitch Lang is designed for orchestration, not computation. Delegate heavy tasks and continue orchestration based on results.

## ❓ How does Stitch Lang help with determinism?

Fabrics are static, flow is explicit, and state changes are declarative. No runtime mutation of logic flow = maximum determinism.

## ❓ Is Stitch Lang a visual language?

Not inherently. Stitch Lang is data-defined (like RON/JSON), but Weaver provides the visual interface for working with fabrics.

## ❓ Can I use Stitch Lang outside of game development?

Absolutely. It is domain-agnostic. Stitch Lang is suitable for orchestrating workflows in simulations, interactive environments, automation tools, and any system needing deterministic flow control.

## ❓ Can Stitch Lang be embedded?

Yes. The runtime is designed to be embeddable in Rust projects — standalone or inside engines like Bevy.
