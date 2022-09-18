# SliceR - Building a Specialized Browser With First Class Support for Incremental Learning 
by [Alexander Alemayhu](mailto:alexander@alemayhu.com)

One of the hidden gems in the [SuperMemo](https://www.google.com/url?q=http://supermemopedia.com/wiki/Main_Page&sa=D&source=editors&ust=1663531318000741&usg=AOvVaw0MK3aWK8ALwpVpm_tYlkcH) program is the incremental reading features

You can do spaced repetition, make extracts, create cloze deletions, bookmarks, use priorities, auto behaviour and web import. It’s all still fairly primitive.

SuperMemo was initially released back in 1987. This tool is old and was even built before the first web browsers ever existed. The user interface shows this clearly and it’s no surprise that projects like Anki emerge. Anki itself has lots of user interface issues too but those are out of the scope of this document.

Why Build a Web Browser?
------------------------

You could just make a Google Chrome extension, why go through all of the hassle to reinvent the wheel. Most of these projects focusing on spaced repetition, inherently do not understand the document object model (DOM). There is so much information and patterns in the DOM.

Managing information is hard. Trying to schedule and maintain a learning pace is near impossible unless you really focus down or get somebody else to deal with all of the planning for you. There is a lot of manual steps involved, like priotiizing the knowledge, scheduling when to learn, review, etc. This is all stuff that can be done by a computer much faster and more effieceinetly.

Creating yet another tool that does web scraping will not enable you to build powerful learning tools that deliver people insane value. If you want something very flexible, you need to invent it. The DOM is here to stay. It’s time your learning tools understand it well.

Features
--------

Being a web browser will enable us to do more things with a HTML webpages and other file formats. We can read much more in depth what is there, visualize it anyway we want it and provide the user with true control of what they want to extract.

In no particular order, here are the features that I consider to be important to get this to be an amazing experience. Note that this differs from Peter Wozniak’s minimum defninitation and SRS is not considered a part of this browser. This where you would integrate with Anki. This might change in the future but not for v1.

- Support for highlighting
- - Text and this includes images, video, and audio files.
- Bookmarks
- - Intelligently be able to bookmark anything or anywhere.
- Prioritisation
- - Define what is important easily
- Automatic behaviour
- - Sorting based on priority
- - Postponing less important
