# ring
A CLI utility written in Rust to test user's ping to multiple video game servers.

# How to Use
Because there won't be any binaries until I hit a major release, you'll need to build it. But it's a small program, so it's easy.
- You'll need the Rust programming language installed including cargo, it's dependency / package manager.

`git clone https://github.com/murnux/ring`
`cd ring`
`cargo run <arguments>`

Read the next section for options to use with the program.

### Arguments
To list all current servers available for pinging:
`cargo run list`

Then to ping a server:
`cargo run dota2-uswest`
or any other server name in the list. 

You can also ping as many servers as you want at once:
`cargo run dota2-uswest fortnite-uswest hearthstone-americas rocketleague-uswest ...`

This will cycle, or iterate, through all the servers you named and provide ping info for each. 


# Contributing
So far, there are two major ways to contribue to ring: 

### Development
If you have ideas for any features, or want to touch up some already existing code, submit a pull request!

### Adding Game Servers
A great way to contribute for programmers and non-programmers alike is to add to the program's list of game servers. 

You will need a text editor. Using the already in-place syntax, add the name of the server like 'rocketleague-uswest' to games.json. 

Then proceed to submit a pull request. 
