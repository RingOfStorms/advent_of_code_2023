Starting to loose steam on doing these problems, will write down what I would do for part 1, may come back to this one day.

- Create a map of all named modules
- enums for FlipFlip, Conjunction, Broadcast modules
- cache broadcast module's name for easy button pressing
- write a button press simulator step
    - set current frame = low pulse on all broadcast targets
        - exit loop if current frame has no new pulses
        - low pulse to all Broadcast targets is the initial frame
        - update internal state of every module based on current frame's pulses
        - as state is updated add any new pulses to a the upcoming/next frame
        - once frame is drained of pulses, set to the next frame and repeat
- We will want to cache/save all module states at every frame and the current counts of low/high pulses. If the state repeats then we know we can exit early and figure out the math for the rest: just like in day 14
- Calculate the total low/high for 1000 button presses based on above info
- get answer by multiplying those two
