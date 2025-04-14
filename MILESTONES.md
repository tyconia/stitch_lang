# **stitch_lang + Weaver Milestone Roadmap**

## ✅ Phase 1: Core Runtime Foundation (stitch_lang)

- [ ] **Event System**  
  <sub>April 15–20, 2025</sub>  
  - [ ] TCP / unix socket / pipe reception (standalone runtime)
  - [ ] Leverage event channels and ECS decoupling (bevy integration)
    - [ ] Bead Slots are entities. Arguments are polled manually, update components when resolved
  - [ ] Filter expected event data shapes, error when shape is not anticipated, warn if shape is anticipated but event node is not turned on.

- [ ] **Bead System**  
  <sub>April 21–26, 2025</sub>  
  - [ ] Define bead schema: inputs and outputs(Bead slots, stitchables), implementation and metadata
  - [ ] Bead and respective implementation registration to runtime

- [ ] **Fabric Compilation**  
  <sub>April 27–May 5, 2025</sub>  
  - [ ] Parse fabric definitions from RON to runtime-ready data 
    - [ ] Reference beads using namespaces which is pulled from Runtime
    - [ ] Inline / Hardcoded data as static arguments
    - [ ] A Stitch defines bead slot source and destination
  - [ ] Validate stitches statically
    - [ ] Bead only participates in a lifecycle when all inputs are stitched; and
    - [ ] Verify predecessor node is an Event node and subsequent beads are Effect node(s) unless of course Bead is itself an Event bead. (mutation pattern)

- [ ] **Lifecycle Management**  
  <sub>May 6–10, 2025</sub>  
  - [ ] Startup + Shutdown events dispatched and handled
    - [ ] Startup event is precursor to turning on Event nodes allowing initialization 
    - [ ] Propagate data in the beads of the fabric
      - [ ] mpsc channels to await arguments (standalone runtime)
      - [ ] poll bead slots manually with a system (bevy integration)
  - [ ] Graceful exit hooks and debugging info
    - [ ] Clean up system (bevy integration)
    - [ ] SIGINT ctrl + C handling
    - [ ] SIGTERM

- [ ] **Basic Logging & Diagnostics**  
  <sub>May 11–15, 2025</sub>  
  - [ ] Print bead and event flow diagnostics

## ✅ Phase 1.2: UI + UX

## ✅ Phase 1.4: Attributions and licensing
---

## ✅ Phase 2: Weaver Editor Foundation

- [ ] **Editor Scaffold**  
  <sub>May 16–20, 2025</sub>  
  - [ ] Set up web project (eg. Vite/React/Svelte)  
  - [ ] Layout: canvas area, sidebar, properties pane

- [ ] **Visual Fabric Editor (MVP)**  
  <sub>May 21–30, 2025</sub>  
  - [ ] Node system for beads and fabrics  
  - [ ] Create connections (inputs/outputs) visually  
  - [ ] Serialize into fabric definitions

- [ ] **Live Compile & Hot Reload**  
  <sub>May 31–June 5, 2025</sub>  
  - [ ] Send fabric to runtime without restarting  
  - [ ] Immediate feedback loop for testing

---

## ✅ Phase 3: Intermediate Features

- [ ] **Dynamic Bead Creation (Meta-Fabrics)**  
  <sub>June 6–15, 2025</sub>  
  - [ ] Define fabrics that output new beads  
  - [ ] Support for dynamic loading into the runtime

- [ ] **Component Reflection System**  
  <sub>June 16–20, 2025</sub>  
  - [ ] Introspect bead types and fabric structures in runtime  
  - [ ] Allow Weaver to auto-generate UI from bead definitions

- [ ] **HTTP / Networking Bead Pack**  
  <sub>June 21–28, 2025</sub>  
  - [ ] Basic HTTP server bead  
  - [ ] HTTP request/response beads

- [ ] **Async / WASM Runtime Support**  
  <sub>June 29–July 5, 2025</sub>  
  - [ ] Build runtime compatibility for browsers  
  - [ ] Test runtime in WASM environments

---

## ✅ Phase 4: Polish & Proof of Concept World

- [ ] **Initial Tycoon World**  
  <sub>July 6–15, 2025</sub>  
  - [ ] Build a small interactive simulation  
  - [ ] Use HTTP beads for remote triggers

- [ ] **Editor UX Polish**  
  <sub>July 16–20, 2025</sub>  
  - [ ] Drag & drop improvements, panning, zoom  
  - [ ] Context menus and shortcuts

- [ ] **Documentation & Tutorial Fabric**  
  <sub>July 21–25, 2025</sub>  
  - [ ] Make a “tutorial” fabric to teach users  
  - [ ] Document internal APIs

- [ ] **Public Alpha Release**  
  <sub>July 26–31, 2025</sub>  
  - [ ] Package Weaver + runtime  
  - [ ] Announce + start feedback collection

---

### ✅ **Optional (Post-Alpha Goals)**

- [ ] Multiplayer runtime synchronization  
- [ ] Remote fabric loading and saving  
- [ ] Cloud-hosted runtime playground  
- [ ] Community bead library
