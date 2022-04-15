# CS4100 Introduction to Formal Languages and Compilers 

## Spring 2022

An upper-level course for CS majors on formal language theories and compiler designs. 

Topics (subject to revision): regular expressions; finite automata; context-free grammars; predictive parsing; LR parsing; abstract syntax; type systems and type-checking; stack layout and activation records; intermediate representations; control-flow graphs; dataflow/liveness analysis; register allocation; garbage collection/runtimes; virtual machines; assemblers. Over the course of the semester, students will implement a full functioning compiler for a small programming language, targeting a bespoke virtual machine. The course requires a significant amount of programming.

|                       |         Details      |
|-----------------------|----------------------|
| **Lecture**           | Tu/Th 3:05-4:25pm in Morton Hall 127 |
| **Instructors**        | Chang Liu (liuc@ohio.edu) <br /> Alexander Bagnall (ab667712@ohio.edu) |
| **Office Hours**      | Tu 8-9:40am (email or Teams) |
| **TA/GA**                | "Kanieski, William" <wk142016@ohio.edu> |
| **Lab Hours**         | Usually Fridays at 5pm, the days before PA due dates |

## Texbook

There's no one textbook that covers everything we'll be talking about in this course. Instead, we'll assign readings each week from the following sources:

* "Compilers: Principles, Techniques, and Tools, 2nd edition" by Alfred Aho, Monica Lam, et al. Aug 31, 2006 (The purple dragon book)
* Modern Compiler Implementation in ML, Andrew W. Appel (The tiger book)
* Types and Programming Languages, Benjamin Pierce
* [The Rust Book](https://doc.rust-lang.org/book/index.html)
* [Crafting Interpreters, Nystrom](http://www.craftinginterpreters.com/contents.html)

## Course Difficulty

This is a demanding course that requires extensive programming work, in the form of a series of (often increasingly) difficult assignments. Expect to put in at least 10 hours (sometimes much more) per programming assignment.

## Course Structure

The course consists of weekly lectures (Tu/Th), attendance at which is required. The programming assignments for this course are extensive and time consuming, so be prepared!

In addition to biweekly homework assignments, there will be a midterm exam (Week 9, approximately 15% of your grade) and a final (approximately 25%). The homeworks (programming assignments) are worth approximately 40%. We'll also have simple Blackboard quizzes (10%) and programming exercises (10%). 

### Grade Breakdown

| Component               | Percentage |
|-------------------------|-----|
| Programming exercises   | 10% |
| Project assignments | 40% |
| Quizzes                 | 10% |
| Midterm exam            | 15% |
| Final exam              | 25% |

Blackboard will be used to report grades and to post lecture notes, homeworks, and reading materials. 

## Schedule

The schedule is subject to revision.

| Week                        | Topic                                 | Reading                        | Assignment |
|-----------------------------|---------------------------------------|--------------------------------|------------|
| Week 1 (11 Jan) | Intro. to the course, compilers, Rust | [The Rust Book](https://doc.rust-lang.org/book/index.html) 1-3 | Q0/[PE1](pe1.md) (Sat. 15 Jan) |
| Week 2 (18 Jan) | Rust contd. | [The Rust Book](https://doc.rust-lang.org/book/index.html) 4-6, 8 | Q1/[PE2](pe2.md) (Sat. 22 Jan)  |
| Week 3 (25 Jan) | Rust contd. | [The Rust Book](https://doc.rust-lang.org/book/index.html) 9, 10.1, 10.2 | [PA0: Intro. to Rust](pa/0.md) (Sat. 29 Jan) |
| Week 4 (1 Feb) | Virtual machines, bytecode, assemblers | Crafting Interpreters [14](http://www.craftinginterpreters.com/chunks-of-bytecode.html), [15](http://www.craftinginterpreters.com/a-virtual-machine.html) | Q2/[PE3](pe3.md) (Sat. 5 Feb) |
| Week 5 (8 Feb) | Garbage collection, concurrency | Appel 13 | [PA1: Assembler](pa/1.md) (12 Feb - extended to the following Wed) |
| Week 6 (15 Feb) | Regular languages, regular expressions | Appel 2 (through 2.2) or Aho 3.3 | Q3 Garbage Collections/[PE4](pe-rustlings.md) (19 Feb - extended to the following Wed) |
| Week 7 (22 Feb) | DFAs, NFAs, lexers and lexer generators | Appel 2.3-2.5 | [PE5](pe-regularexpressions.md)(26 Feb)/Q4 Regular Expressions (5 Mar)   |
| Week 8 (1 Mar) | Parsing, Context-free languages, pushdown automata | Appel 3 | [PA2: VM](pa/2.md) (extended to 19 Mar) |
| Week 9 (15 Mar) | Midterm review | | Midterm Exam Review (Thursday 17 Mar) |
| Week 10 (22 Mar) | Recursive descent and predictive parsing, parser generators | Appel 3.2-3.3 | [PA3: GC](pa/3.md) (26 Mar) |
| Week 11 (29 Mar) | Abstract syntax trees, type systems, typechecking | TAPL 3, 8 |  No quiz -- work on PA4! |
| Week 12 (5 Apr) | Intermediate representations, code generation | [Intermediate Representations](doc/ir.md), [Code Generation](doc/codegen.md) | Q5 (9 Apr) |
| Week 13 (12 Apr) | IR/codegen continued, control-flow graphs | Appel 7.1 | [PA4: IR](pa/4.md) (16 Apr) |
| Week 14 (19 Apr) | Dataflow/liveness analysis | Appel 10.1, Supplementary: [SSA is Functional Programming](https://www.cs.princeton.edu/~appel/papers/ssafun.pdf) [(acm link)](https://dl.acm.org/doi/pdf/10.1145/278283.278285) | No quiz -- study for finals! |
| Apr 25 - Apr 29 | **FINAL EXAM** (TBD) | | [PA5](pa/5.md) (Optional, 23 Apr)|
<!-- | Week 14 (19 Apr) | Register allocation | Appel 11 | No quiz -- study for finals! | -->

* Note: we will move the Week 11 and Week 12 content forward one week to get ready for PA4 sooner.

Assignments are due in Blackboard at 11:59pm unless otherwise specified. **Q0**, **Q1**, etc., denote quizzes in Blackboard.

## Homework and Collaboration Policies

### Acceptable Collaboration Matrix

|            | Instructor/GA	| Noninstructor (e.g., Another Student) | 
|------------|----------------|---------------------------------------|
| ***You***  | All collaboration allowed | High-level discussion (of the problems, not your code!) allowed but only after you've started the assignment; must be documented in README as described below |

Unless otherwise noted, homeworks are due Saturdays by 11:59 p.m. Late homework assignments will be penalized according to the following formula:

* Up to 24 hours late: no deduction, for a max 2 late homeworks per student across the entire semester
* Homeworks later than 24 hours, or from students who have already turned in 2 late homeworks, will receive 0 points.

You may discuss the homework with other students in the class, but only after you've attempted the problems on your own first. If you do discuss the homework problems with others, write the names of the students you spoke with, along with a brief summary of what you discussed, in a README comment at the top of each submission. Example:

```
(* README Alex Bagnall, Assn #1 
I worked with X and Y. We swapped tips regarding the use of pattern-matching in Rust. *)
```

However, **under no circumstances are you permitted to share or directly copy code or other written homework material**, except with course instructors. The code and proofs you turn in must be your own. Remember: homework is there to give **you** practice in the new ideas and techniques covered by the course; it does you no good if you don't engage!

That said, if we find that you have cheated on an assignment in this course, you will immediately:

* Be referred to the Office of Community Standards (which may take disciplinary action against you, possibly expulsion); and
* Flunk the course (receive a final grade of F).

Students in EECS courses such as this one must adhere to the Russ College of Engineering and Technology [Honor Code](https://www.ohio.edu/engineering/current/leadership-integrity/academic-integrity), and to the OU [Student Code of Conduct](https://www.ohio.edu/student-affairs/community-standards/student-code-of-conduct). If you haven't read these policies, do so now.

## Students with Disabilities

If you suspect you may need an accommodation based on the impact of a disability, please contact me privately to discuss your specific needs. If you're not yet registered as a student with a disability, contact the Office of Student Accessibility Services first.

## Student Outcomes vs. Course Learning Outcomes

1. Analyze a complex computing problem and to apply principles of computing and other relevant disciplines to identify solutions.

* Students will be able to appraise the tradeoffs, in terms of asymptotic complexity and precision, of distinct algorithms used in compiler construction (e.g., for garbage collection).

2. Design, implement, and evaluate a computing-based solution to meet a given set of computing requirements in the context of the program’s discipline.

* Students will be able to construct a compiler, over the course of a series of course assignments, for a small programming language.

6. Apply computer science theory and software development fundamentals to produce computing-based solutions.

* Students will be able to determine whether a given language is recognizable (e.g., by a regular expression, deterministic finite automaton, or context-free grammar).
* Students will be able to construct a finite state machine to recognize a given language.
* Students will be able to apply computer science theory to determine whether a given grammar is parseable by recursive descent.


## Classroom COVID-19 policies

Ohio University is committed to maintaining a safe learning environment for all members of its community – students, faculty, and staff – regardless of whether they have been vaccinated against COVID-19.

All members of the OHIO community are expected to take certain precautions in classrooms and elsewhere on campus to keep each other safe. All individuals are required to wear masks in indoor public spaces on campus, including classrooms. 

All OHIO students, faculty and staff must be vaccinated against COVID-19 or have an approved exemption. The University strongly encourages all students, faculty, and staff to get a booster shot.  

If you are on the Weekly Testing Pathway as part of the COVID-19 Testing Pathway Program, you are expected to adhere to your required testing schedule.
Complete a daily COVID-19 symptom assessment(opens in a new window) and do not come to class or participate in an activity if you feel sick, even if you are vaccinated against COVID-19. 

If you test positive, feel sick or suspect exposure to COVID-19, it is your responsibility to follow the OHIO COVID-19 Protocol.

If you do test positive or need to isolate or quarantine this semester, after you have taken care of yourself and followed all the steps in the OHIO COVID-19 Protocol, please email me so that we can develop a plan for you to receive necessary information. COVID-related illness, quarantine, isolation, and remain-in-room directives are legitimate university absences, and I will work with you to manage your academic or work requirements and connect you to resources. 

If you feel that your class or work performance is being impacted by COVID-19, please talk with me and/or contact COVID Operations by phone (740.566.8445) or email (COVIDoperations@ohio.edu) Monday through Friday, 8 a.m. to 8 p.m., and noon to 5 p.m. on weekends. The University has resources available to help with quarantine and isolation support, as well as access to COVID-19 testing, counseling services, food assistance, and more. 
