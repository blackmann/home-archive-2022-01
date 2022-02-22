# Motivation

The first time I saw synced lyrics was with the [Musixmatch](https://www.musixmatch.com) app.
That was many years ago. Even though the title says **Synced Lyrics from Apple 🍎**, I'm just [literally] finding out from the Musixmatch homepage that Apple uses their services.

> Read here: [https://about.musixmatch.com/business/customer-stories/apple-music](https://about.musixmatch.com/business/customer-stories/apple-music)


But recently, I've been closer to Apple Music than with Musixmatch and I attribute the inspiration to the former.


## How I built this

So I'm going to list the challenges I thought I would face and how I could solve them - giving myself options.

### Lyrics file 

I knew there would be a standardised lyrics file format. 
And I also knew there was a higher probability of getting a lyrics file for free online. 
So I searched for "Roddy Rich The Box lyrics file" and I got a [result](https://www.megalobiz.com/lrc/maker/The+box.54814151).
Then given what I was seeing, I wanted to confirm if that was a standard lyrics file. 
Knowing that it was a standard lyrics file will give the assurance that every lyrics' file I find will follow that pattern.
This Wikipedia article ([LRC (file format)](https://en.wikipedia.org/wiki/LRC_(file_format))) gave me the assurance.


### Rendering the lyrics

There are a couple of options to choose from on how to render the markup for the lyrics.

1. `Dynamic Rendering`: With this method, what could be done is have a list of tracks with their lyrics and as the user clicks through the tracks, you render the lyrics. This would mean, you provide a container element in your HTML markup then update the [`innerHtml`](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML) of that element with a rendered lyrics markup whenever the track changes
2. `Static Rendering`: This means the lyrics are already included on the page's markup. That is, you type each line of the lyrics into your HTML file.

The difference between the two options is the same when talking about [Server-Side Rendering vs Client-Side Rendering](https://www.clariontech.com/blog/server-side-rendering-vs.-client-side-rendering) - with client-side being the dynamic option.

I went with static rendering because of SEO and accessibility benefits. 
But this doesn't mean I typed each line into the markup. I wrote a simple node script to parse the lyrics file and generate/insert the markup into a target HTML file.
Source here: [`insert-lyrics.js`](https://github.com/blackmann/blackmann.github.io/blob/master/experiments/synced-lyrics/insert-lyrics.js)

#### Semantic

We could choose to between paragraph (`<p/>`) tags or list (`<li/>`) tags. I preferred the list tags. Makes more sense to me than a paragraph.


### Syncing the lyrics

This was the most interesting part for me.
A couple of things to have in mind are:

1. As the song plays, we need to move the lyric up.
2. When a user moves (seeks) the song forward or backwards, we need to show the correct/current line to that effect.
3. When a user clicks on lyrics line, we need to move (seek) the song to that line.

> Being able to list down challenges to a problem gives you a head start on where to start tacking your problems.

From point **3**, I got an [_eureka_](https://en.wikipedia.org/wiki/Eureka_(word)) to annotate each line with the time it starts.
Then, when a user clicks on that line, we move the player to that time.


Now since we have each line annotated with its start time, to scroll to that line we can just get the [`bounding_rectangle`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect)` > top` of that line and move by that distance.
The snippet below demonstrates what I mean

<script src="https://gist.github.com/blackmann/d22ab1e2463ccd0135aa766a503dec0e.js"></script>

Now for point **2** and **3**, the approach is to figure the event when time changes for an audio. 
Reading through the docs for `<audio/>` from [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio), I find out that, that event is `timeupdate`.

What this means going forward is that, whenever `timeupdate` event fires for the `<audio />` - while playing or user clicks on the seekbar -  we will find the line that spans that time and scroll it into view.

<script src="https://gist.github.com/blackmann/f9199ffb0422d4679b92ef9c1911ad3a.js"></script>

#### Finding the lyric line for a time

The function to find the correct lyric line (for a `currentTime`) can follow this ad-hoc approach:
1. let `found` (the result) be `undefined`
2. iterate through the list of lyrics lines, call each line `line`
   1. if the `currentTime <= line.time`, then `found = line`
   2. else break (from the iteration)

As simple as that. But this method uses a [Linear Time Complexity](https://en.wikipedia.org/wiki/Time_complexity#Linear_time).
_O(n)_ complexities aren't that terrible. But if we have the opportunity to improve our code, let's take it.

We can improve this line search by using [Binary Search](https://en.wikipedia.org/wiki/Binary_search_algorithm) algorithm which is way faster than our linear search method.

That solution will look like this:

<script src="https://gist.github.com/blackmann/1781189eb6ca2ee329940768a8379b3a.js"></script>

## Done

So this is the breakdown for how I worked on this. The complete source code can be found [here/Github](https://github.com/blackmann/blackmann.github.io/blob/master/experiments/synced-lyrics/main.js).
Only 61 lines of Javascript code.