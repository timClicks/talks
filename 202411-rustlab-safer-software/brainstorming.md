# Brainstorming


What is wrong with this code?

```rust
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```
It can panic.


## The problem

### Causes

If software analysis tools are so good, then why does nobody use them?

They're difficult.

They create friction.

It takes time to learn a new tool.

And it's unclear that the time taken to learn it will be worth it.

Then, it's unclear that you will get the payoff.

Every tool says that it's going to be helpful.

But many tools require a lot of training before you can be skilled in them.

And training yourself is extremely frustrating.

There's a gap between what you want to produce and what you're able to produce.
Everyone who has learned an instrument or draw well has felt this. Worse still. As you learn more, your goals move furhter out of reach. Labelled as the "Taste Gap" by Ira Glass, it's said to mostly be a positive thing. Experiencing the taste gap means that you're able to appreciate excellence even if you're not able to produce it.

While that's reassuring, it's also discouraging. We have deadlines. New bug reports are coming in. Everything is on fire.

> Diagram idea: gap between what you know and what you want to achieve.

## Software is difficult to measure

It's hard to measure the speed to create software and the quality of what is produced.

That means that can't use a purely quantitative approach.

This affects us twice. We can't make accurate predictions about the likely improvements. We don't know how fast we are going.

We can't calculate how fast we will go. So we can't calculate the change. Nor can we calculate the opportunity cost.

## Prevention is extremely difficult to measure

It's very difficult to quantify the number of lives saved through a change.

Failure is complex and attributing errors to a single failure (root cause analysis) masks that complexity.

Among other

## We're very bad at estimation

Programmers are not good at planning. 
We suffer from another fallacy: optimism bias.

## Ways forward

How we address this?

We need to make sure that we're receptive to changing our minds.

### Make it impossible to skip: borrow checker, continuous integration

It used to be difficult to remember to run tests and wait for them.

Now we spend all of our time fighting our CI systems.

The downside here is that there will be significant push back from new developers
and others who don't understand the benefits.
This looks like pure cost.

### Lesson from medicine: checklists

Doctors, especially surgeons, used to kill many of their patients.

Experts are very reluctant to use checklists, even though they improve outcomes.

### Lesson from behavioural economics: anchoring

Let's use cognitive science to help us trick ourselves into doing the upfront work.

Why do marketers use prices like EUR 0,99?

Because even though we believe that this means a single Euro, our brain can't avoid being "anchored" by the first digit that it sees.

Even scientists that know about this effect cannot avoid it.

Can we do the same thing with software tooling?

Some ideas.

Reverse where we're anchoring costs.
Instead, starting with immediate cost of testing
(e.g., "This will take 2 extra days"),
anchor on the long-term cost of not testing.
"Each critical bug in production costs us 5 days of emergency fixes and damages customer trust worth $X,
therefore we will spend 2 days to prevent that from happening."

Reverse the timeframe that you're thinking in.

Anchoring the time saved.
Consider the difference between these two sentences:
"This will take an hour to fix, saving 6 hours of development time over the next 4 weeks.",
versus
"We will lose 6 hours of development time over the next 4 weeks, unless we take an hour to fix this."

Anchor on the cost.
Outages cost ~100k per hour.
Therefore, investing ~1k each week to reduce this risk seems prudent.
It's easier to absorb that cost over the 2-year period,
rather than all in one go.

This strategy also leverages another cognitive bias: loss aversion.
Humans feel loss more heavily than they acquire a benefit of equal value.
Consider how angry you feel and how long that lasts when you lose a 10 Euro note,
compared to the slight fleeting joy that feel when you find one in your pocket.

## Tools as coaches, rather as guards

It's easy to think of checks like the borrow checker as things that "get in the way".
Another way to think about them is to say that they are liberating us.
We don't need to worry.
When Rust was released, one of the expressions that was used heavily was "fearless concurrency".
That's where the term fearless concurrency came from.

Fearless programming does not require ignorance or recklessness.

The borrow checker and other tools are enabling us to develop efficient,
memory safe software that we would otherwise not have been able to.
Before Rust, concurrency was too hard.

But what's the downside of this type of approach? Well, to start with it's annoying.

## We make things harder than they need to be

Consider the term "formal methods". This is intentionally harder than it sounds.

Much of the language that we use in computer science is inherited from mathematics.
That is a fairly archaic discipline with centuries of history.
While the first generation of computer programmers were different,
today it's very rare to be well versed in advanced mathematics before starting to code.
Few computer programmers have grown up proficient in that language.

Not just monad.
Function, argument, type, set, vector, scalar are all terms that come from mathematics.

## What will role of AI be?

We're not sure, but we're all going to find out.

Consider this abstract that relates to the TRACTOR programme from DARPA,
the defense research funding agency of the USA.

> The Defense Advanced Research Projects Agency is soliciting innovative proposals for
> novel techniques to eliminate software memory safety vulnerabilities by large-scale code
> conversion from C to safe, idiomatic Rust. The goal is to achieve the same quality and
> style that a skilled Rust developer would produce, thereby eliminating the entire class
> of memory safety security vulnerabilities present in C programs. This program may involve
> novel combinations of software analysis, such as static analysis and dynamic analysis, and
> machine learning techniques like large language models.
>
> "Translating All C TO Rust (TRACTOR)" Program Solicitation <https://sam.gov/api/prod/opps/v3/opportunities/resources/files/ce67ef1b609a4b20833280c0b9b9f8d5/download>

## Make it very difficult to do the hard thing

In Rust, files, network sockets and locks don't have a `close()` method.
Instead, they rely on the `Drop` trait.
The Drop trait runs the object is out of scope.
This makes it impossible to forget.

Can something like this be be applied to the process of software more generally?

The borrow checker is actually similiar.
It's not an essential part of the compilation process.
Instead, it's sort of a very powerful linter that sits to the side.



---

## Starter tools

linting


---

## Rust is not a panacea

### Panicing

There are man

```rust
fn check_ints
```


## Rust could go furhter


### Effects system 

We could extend Rust to be able to notify the type system 
that functions will not do unwanted things, such as panicing 
or accessing I/O.


---

## Topic ideas

### The Rust Advantage

- Explain how Rust's strong type system contributes to safer software
- Showcase specific Rust features that prevent common errors (e.g., ownership system, Option, Result types, exhaustive pattern matching)

Quirk: Result and Option are not actually part of the language itself. 
They're defined in the standard library.
Their nice properties are provided by exhaustive pattern matching and enums.


### The Borrow Checker as a Coach

- Reframe the borrow checker as a helpful tool rather than an obstacle
- Provide examples of how it catches potential bugs early in development

### Automated Tools for Error Prevention

- Explore various static analysis tools available for Rust
- Demonstrate how to integrate these tools into CI/CD pipelines

## Reducing complexity vs moving complexity

Diagram the difference between shifting complexity from one part of the system to another, versus reducing complexity in the system itself.
"Can we reduce the complexity of our system without changing its behaviour?"

### Semantic Versioning and `cargo-semver-checks`

- Explain the importance of semantic versioning in maintaining software
- Show how to use cargo-semver-checks to ensure version compatibility

### Comprehensive Testing Strategies

- Dive into different testing methodologies (unit, integration, fuzz, property-based)
- Provide examples of each testing type in Rust

### AI in Software Development

- Discuss how AI can be leveraged to improve code quality and security
- Explore potential risks and mitigation strategies when using AI in development

### Supply Chain Security

- Highlight the importance of securing the software supply chain
- Discuss tools and practices for auditing dependencies in Rust projects

### Formal Verification and Model Checking

- Introduce these concepts and their relevance to Rust development
- Provide simple examples of how they can be applied in real-world projects

### Energy Efficiency in Software

- Discuss how robust, efficient code contributes to energy savings
- Showcase Rust's potential for writing energy-efficient software

### Building a Culture of Excellence

- Provide strategies for fostering a culture that prioritizes software quality and security
- Discuss how to implement these practices in development teams

### Case Studies

- Present real-world examples of how Rust has improved software safety and reliability
- Analyze lessons learned from these cases

### Future of Safe Software

- Speculate on upcoming trends in software safety
- Discuss how Rust is positioned to address future challenges

## Topic: The State of Software Today: Challenges and Constraints

### Current Challenges

1. Increasing Complexity
   - Software systems are becoming increasingly complex, making them harder to understand, maintain, and secure.
   - Interconnected systems create a vast attack surface for potential vulnerabilities.

2. Legacy Systems
   - Many critical systems still run on outdated, poorly maintained code.
   - Rewriting or upgrading these systems is often seen as too risky or expensive.

3. Time-to-Market Pressure
   - The drive to release features quickly often leads to cutting corners in security and reliability.
   - "Move fast and break things" mentality can result in unstable and insecure software.

4. Lack of Security Awareness
   - Many developers lack formal training in security best practices.
   - Security is often treated as an afterthought rather than a fundamental aspect of development.

5. Resource Constraints
   - Limited budgets for security audits, testing, and infrastructure upgrades.
   - Shortage of skilled professionals in cybersecurity and advanced software engineering.

### Constraints to Improvement

1. Inertia and Resistance to Change
   - Established practices and technologies are difficult to replace, even when better alternatives exist.
   - "If it ain't broke, don't fix it" mentality can prevent necessary upgrades.

2. Backward Compatibility Requirements
   - Need to support older systems and APIs can limit the adoption of newer, safer technologies.
   - Maintaining compatibility often means carrying forward old vulnerabilities.

3. Economic Factors
   - Short-term financial goals often overshadow long-term investments in software quality.
   - Difficulty in quantifying the ROI of preventive measures can make them hard to justify.

4. Skill Gap
   - Adopting new, safer technologies like Rust requires retraining and can initially slow down development.
   - Shortage of developers skilled in advanced safety-focused languages and techniques.

5. Tooling and Ecosystem Maturity
   - Newer, safer technologies may lack the rich ecosystem of tools and libraries found in more established languages.
   - Integration with existing systems and workflows can be challenging.

6. Cultural and Organizational Issues
   - Siloed departments can lead to a lack of holistic approach to software safety.
   - Blame culture can discourage open discussions about failures and vulnerabilities.

7. Regulatory Lag
   - Legislation and standards often lag behind technological advancements.
   - Compliance-focused approach may not always align with best practices for safety and security.

By understanding these challenges and constraints, we can begin to address them systematically. Rust, with its focus on safety and performance, offers solutions to many of these issues. However, creating a safer future for software requires not just technical solutions, but also shifts in culture, education, and organizational priorities.

---

## Problems

### underscore let Drop

Given this definition of `defer()`, 

```rust
struct Deferred<T: FnOnce()> {
    task: Option<T>,
}

impl<T: FnOnce()> Deferred<T> {
    fn abort(&mut self) {
        self.task.take();
    }
}

impl<T: FnOnce()> Drop for Deferred<T> {
    fn drop(&mut self) {
        if let Some(task) = self.task.take() {
            task();
        }
    }
}

fn defer<T: FnOnce()>(f: T) -> Deferred<T> {
    Deferred { task: Some(f) }
}
```

...hat is the difference between these two code blocks?

```rust
async fn text_from_url(
    url: &str,
    metrics: &mut Metrics
) -> Result<String> {
    let mut at_finish = defer(|| metrics.completed_requests += 1);

    let request = reqwest::get(url);
    tokio::select! {
        response = request => {
            let response = response?;
            let body = response.text().await?;
            Ok(body)
        }
        _ = tokio::time::sleep(Duration::from_millis(2500)) => {
            at_finish.abort();
            Err("timeout".into())
        }
    }
}
```

```rust
async fn text_from_url(
    url: &str,
    metrics: &mut Metrics
) -> Result<String> {
    let mut _ = defer(|| metrics.completed_requests += 1);

    let request = reqwest::get(url);
    tokio::select! {
        response = request => {
            let response = response?;
            let body = response.text().await?;
            Ok(body)
        }
        _ = tokio::time::sleep(Duration::from_millis(2500)) => {
            at_finish.abort();
            Err("timeout".into())
        }
    }
}
```

The second

---

## Solutions

### Fuzzing

Cargo semver checks


### Semantic versioning checking

Cargo semver checks



### Linting

The term "linting" is a metaphor from removing lint from clothing. It refers to process of scanning the code base, detecting and removing potential problems.

Tip: consider promoting selective [lints that are allowed by default] to warnings/errors. This will help to keep code quality very high.

```rust
#![warn(missing_copy_implementations)]
#![warn(missing_docs)]
```

Tip: avoid promoting warnings to denials with `#![deny(warnings)]`. This is likely to break your build in the future, because lints are changed each Rust release, e.g. every 6 weeks.

Rust has a number of built-in lints that help you find common mistakes in your code. There is also a wider project, `clippy`, which offers more checks for common patterns.

These come in 6 levels.

| Level         | Emit message? | Allows compilation? | Notes                          |
|:-------------:|:-------------:|:-------------------:|:------------------------------:|
| allow         | No            | Yes                 |                                |
| expect        | Yes           | Yes                 | Alerts when lint not triggered |
| warn          | Yes           | Yes                 |                                |
| force-warn    | Yes           | Yes                 | Cannot suppress                |
| deny          | Yes           | No                  |                                |
| forbid        | Yes           | No                  | Cannot suppress                |


These are called "warnings", which means they don't stop the compilation but may
be fixed by changing some code.

Tip: add clippy

[lints that are allowed by default]: https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html


---

## Other notes/thoughts

### Rust is gong to be part of the software industry for a long time

Rust has crossed the chasm and is now a "forever language".
It is now in so many large projects run by large companies that it will be continued for all of our lives.
It is within Windows and Linux.
In some sense, it's too big to fail.
Being supported by a foundation means that it's highly resistant to failure by one company withdrawing its support.
