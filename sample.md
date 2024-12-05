~~~
echo '{
    "bot_id": "Um92ZXJXYXJzLmNvbQo=",
    "action": "move",
    "say": "hello?"
}' | target/release/roverwars-spec
Got Message { bot_id: "Um92ZXJXYXJzLmNvbQo=", action: Move, say: "hello?" }
{"bot_id":"Um92ZXJXYXJzLmNvbQo=","action":"move","say":"hello?"}
{
  "bot_id": "Um92ZXJXYXJzLmNvbQo=",
  "actions": [
    "move"
  ],
  "map": {
    "id": "0",
    "tiles": [
      ["wall","wall","wall","wall","wall","wall","wall","wall","wall","wall"],
      ["wall","sand","sand","sand","sand","sand","sand","sand","sand","wall"],
      ["wall","sand","sand","sand","sand","sand","sand","sand","sand","wall"],
      ["wall","sand","sand","sand","sand","sand","sand","sand","sand","wall"],
      ["wall","sand","sand","sand","sand","sand","sand","sand","sand","wall"],
      ["wall","sand","sand","sand","sand","sand","sand","sand","sand","wall"],
      ["wall","sand","sand","sand","sand","sand","sand","sand","sand","wall"],
      ["wall","sand","sand","sand","sand","sand","sand","sand","sand","wall"],
      ["wall","sand","sand","sand","sand","sand","sand","sand","sand","wall"],
      ["wall","wall","wall","wall","wall","wall","wall","wall","wall","wall"]
    ]
  }
}

~~~