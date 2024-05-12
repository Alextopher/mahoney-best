# Java Programming

If you just joined FRC 5881 as a new student you're very unlikely to be ready to write robot code. There are 2 things you need before you can really get started.

First, as a newbie to FRC you just need to get comfortable with the robot and how the team operates. **You should focus on having fun** and getting a sense if you're really interested in programming. Ask any upperclassmen, or team alumnus, about their impact as a Freshman and they'd all agree that they weren't really ready to influence the robot profoundly like they did in their later years.

Second, just start programming on your own, preferably in Java. Learning to code will take agency and self-direction on your part but I'm confident everyone can learn to code.

## _You_ need to learn Java

Sadly, I don't have enough time on the robotics team to teach every student Java in the way we could in a classroom. Furthermore, at TVHS we don't offer AP CS until Senior year, if it's offered at all. This is a big burden on our team, but it's also a big opportunity. While software engineering is a very collaborative field, _programming_ is extremely individual. In my experience, learning to code set me apart from my peers who only learned in a classroom.

I'm here to **mentor** you on your own journey. When it comes up I'll give one-shot lectures on anything in CS that you need to know more about.

To get to the point where you can write FRC code comfortable requires around 100 hours of Java programming experience. From there you'd know enough of the basics that you won't be completely lost on the fundamentals. It isn't a big deal where you get this experience, I would just encourage you to avoid over-reliance on Large Language Models (AI) systems.

Here are a few potential starting points:

1. [Codecademy](https://www.codecademy.com/learn/learn-java) - 16 hour course. Pro-tip this is pretty much a condensed version of the AP CS course.
2. [CodingGame](https://www.codingame.com/start) - A programming "puzzle"/"game" site with fairly unique problems and nice graphics.
3. [Processing](https://processing.org/) - A Java based language that's used for creating visual art. Processing gives you a way to "feel" (see) the results of your code. It's really nice to get immediate feedback on your code.
4. [Project Euler](https://projecteuler.net/) - A series of challenging mathematical/computer programming problems that will require more than just programming to solve. For advanced math students this gives you a way to enter the programming world through a familiar lens.
5. Just start writing code locally for a project you have in mind.

What ever you choose to do you should be 100% invested in it. If you're not having fun, or you're not learning, then you should switch to something else. Around 90% of my programming projects I would consider either "unfinished" or "abandoned", I wouldn't have it any other way.

### Programming vs Software Engineering

A quick note on terminology.

I use the terms _programming_ and _software engineering_ deliberately to make an important distinction. **Programming** is the act of writing code, while **software engineering** is the act of designing and "engineering" software. To begin with you should just program, get some lines of code written down and build cool things.

When we reach the robotics season things will change. In a personal setting you are free to rewrite your code, or stop working on projects, whenever you like. During the season we obviously have a deadline and we want to avoid the trap of rewriting the robot the night before competition. This is where software engineering comes in.

I put a lot of emphasis on _engineering_ the robot's software. Before we do any programming for the robot it's important to develop a plan and discuss what our requirements are and how we can minimize the complexity of our code while maximizing our flexibility. I won't go as far as using [waterfall](https://en.wikipedia.org/wiki/Waterfall_model) or [agile](https://en.wikipedia.org/wiki/Agile_software_development) methodologies, but I will insist that we have conversations about what we're going to do before we commit to it.

When you witness engineers talking to each other you'll notice they hardly ever speak about which line of code to write. It's almost always about how they're going to structure a project and what the requirements are. After agreeing to the big picture we trust each other to program our little pieces of the puzzle on our own.

## Why Java?

Within the FRC ecosystem there are 4 supported programming languages, namely [C++](https://en.wikipedia.org/wiki/C%2B%2B), [Java](https://en.wikipedia.org/wiki/Java_%28programming_language%29), [LabView](https://en.wikipedia.org/wiki/LabVIEW), and [Python](https://en.wikipedia.org/wiki/Python_%28programming_language%29). It's worth briefly considering why we use Java on 5881.

Of the 4 languages LabView is off the table for our team because it's not a _written_ language and rather a _visual_ language. For some teams, especially without a dedicated programming mentor, LabView might be the right choice. In our team where we have programming mentors and students who are interested in perusing CS careers it's important to use written languages. Here's a picture of what LabView looks like:

![LabView Example](/static/img/labview.png)

In addition to LabView, Python is also off the table for our team. Python is generally a great language for beginners, but for FRC I prefer object-oriented languages where we have more freedom to create abstractions. Strictly speaking Python supports "object-oriented programming" but it comes with a caveat that it's a lot more nuanced than what Java and C++ expects of us. Furthermore, it's the newest language to be supported by FRC and there isn't currently enough library support for it.

Between C++ and Java, Java is more popular in the FRC community while C++ is actually language that most of the FRC ecosystem is written in. Java exists mostly as a wrapper around C++ code. There's a good reason for this dichotomy, Java is widely agreed to be easier to learn than C++. Most teams write their robot code in Java. C++ is faster and gives you more control over what's happening on the hardware, where you can write extremely efficient code to talk to a motor, or communicate over a USB port. Overall, there are less ways to screw up a program in Java than in C++.

Java is also the language that the College Board has chosen to test students on for AP Computer Science. Finally, maybe most importantly, I just don't like C++ all that much!
