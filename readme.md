# 🍕 SliceR - Not Everything at Once

SliceR is a specialized browser with first class support for [incremental
learning][0].

> ⚠️ Please note this project is under development, use at your own risk!

There is a wealth of information out there that your are never going to consume or gain knowledge from.
All those files, videos, bookmarks, screenshots will be forgotten. You won’t  become a master.

SliceR is your teacher. The one tool that will keep you accountable to your learning goals. It will help you 
stay on track and be true to your goals. SliceR makes learning fun 🤩 Achieve mastery!

## Why Build a Web Browser?

You could just make a Google Chrome extension, why go through all of the hassle
to reinvent the wheel. Most of these projects focusing on spaced repetition,
inherently do not understand the document object model (DOM). There is so much
information and patterns in the DOM.

Managing information is hard. Trying to schedule and maintain a learning pace
is near impossible unless you really focus down or get somebody else to deal
with all of the planning for you. There is a lot of manual steps involved, like
prioritizing the knowledge, scheduling when to learn, review, etc. This is all
stuff that can be done by a computer much faster and more efficiently.

Creating yet another tool that does web scraping will not enable you to build
powerful learning tools that deliver people with insane value. If you want
something very flexible, you need to invent it. The DOM is here to stay. It’s
time your learning tools understand it well!

## Development

This project was bootstrapped with [imba-electron-template](https://github.com/imba/imba-electron-template).

## Available Scripts

In the project directory, you can run:

### `npm run watch`

Continuously build the project.

### `npm start`

Start the electron application.

# Caveats

If you set `nodeIntegration` to `true` you also need to add `"builtins"` to `"external"` list in imbaconfig->defaults->renderer

[0]: https://supermemo.guru/wiki/Incremental_learning
