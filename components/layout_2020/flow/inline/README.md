# Inline elements processing

In any browser inline elements processing is quite hard procedure cause it involves quite deep understanding of several inndustry specifications.
Also it often involves non trivial optimizations in several domains: markup such as XML or HTML, general text processing, and browser engines.
In this file I want to provide good summarization of knowledge that I get due to work necessity of studying several code databases and blogs to
understand pipelines of working with text in modern browsers.

## List of formal specifications
 - [Microsoft Opentype Fonts specification. File extensions: *.ttf, *.otf, *.ttc, *.otc](https://learn.microsoft.com/en-us/typography/opentype/spec/)
 - [Apple Advanced Typography (extended Opentype) specification.](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6AATIntro.html)
 - [Unicode bidirectional algorithm](http://www.unicode.org/reports/tr9/)
 - [Unicode linebreaking algorithm](http://www.unicode.org/reports/tr14/)
 - [Unicode text segmentation algorithm](https://www.unicode.org/reports/tr29/)
 - [Unicode technical reports](https://www.unicode.org/reports/)

## List of intresting online materials
 - [Blink inline layout README.md](https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/layout/inline/README.md)
 - [Android Minikin Typesetting autor's article about core concepts of working with text](https://raphlinus.github.io/text/2020/10/26/text-layout.html)
 - [Core features of minikin by the same author. Important here we can understand why it is not suitable for web.](https://raphlinus.github.io/text/2022/11/08/minikin.html)
 - [Overview article of Saffron, technology used in Adobe Flash Player, Amazon Kindle and Monotype. By core author](https://ronaldperry.org/SaffronWebPage/index.html)
 - [List of various technical aspects of Saffron solution. Lots of ideas could be taken to improve layout and rendering speeds. Patent rights should expire quite soon](https://ronaldperry.org/SaffronTechDocs/index.html)
