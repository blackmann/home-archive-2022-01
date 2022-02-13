---
title: From "Hello, World!" to Hero. Planning and Building Software 💽
slug: from-hello-world-to-hero
title_meta: For beginners
date: 12-02-2022
description: Do you want advance away from blindly following tutorials? Learn how to plan an application from wireframing, planning [re-]search and implementation as a beginner. 
---

When I began learning how to program, I didn't know of YouTube. 
I didn't know videos were made on how to build apps.
_And even if I knew, I couldn't afford to watch them._ 
But I find myself privileged today for not knowing about that.
Looking at every tutorial video today, the teacher just jumps right into setting up the project and adding code.
The teacher then gives some commentary as he/she goes on.

I think this approach has left a lot of beginners with a lot more questions than what they had before they started watching the tutorial.

- What is this new library he's using? Should I go and learn that too?
- Why did we need to have this code here and that code there?
- How did he know we needed to add that?

## So what now?

What's really missing in Tutorial Land is problem-solving skills. _Let's solve a problem._
Not just jump right into the solution. How did we arrive at using this library? 
What was the need for this function or that code source [file].

That is what this post is going to address. 
Hopefully, what you learn from this post (or series of posts) moves you [a beginner up] away from such tutorials [to relying a lot on documentations and your initiative].

> I personally find video tutorials problematic because they don't teach beginners to be able to think independently.

## What's missing?

Planning, research and study. 
Since these may be new to a lot of beginners, there may be hesitation in being practical with these methods.
But engaging in these stages will help you realize that coding/programming is the least difficult part of building apps.
_Whereas you already believed programming was hard, because that was the only thing you thought was involved in making apps._

To kind of contradict myself, this is not to say the stages of planning, research and study are actually hard. 
They are also easy but lay down the groundwork for programming to be way easier. 

### Planning 🗺

This involves expanding on your idea of the app. 
Personally, (and practically) I begin by sketching out the screens involved in the project. 
And if this involves an API to the server, I outline the kind of models and endpoints required.
This stage is very exciting because the vision for the end product becomes clearer.
While planning, you need to imagine yourself as the user of the app and ask yourself questions about its usability, simplicity and discoverability.

Do not marry your first ideas at this stage. If something doesn't fit well, take it out or improve it. 
You can find inspiration from other apps.

Also, **do not think about code at all**. Anything you sketch at this stage can be done.
Do not worry if you know how code some part or not. That's what the research and study stage is for.

This is not to say you need to be very detailed at this stage. 
The following is an image demonstrating how to sketch your screens [and prepare API requirements]. 
Consider an online shopping app:

<img src="/static/images/sketch.jpeg" alt="Pages sketch">

From this exercise, you begin to realize the knowledge gaps needed to be filled to complete the project.

### Research & Study

At this stage, you're trying to figure out how to bring your sketch to life with the programming language you know.
You'll have to make a lots of Google searches on the various aspects of the project to find out how to solve them.
This time, you're not going to make a search on how to build the whole app, but rather the little pieces that make up the app.

Let's take a shopping app for instance. On the products page, you want customers to be able to filter the products based on category and price range.
From this example, (let's assume) we don't know how to filter a list of products. 
Our research here will be figuring out how to filter a list (in your language).

✅ How to filter a list in Javascript

❌ How filter a list of products on a products page with Javascript

❌ How to filter a list of products

The problem with the last two search queries is that we have to assume someone on the internet has published a solution or document addressing your specific issue.
In most simple cases, you could find such a document. But for a lot of unique cases, you'll barely a result <b>that matches your exact problem</b>.

But the benefit of the first query is that, it is generic. 
Meaning, the solution you may find as a result can be applied to many other parts of your app.
Like filtering a list of orders based on whether they're packaged or delivered or canceled.

Now as you find the solutions to the various pieces of your app, you could do the following:

- Annotate your sketch with how you can solve that part with your research
- Quickly try out what you found as a solution in an online code runner to test your understanding and modify it to see if you can apply it as an actual solution to your problem.

The basic takeaway here is that, you need to find ideas on how to solve the smaller pieces/components of your app.


## Done

After completing the exercises above, programming will become the easier part. 
But without demonstrating, it may be hard get started with this method. 
And to be fair, there aren't a lot of _video_ tutorials to demonstrate these methods. 

So in some consecutive posts, we'll practically build a project straight from planning through coding.
I'll notify updates on that on my [Twitter](https://twitter.com/__degreat), but you can check here periodically to follow progress.


### More resources while you wait

- [App Sketches](https://www.pinterest.com/pin/239605642645185767/) [Pinterest] - watch a lot of related pins to get used how sketches are made.
- [ERDs](https://www.lucidchart.com/pages/er-diagrams)
- [What is REST](https://www.redhat.com/en/topics/api/what-is-a-rest-api)
- [Restful Service with Express/JS](https://www.robinwieruch.de/node-express-server-rest-api/)