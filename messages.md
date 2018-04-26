# Messages


## EventMessage

EventMessage represents the normalized data from various chat sources. This
message may also contain plugin information if the message sent is a plugin
trigger.

| Field           |Type             | Description|
| ---------       |-----            |---------   |
| platform        | **String**      | The platform the chat message came from (i.e Twitch, YouTube, HitBox) |
| channel         | **String**      | The channel the server came from (i.e dwangoAC) |
| timestamp       | **String**      | The timestamp of when the chat message was sent sent |
| sender          | **String**      | The one who sent the chat message |
| message         | **String**      | The chat message |
| split_msg       | **String Array**| The chat message split into individual words|
| platform_meta   | **Object**      | An object containing platfrom metadata (i.e isMod, isSteamer, badges, etc...)
| plugin          | **Object**      | Plugin object

Plugin Data

| Field           |Type             | Description|
| ---------       |-----            |---------   |
| plugin_id       | **Integer**     | The plugin id that this message corresponds to
| plugin_type     | **Integer**     | The plugin type signifies wether the messages need to be processed by state processor or if it can be passed directly to botserv. **1** means the messages needs to be processed by state processor. **-1** means the message be passed to botserv without processing. A message should never return **0**
| trigger         | **String**      | The string that triggered the plugin message event
| data            | **Object**      | A data object that is sent with the relevant information generated gathered by command

## Example with a plugin
```json
{
	"platform": "Twitch",
	"channel": "dwangoAC",
	"timestamp": "1522899575",
	"sender" : "jeff",
	"message":"Lets sit around the campfire and sing the campfire song the C A M P F I R E song",
	"split_msg":["Lets", "sit", "around", "the", "campfire", "and", "sing", "the", "campfire", 
			"song", "the", "C", "A", "M", "P", "F", "I", "R", "E", "song"],
	"platform_meta": {
        	...
	},
	
	"plugin": {
		"plugin_id" :123,
		"plugin_type": 0,
		"trigger": "camping",
		"data": {
			...
		}
	}
	
}
```

## Example without a plugin
```json
{
	"platform": "Twitch",
	"channel": "dwangoAC",
	"timestamp": "1522899575",
	"sender" : "jeff",
	"message":"Lets sit around the campfire and sing the campfire song the C A M P F I R E song",
	"split_msg":["Lets", "sit", "around", "the", "campfire", "and", "sing", "the", "campfire", 
			"song", "the", "C", "A", "M", "P", "F", "I", "R", "E", "song"],
	"platform_meta": {
        	...
	},
	
	"plugin": null
	
}
```