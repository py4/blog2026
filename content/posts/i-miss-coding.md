---
title: I Miss Coding
date: '2026-03-17'
date_display: March 17, 2026
description: Coding agents have transformed my work at Google. But even if I keep
  my job, I don't enjoy this new role. I used to love figuring out the 'how.' Now
  the agent does it, and doing it myself is just inefficient.
slug: i-miss-coding
layout: post
---

I see Hacker News is very polarized about coding agents. Half the people believe they're transforming the industry. Among them, many believe there will be fewer spots for SWEs, at least in the traditional sense. The other half believe coding agents are very dumb and only useful for trivial tasks—"not real work."

I have bad news for the latter group. My (And many groups that I see) work at Google has been transformed, and I see the same thing across all FAANGs. It's been a long time since I've coded manually, and I hope you believe we're doing "real work." Major companies are pushing hard for more AI usage—they have leaderboards to motivate people to appear at the top. But anyway, my goal here isn't to persuade anyone that coding agents are useful.

In these discussions, people who argue agents won't replace SWEs make two points: either the agents aren't good enough, or they still need higher-level supervision. I'm not sure about the first argument, but the second one has made me sad for quite some time.

For me, losing my job is one aspect of the problem. The other aspect is, even if I keep my job, it's not fun for me to talk to an AI agent at a higher level. You tell it what problem you want solved, and most of the time, it figures out the "how" correctly. I used to enjoy figuring out "how" myself. Now, it's an inefficient use of time if I do it myself.

Consider performance optimization, something I've been passionate about. Say you want to improve the performance of something. Either the agent finds a legit bottleneck at first glance and proposes a solution, or you put it in a loop to brute-force different solutions and get feedback from its previous experiments.

I've done this myself. I wanted to improve the performance of some MoE training and the MFU (model flop utilization) was low. I put the agent in a loop, told it to track experiments in a markdown file, it ran 40 experiments iteratively and cracked it. It would have taken me much more time to do the same. There are even much more advanced versions of this technique, like [AlphaEvolve](https://arxiv.org/abs/2506.13131) and [OpenEvolve](https://github.com/algorithmicsuperintelligence/openevolve). Others have shared similar patterns—Andrej Karpathy [tweeted](https://x.com/karpathy/status/2030371219518931079) about his [autoresearch](https://github.com/karpathy/autoresearch) project. There's also [PostTrainBench](https://github.com/aisa-group/PostTrainBench). Performance optimization, my passion, is getting more and more automated, because it's easily verifiable—extremely GRPOable, like math. And performance optimization isn't even just coding. A lot of it is acting as a detective—deeply debugging a distributed stack end-to-end, figuring out where the source of inefficiency is, and coming up with an innovative solution. Not only is the implementation part getting solved, but even the discovery process, which was 90% of the fun, is getting more and more automated.

People often say if your concern is coding, you have a bigger problem—that coding is a small part of the solution. Agreed, but it was also one of my most favorite parts.

The moral is, even if I don't lose my job, I don't seem to be liking this new role of SWE *that* much, and I miss—and will always miss—the old world.

**P.S.** I talked to a friend a couple of nights ago. He mentioned an interesting point. Maybe now we can enjoy programming for ourselves more. Since starting my professional career, coding hasn't been as much fun as it used to be. One theory is that it's no longer a black box—I've figured so many things out that there's less wonder. Another theory is that when you code for money from morning till night, you lose interest in it as a standalone passion.

For more on what "higher-level" work looks like in practice—and whether it's actually safe from automation—see [The Priesthood of System Design](llms-system-design).
