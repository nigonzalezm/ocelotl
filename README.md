# Ocelot

The default client will connect to the server and will exit once the game time reaches to 6000 or if it receives any message different to "continue" from the trainer.

## Server

Use

    $ rcssserver

to run the RoboCup2D server with default configuration. In this mode it is recommended to use rcssmonitor to start the play with a kick_off command. It is also possible to start the server in trainer mode with:

    $ rcssserver server::coach=on

In trainer mode you can connect as a trainer and change the play modes programatically.

## Check rust code

    $ cargo check

## Run tests

    $ cargo test <test_name> -- --nocapture