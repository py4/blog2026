---
title: The Priesthood of System Design
date: '2026-03-23'
date_display: March 23, 2026
description: People say LLMs can code but can't do system design. I disagree. System
  design is more pattern-matching than people admit — and if you give an LLM enough
  context, it can outdesign 99% of engineers.
slug: llms-system-design
layout: post
---

Over the past year, as coding agents have rapidly evolved, I've had countless conversations with friends and colleagues debating whether they'll eventually replace software engineers. One argument I kept hearing went something like: "Sure, it can write clean, working code—but system design? That's a different story."

There's a near-reverent respect for "system design" in tech circles, especially at FAANG companies. But that argument never persuaded me, and I've spent a lot of time trying to figure out why people hold it.

Here's my experience: whenever I prepare for tech interviews, system design has always felt considerably easier than grinding LeetCode. The scope is narrower. You're working with a well-known toolkit—caching, message queues, horizontal scaling, sharding, load balancers—and the problems tend to rhyme with each other. Yes, LeetCode has its own patterns too—DFS, BFS, dynamic programming, two pointers, linked lists—but its surface area and variety of problem types is still far larger. With system design, once you've internalized the core building blocks, you can decompose almost any problem. System design is more of a pattern-matching exercise than people like to admit.

So if system design is this formulaic, why do so many people believe it's the weak spot for LLMs? My hypothesis: it's a context problem, not a model problem. Engineers typically work inside large, sprawling systems. When they hand a design task to an LLM without explaining where the new component fits—the existing architecture, the constraints, the traffic patterns, the team's operational limits—they're setting it up to fail. That's poor context management on the user's side, not a fundamental limitation of the model.

I can speak from personal experience. Since December, I've been working on a new LLM training project with significant distributed systems design involved—and that's the domain I'm speaking to throughout this post. I don't have experience designing something highly specialized, like a new database engine with specific performance characteristics, so I won't make claims about those domains. But for distributed systems, the kind of design work that comes up in typical backend and infrastructure engineering, I've been using AI throughout and it's been genuinely impressive—not just at the implementation level, but at the architectural level too. Given the right context, it reasons through trade-offs, catches edge cases, and proposes designs I might not have reached on my own.

Granted, it sometimes misses a constraint or overlooks something important—but when I trace it back, it's almost always because I didn't specify it clearly enough. (You could argue that identifying and [articulating those constraints is precisely what software engineering *is*](https://haskellforall.com/2026/03/a-sufficiently-detailed-spec-is-code), and I'll give you that.) Still, assuming sufficient context, I'd argue an LLM can design better systems than 99% of engineers.

What this means for the people doing this work—even the ones who keep their jobs—I wrote about in [I Miss Coding](i-miss-coding).
