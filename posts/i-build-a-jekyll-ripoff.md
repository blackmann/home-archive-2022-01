---
title: I built a Jekyll rip-off with Rust one weekend
slug: jekyll-ripoff-project
title_meta: Fun 
date: 7-02-2022
description: How I created a static site generator with Rust (inexperienced) in one weekend.
---

For so long I've been wanting to redo my homepage, this time with an easier blogging component. 
I've tried to use [Next JS](https://nextjs.org) with [MDX](https://mdxjs.com) but the experience at the end of the day was going to feel more like coding, than writing [to me].
Other options included static site generators like [Jekyll](https://jekyllrb.com), [Hexo](https://hexo.io), etc. 
But these solutions came with their troubles: overriding a lot of stuff.


Throughout this period, it felt like I had to learn these systems and then customize them [with configurations] per my preference. 
That friction prevented the need to get through setting up my personal homepage.

About two weeks ago, I picked up [Rust 🦀](https://rust-lang.org) because I began to grow some obsession for it. 
I skimmed through the [Rust Book](https://doc.rust-lang.org/book/) and tried to compare it with other languages I know.
Syntax, semantics, thought process. I also picked up [_Rust koans_](https://github.com/rust-lang/rustlings) (actually known as `rustlings`) and completed a few exercises.

## What happened    

Last weekend, despite my immature experience with Rust, I decided to build a tool that will help me generate my site from `.md` files.
I felt that would be _more lazier_ and convenient than learning a these existing site generators 😉.
Just like how Jekyll and the rest works. So I laid out the process based on how I understood static generators worked.

<script src="https://gist.github.com/blackmann/d72a975aaae5e079e5ae339dfe20fcf0.js"></script>

#### Breakdown

`.initialize` 

In here I gather all the `.md` files from the `posts` directory then convert them into plain HTML string using [comrak](https://github.com/kivikakk/comrak). While parsing the markdown, I scan out the `front matter` which include necessary metadata that form a `Post`.
This method needs to be called first. The other methods can be called in any order.

`.build_posts`

The requirement for this method is to load a template (`HTML`) and then iterate through all the gathered posts [from `.initialize`], insert the parsed `.md` contents and output each post into separate HTML files. 
Simple. 
The [Liquid](https://shopify.github.io/liquid/) templating language is the perfect solution for defining our posts' template. 
Liquid is just same old HTML but with super-powers. I only borrowed a few powers like `iteration` and `control flow`.
It allows us to also work variables that we can use in our template.

`.build_home`

This method uses the same idea as `.build_posts`. Which is, I load a home template (written in Liquid) and pass it the list of posts.
This list is used to prepare the _list_ of blog posts on the home.


## Voila

In these three easy steps, my _rip-off_ works. There are only pros to this approach (of writing my own generator).

- I only spent about 8 hours (net time) on it. I think this is better than finding the correct configurations for a static site generator over days (gross).
- I happened to learn a lot about Rust. More experience. _I'm now a [rustaman](https://twitter.com/__degreat)_
- I understand some intrinsics of a static site generator.


## The source

If you reached here. Thanks. You found a treasure 💎: [`blackmann/blackmann.github.io`](https://github.com/blackmann/blackmann.github.io). The source to `myjekyllripoff`.