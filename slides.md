---
theme: default
highlighter: shiki
lineNumbers: false
info: |
  ## What the WASM?
  Demistfying Web Assembly, how it works, why it's cool
  A Presentation to the Catawiki Tech team
---

# What the WASM

Demistfying Web Assembly, how it works, why it's cool

---
layout: center
---

<style>
.container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
}
.key {
  text-align: right;
  font-weight: 600;
}
</style>

<div class="container">
  <div class="key">Question</div>
  <div>What <em>is</em> WASM?</div>
  <div v-click class="key">Answer</div>
  <div v-click>
    Web assembly<br /><br />
    <code>&#60;/talk&#62;</code>
  </div>
</div>

---

How my brain works:

1. I was learning Rust for "Fun"
2. I saw a Rust frontend framework - it was using WASM
3. Huh... I wonder...

Then: 

- What would I even use web assembly for?
- How does web assembly even?
- Rewind: How does assembly even?

TODO: Rustacean

<!--
- Found a rust framework for frontend (yew.rs)
- That looks pretty cool
- Oh web assembly, nice, but i'm sure this isn't only what it's designed for
- {slide}

- So, let's get to the root and come back
-->

---
layout: image-right
image: ./assembly.JPG
---

## Contents

<p v-click>Rewind: How does assembly even?</p>

---

## What _is_ Assembly?

Trust: This will help us later

CPUs are smart dumb things

- CPUs have a set of instructions
- Assebmbly are those raw instructions that we send to a CPU
- But, if we are sending raw instructions to the CPU, how is this even safe?

---

## Kernels

- Our code compiles to assembly
- Our code also is built against a kernel
- Hello LibC and stdio.h

- Cool facts, let's check C => assembly

- Did you know you can invent your own malloc?
- Did you know LibC isn't actually doesn't follow the C standard?
- How much are we reliant on libc?

Why is this even important?

- Web assembly

---
layout: image-right
image: ./assembly.JPG
---

## Contents

<p v-click-hide class="faded-hide">Rewind: How does assembly even?</p>
<p v-after>How does web assembly even?</p>

---

:drake_no: Assembly

:drake_yes: C

Let's hello console!

---
layout: image-right
image: ./assembly.JPG
---

## Contents

<p class="faded">Rewind: How does assembly even?</p>
<p v-click-hide class="faded-hide">How does web assembly even?</p>
<p v-after>What would  even use web assembly for?</p>

---

# MARKDOWN
Audience: Developers
Assumption:
- People know what assembly is, but never use it
- People have heard of web assembly, but it's vague

Goals:
- What web assembly is, how does it work
- What problems does it solve?

Caveats:
- Is it the new JVM?

Structure

<!--
The last comment block of each slide will be treated as slide notes. It will be visible and editable in Presenter Mode along with the slide. [Read more in the docs](https://sli.dev/guide/syntax.html#notes)
-->
