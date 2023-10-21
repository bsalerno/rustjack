# rustjack

_rustjack? yeah buddy_

## What is rustjack?

`rustjack` is a cli blackjack game, written in rust. It's pretty rudimentary at the moment, but that's because I am using this project to learn rust. Please don't give me too much credit for the name, I know I am truly revolutionizing the naming game. I had the idea to use this as a rust training exercise and didn't think of a better name in time.

I decided to make a blackjack game since a) it's a simple exercise in object oriented programming (at least in my opinion) and b) playing better blackjack is something I am interested in learning. Note that the structure of the game is intended to be just as it is in a cardroom: the shoe contains multiple decks, blackjack pays 3:2, dealer only shows one card, etc. Ideally, this will also serve as a tool by which I can learn a simple card counting system.

## Things I want to add in the future:

- ~sessions: just keep playing hands until you say to stop~
  - maybe prompt user every X hands to see if they want to keep playing?
  - ~stats on session: +/- for hands/units~
  - reshuffle the shoe at a certain point of cards remaining
- actual game mechanics
  - splitting
- stats: writing hit/stand decisions to a file for future analysis
- keeping count: prompt the player at certain intervals to check the count
