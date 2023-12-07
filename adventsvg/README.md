# adventsvg

This repository follows my journey to follow and understand this [SVG Tutorial](https://svg-tutorial.com/), one of many web advent calendars I've come across this December (2023). I'm going to document each day as I approach it, and amend this README accordingly.

## Day 1

Drawing basic shapes, and then compositing them into a bauble for a Christmas tree. I got a little bit lost trying to figure out why the first one I made wasn't working, so I read [the MDN page](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/svg) on `<svg>`. I never copy & paste from tutorials: in this scenario, my error was forgetting to add the `r` parameter to set a radius for the circle.

I don't think the tutorial is very clear about precisely what the [`viewbox`](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/viewBox) attribute and its parameters are for. I'm generally pretty good about spatial reasoning but coordinate geometry has always been a sore point for me. It reminds me of struggling to create animations in Processing during college, but the nature of the IDE and runtime meant that it was quick to iterate when experimenting.

I'll have to do some additional reading, and maybe get to grips with Firefox's developer tools so I can more easily map the values to what they represent.