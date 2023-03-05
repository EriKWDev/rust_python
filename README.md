
# About
Just experimenting with calling python from Rust to see if it would be viable to use as a scripting language.
I might not be doing this optimally, but it turns out that iterating 3300 times and appending to a list (not doing anything else)
takes around `2.3 ms` on my machine from python.

Given that we are doing nothing else, this is horrible performance. Doing the same from rust takes tens of nanoseconds.
