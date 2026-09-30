---
title: Software Engineering After Code
date: '2026-09-30'
date_display: September 30, 2026
description: "Software engineering career is being rapidly changed. We no longer design & implement. Implementation is cheap now. What matters now is figuring out: what to implement (product), in what shape and behaviour (spec, constraints), how to make agents efficiently work on it (harness, guardrails) and how to verify/validate it. Unfortunately, the problem-solving part is fading."
slug: software-engineer-after-code
layout: post
ai_use: no_ai
---

In the past 9–10 months, as I delegated more and more of design and implementation to AI, I accepted that my former work life is fully over, long before [DHH](https://x.com/dhh/status/2102936073642869121) talked about it at Rails World 2026.
I have successfully completed the [5 stages of grief](https://en.wikipedia.org/wiki/Five_stages_of_grief). AI skeptics always think that those people who claim AI is taking over SWE tasks are working on CRUD apps with trivial backend/frontend. I work on performance of LLM training and inference on GPUs and TPUs. Anything verifiable (most SWE applications) is [RLVR-able](https://rlvrbook.com/) and an easy target for AI. Consider the most difficult example of recent works, [navier stokes](https://openai.com/index/navier-stokes-solution/) problem. They ran 10K agents working 88 hours in parallel trying different ideas. I don't think our day-to-day *design & implementation* problems are more difficult than proving navier stokes. 

I really need to write another blog post on how I personally feel as a human about all these, but that's a topic for another time.

Reflecting upon past couple of months, I decided to write this position article on where the job is moving, at least in the near future. In the current AI landscape, 1 year is equivalent to 5 years, so I have zero confidence about predicting beyond 1 year, but I guess the following makes sense to me within the next 1 year.

:::tldr
**tl;dr** is I think the future of SWE in the short term is 50% product management (figuring out what to build and in what shape), and 50% enabling effective collaboration of agents (harness, guardrails) and verification of their output (designing proper tests). 
:::

## On Product Management

This is the part I'm mostly excited about. The cost of implementation is going down toward zero and these days an agent or combination of agents can implement many things, ranging from coding an Electron app for a niche usecase you have in your life, to solving navier stokes (Albeit the latter is currently *extremely expensive*).
I feel that I'm not limited by design and implementation cost anymore and my limit is *mostly* my imagination. Being an overthinker and perfectionist doubles the "getting started" cost. And AI productivity also eliminates this issue to a large extent. This feels VERY VERY empowering. 

In a world where the cost of implementation and solving problems is going down, what remains valuable in the short term is **figuring out what to solve or what to build**, and **in what shape**. 

Imagine you are not bottlenecked by implementation cost. What will you build? 

P.s: Note that once you start describing the product and its shape and spec, you are [effectively writing code](https://haskellforall.com/2026/03/a-sufficiently-detailed-spec-is-code).

## On SWE

Let's say we know the product we want to build. Here product can refer to an actual e2e product like an app, or a goal such as improving performance of training/inference on Google Cloud TPU. Any attempt like prompting "Achieve goal X" or "build product Y" will fail hard. Concrete example: Let's say we want to improve inference performance of model X on Google Cloud TPU.

### Ambiguities

We need to **figure out all the ambiguities** before handing the task over to AI. 

1. What does "optimizing inference performance" even mean? What exactly should the agent optimize for? Which workload? What's the objective function? Which model? Which hardware? 
2. When can the agent stop work? What's the ending condition?
3. What are the red lines the agent should never cross? Can it do quantization? At which levels?


I've found that with agents, **any room for ambiguity will backfire**. I've improved my instructions for the agent for several weeks. I was like OK, I've resolved all ambiguities and I've defined very clear boundary that the agent should operate within. You know what happened? I noticed the agent was *retraining* a part of the model to improve speculative decoding acceptance length!! I was like dude WTF. Normally when I do inference optimization, we never ever retrain a model. We might quantize further, but we always assume the model is frozen. It had not crossed my mind that the agent can do this unprompted. Otherwise I would have constrained it in the first place. This wasted a couple of hours of the agents' time and the company's resources.

### Harness & Guardrails
Depending on your task, you might want to *deterministically* enforce some sorts of behaviour on the agent. For instance, if I have an agent working on performance optimization, I need to enforce that it runs `correctness.sh` before committing the changes to make sure the model quality is not compromised. You can try prompting it, however there is no guarantee that the agent follows. Especially in the long running sessions that get auto compacted frequently.

### Verification
When the agent delivers something, how should we verify it? Unless you are working on something critical, I feel reviewing AI code is becoming less and less relevant. Nx productivity gain by AI [remains bottlenecked](https://imgur.com/a/Du9Gedt) by human code review speed in that case. This issue is even more severe when AI is touching codes outside of your team's expertise. Recently, we had AI write a kernel in a third party library for a specific non-famous GPU hardware and in some parts it wrote PTX. We were not experts in either that particular GPU architecture or PTX code.  But agent changes ended up improving performance dramatically. What were we supposed to do? We could try to spend a lot of time to become an expert in that domain. Or we could try to *be pragmatic* and minimize the risk by comprehensive black box testing (granted, the risk is still higher than the case when you have an expert reviewing it) for the sake of getting a lot of productivity. 

You have a black box. How do you make sure it's working as expected without actually needing to understand the details of it? 

## Moral of the Story
I am happy that I experienced a pre-AI SWE career and I think the old problem solving, coding, system design, … are all gone. I've accepted it. 
Implementation is cheap now. What matters now is figuring out: what to implement (product), in what shape and behaviour (spec, constraints), how to make agents efficiently work on it (harness, guardrails) and how to verify/validate it. Unfortunately, the problem-solving part is fading.
