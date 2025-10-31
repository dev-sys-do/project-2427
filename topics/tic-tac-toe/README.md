# A Simple Tic-Tac-Toe Agent

## Description and Goal

The goal is to build a command-line [Tic-Tac-Toe](https://en.wikipedia.org/wiki/Tic-tac-toe) game where a human player can play against an AI opponent.

The AI should be "smart" enough to play optimally, meaning it can't be beaten and will either win or draw every game.

## Game State Representation

A simple 2D array or a 1D array of a fixed size is a good choice.

## Agent Algorithm

The recommended choice for the Agent algorithm is the `Minimax` algorithm with a depth-first search: The AI assumes its opponent will always make the best possible move, and it will choose its own move to minimize the maximum possible loss (or, conversely, maximize the minimum possible gain).

A simpler, alternative option is the tree search one: The algorithm builds a game tree to explore all possible future moves. The agent assigns scores to the board's end states (+1 for an AI win, -1 for a human win, 0 for a draw) and "propagates" those scores up the tree to find the best move.

## References

[Minimax and tic-tac-toe](https://www.neverstopbuilding.com/blog/minimax)

## Grade Factor

The grade factor for this project is *1.2*.
