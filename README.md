# What is this?

I've been playing Magic the Gathering with my friends a bunch lately and they were complaining that Magic the Gathering 
Arena "holds your hand" too much. I've been wanting to try some game development lately so
I figured "why not?"

I am now beginning to understand the answer to my own question (seriously, look how many [rules there are](https://media.wizards.com/2024/downloads/MagicCompRules%2020240607.pdf))

But I'm in too deep and having too much fun with it so, onwards we march.


After a few days of writing this in TypeScript, I realized this would be a great way to learn [Rust](https://www.rust-lang.org/).
So I started porting over what I'd already done in TypeScript to Rust.
And it's been going great so far- but there's a lot still left to do.


# Installation

If you wanna check it out, you can install the Typescript version by cloning the repo
and running just running ```npm install```. It uses next so to run it you can just run ```npm run dev``` to check it out.

If you wanna check out the Rust version which is WAY faster (the json file this uses for data is 12mb; it was originally
56mb but thanks to my formatAtomicData script I got rid of a bunch of data I wouldn't need), go install rust and then run
```cargo install``` from within the ```/rust``` directory. after that you can run ```cargo run``` to check it out
(it only exists in the terminal in rust as of right now.) If you want to poke around, I suggest using nodemon to watch 
your files and recompile the binary. ```nodemon --watch srce - e rs --exec cargo run```
You can run the tests with ```cargo test```

