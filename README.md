# RoverWars

The mission was compromised by an anomaly that destroyed our Mars Rover and
created clones all over the surface of Mars placed in mazes.

We have radio communication with the rovers and early investigation found
artifacts in the mazes containing plans for advanced technology which the
rovers are able to pick up and transmit back to our satellites.

There is a countdown timer constantly being transmitted with about a day 
remaining.

Help us solve the mazes created by the anomoly and collect as much technology 
as possible before time runs out.


## Capabilities

The cloned rovers perform actions in response to messages from our satellites.
We have discovered the following capabilities so far:

- Move forward
- Turn left
- Turn right
- Report everything in the rover's line of sight
- Pick up an object in front of the rover
- Drop a help up object
- Wait


## Example Message to Rover

~~~json
{
    "bot_id": "Um92ZXJXYXJzLmNvbQo=",
    "action": "move",
    "say": "hello?"
}
~~~


## Example Response from Rover

~~~json
{
    "bot_id": "Um92ZXJXYXJzLmNvbQo=",
    "action_attempted": "move",
    "action_succeeded": true,
    "held": "nothing",
    "saw": [
        ["wall","wall","wall","wall","wall"],
        ["wall","sand","sand","sand","wall"],
        ["wall","sand","self","sand","wall"],
        ["wall","sand","sand","sand","wall"],
        ["wall","wall","wall","wall","wall"]
    ],
    "said": "hello?",
    "heard": "<wind>"
}
~~~


## How to Participate

Send commands to the satellites to be relayed to a rover or send a program to
run on the satellite and generate the commands in Mars orbit.

Each command or program takes a few minutes to send to our satellites in Mars 
orbit and each command takes a couple seconds to send from Mars orbit to the 
rovers on the surface and receive a response.


### Sending a Command to a Satellite

~~~json
{
    "api_key": "<my_key>",
    "bot_id": "<bot_id>",
    "action": "<move|left|right|pick|drop|wait>",
    "say": "<string>"
}
~~~


### Sending a Program to a Satellite

~~~json
{
    "api_key": "<my_key>",
    "bot_id": "<bot_id>",
    "program": "<base64_encoded_program>"
}
~~~


