# Messages


## EventMessage

EventMessage represents the normalized data from various chat sources. This
message may also contain plugin information if the message sent is a plugin
trigger.

| Field           |Type             | Description|
| ---------       |-----            |---------   |
| platform        | **String**      | The platform the chat message came from (i.e Twitch, YouTube, HitBox) |
| channel         | **User Object** | The channel the server came from (i.e dwangoAC) |
| timestamp       | **String**      | The timestamp of when the chat message was sent sent |
| sender          | **User Object** | The one who sent the chat message |
| message         | **String**      | The chat message |
| split_msg       | **String Array**| The chat message split into individual words|
| platform_meta   | **Object**      | An object containing platform data(if Twitch it might look like " {"brodcaster-lang":null, "emote-only":false, "followers-only":false, "sub-only":false, "r9k":false}")
| plugin          | **Object**      | Plugin object

#### User

| Field           |Type             | Description|
| ---------       |-----            |---------   |
| name            | **String**      | The name of a user |
| user_id         | **String**      | The uid of the user based on platform |
| user_meta       | **Object**      | The meta of an user. (if its a Twitch user it might look like "{"isMod":false, "isSubscriber":false, "isTurbo":false, "badges":[], "color":"#0D4200", "bits":100, "user-type":null}|

#### Plugin

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
	"channel": {
		"name":"dwangoAC",
		"id":"lajsd;ljeffl;kajsd;ihih",
		"user_meta": {
			...
		}
	},
	"timestamp": "1522899575",
	"sender" : {
		"name":"jeff",
		"id":"jeff_id",
		"user_meta": {
			...
		}
	},
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
	"channel": {
		"name":"dwangoAC",
		"id":"lajsd;ljeffl;kajsd;ihih",
		"user_meta": {
			...
		}
	},
	"timestamp": "1522899575",
	"sender" : {
		"name":"jeff",
		"id":"jeff_id",
		"user_meta": {
			...
		}
	},
	"message":"Lets sit around the campfire and sing the campfire song the C A M P F I R E song",
	"split_msg":["Lets", "sit", "around", "the", "campfire", "and", "sing", "the", "campfire", 
			"song", "the", "C", "A", "M", "P", "F", "I", "R", "E", "song"],
	"platform_meta": {
        	...
	},
	
	"plugin": null
	
}
```