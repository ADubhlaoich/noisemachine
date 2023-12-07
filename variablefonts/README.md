# variablefonts

Sometime this week I read about the [Inter](https://rsms.me/inter/) typeface getting an update. I think I'd heard of it but never used it, so I have no context for its updates.

Around the same time I independently read about [monaspace](https://monaspace.githubnext.com/), and realised both were variable fonts. I had no idea what a variable font was, so I looked into it.

In short, variable fonts are a typeface family explicitly designed for interpolating variations. I'm not a typographer, but the gidst of it is that not all typefaces come with every font (Or style) you'd like: computers need to make their best guesses at what bold or italic should look like if they're not explicitly defined.

If we take a look back at my [webfonts](../webfonts/) study, Atkinson Hyperlegible comes with four variations:

- Regular
- Bold
- Italic
- Bold & Italic

I created separate `@font-face` entries for each of them and tied them together using the `font-family` parameter. 

Packaged for web, a variable typeface doesn't need all of those variations. In fact, I was left confused as I attempted to debug the two entries for the typeface: I was trying to explicitly set the `font-style` parameter as `italic` in the italic `@font-face` block and it disabled it entirely.

Investigating everything slightly farther, I learned a little bit more useful information for all web typeface usage:

- `woff2` is the contemporary version of `woff`, meaning you can skip the latter.
- `woff2 supports variations` is the modern `format` for `woff2-variations`, which has better browser support.

So if you're willing to take bets on your reader using the latest everything, you only need `woff2`, and you can invoke it once with `woff2 supports variations`.

I try my best to ensure my work is accessible, and this feels less aggregious than attempting to support IE6, so when given the option, I'll include the fallbacks.

The CSS I have written for the `@font-face` entries doesn't include expliclit clamped ranges for how much the typeface can vary, though in some cases the ranges aren't applicable.

One of the key points I read about variable typefaces is filesize. These are the variations of the typeface on my machine:

```shell
➜ du -h Inter.ttc 
13M     Inter.ttc

➜ du -h InterVariable.ttf
844K    InterVariable.ttf
```

The `.ttc` file contains all the normal "fixed" variations, while the `.ttf` one is the variable file. A different of 12 megabytes!

Cheap for your local storage, but proportionally significant. Then I compared the two `.woff2` variations with just the two regular fonts for Atkinson Hyperlegible.


```shell
➜ du -h InterVariable.woff2
340K    InterVariable.woff2

➜ du -h InterVariable-Italic.woff2 
372K    InterVariable-Italic.woff2

➜ du -h Atkinson-Hyperlegible-Regular.woff
24K     Atkinson-Hyperlegible-Regular.woff

➜ du -h Atkinson-Hyperlegible-Regular.woff2
16K     Atkinson-Hyperlegible-Regular.woff2
```

A difference of just over 700 kilobytes versus 40 kilobytes. But what about all the variations on Atkinson Hyperlegible?


```shell
➜ ls -l
total 180
-rw-r--r-- 1 adubhlaoich adubhlaoich 25224 Apr 30  2020 Atkinson-Hyperlegible-BoldItalic.woff
-rw-r--r-- 1 adubhlaoich adubhlaoich 18068 May 14  2020 Atkinson-Hyperlegible-BoldItalic.woff2
-rw-r--r-- 1 adubhlaoich adubhlaoich 23780 Apr 30  2020 Atkinson-Hyperlegible-Bold.woff
-rw-r--r-- 1 adubhlaoich adubhlaoich 16484 May 14  2020 Atkinson-Hyperlegible-Bold.woff2
-rw-r--r-- 1 adubhlaoich adubhlaoich 24584 Apr 30  2020 Atkinson-Hyperlegible-Italic.woff
-rw-r--r-- 1 adubhlaoich adubhlaoich 17712 May 14  2020 Atkinson-Hyperlegible-Italic.woff2
-rw-r--r-- 1 adubhlaoich adubhlaoich 22792 Apr 30  2020 Atkinson-Hyperlegible-Regular.woff
-rw-r--r-- 1 adubhlaoich adubhlaoich 15884 May 14  2020 Atkinson-Hyperlegible-Regular.woff2

➜ du -h fonts 
184K    fonts
```

Eight variations are just over half the filesize of one of the Inter typeface files, even when half the variations are just fallbacks for compatibility.

This isn't an entirely fair comparison, since Atkinson Hyperlegible doesn't include all of the variation of Inter. 

Beyond that, Inter would only need 2 calls to fetch its fonts compared to Atskin Hyperlegible's 4-8.

In a vaccuum the variable typefaces feel easier from a code maintenance perspective, and assuming the two typefaces were identical besides format, the reduced amount of requests might be reason enough to choose variable over static.

Small things like these might become atomic bottlenecks: this time I thought to explicitly include `font-display: swap`.

I'm going to continue to put off learning web font loading strategies...

**Resources**
- https://variablefonts.dev/
- https://variablefonts.io/
- https://fonts.google.com/knowledge
- https://www.youtube.com/watch?v=K9q29TG4n-Q

**Bonus**  
I couldn't remember how to add typefaces to a Linux operating system. Here's the cheatsheet I put together:

```shell
# Create a fonts folder and move the font files there
mkdir -p ~/.local/share/fonts
cp ./fontfolder/* ~/.local/share/fonts

# Alternatively, for backwards compatibility
mkdir -p ~/.fonts
cp ./fontfolder/* ~/.fonts

# Check if the font is there or not
fc-list | grep "FontName"
# Clear cache if the font hasn't appeared
sudo fc-cache -f -v```
# List and preview all your fonts
fontpreview